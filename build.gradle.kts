plugins {
    kotlin("jvm") version "2.4.10"
    id("org.jlleitschuh.gradle.ktlint") version "12.1.1"
    `maven-publish`
}

group = "dev.lunasa"
version = (findProperty("skialin.version") as String?) ?: "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

val hostOs =
    run {
        val os = System.getProperty("os.name").lowercase()
        when {
            os.contains("win") -> "windows"
            os.contains("mac") -> "macos"
            os.contains("linux") -> "linux"
            else -> error("skialin: unsupported OS $os")
        }
    }

val hostArch =
    run {
        when (val arch = System.getProperty("os.arch").lowercase()) {
            "amd64", "x86_64" -> "x64"
            "aarch64", "arm64" -> "arm64"
            else -> arch
        }
    }

val targetOs = (findProperty("skialin.targetOs") as String?) ?: hostOs
val targetArch = (findProperty("skialin.targetArch") as String?) ?: hostArch
val nativePlatformDir = "$targetOs-$targetArch"

val lwjglVersion = "3.3.4"
val lwjglNatives = "natives-$hostOs"

dependencies {
    testImplementation(kotlin("test"))

    testImplementation(platform("org.lwjgl:lwjgl-bom:$lwjglVersion"))
    testImplementation("org.lwjgl:lwjgl")
    testImplementation("org.lwjgl:lwjgl-glfw")
    testImplementation("org.lwjgl:lwjgl-opengl")
    testImplementation("org.lwjgl:lwjgl-vulkan")
    testRuntimeOnly("org.lwjgl:lwjgl::$lwjglNatives")
    testRuntimeOnly("org.lwjgl:lwjgl-glfw::$lwjglNatives")
    testRuntimeOnly("org.lwjgl:lwjgl-opengl::$lwjglNatives")
}

kotlin {
    jvmToolchain(25)
}

ktlint {
    version.set("1.5.0")
}

tasks.test {
    useJUnitPlatform()
    systemProperty("org.lwjgl.system.stackSize", "4096")
}

val buildNative = (findProperty("skialin.buildNative") as String?).toBoolean()
val rustDir = layout.projectDirectory.dir("rust")
val cargoProfile = "release"

val nativeLibName =
    when (targetOs) {
        "windows" -> "skialin_jni.dll"
        "macos" -> "libskialin_jni.dylib"
        else -> "libskialin_jni.so"
    }

val skiaDir = layout.projectDirectory.dir("external/skia")

val setupSkia =
    tasks.register<Exec>("setupSkia") {
        workingDir = skiaDir.asFile
        doFirst {
            val outDir = skiaDir.dir("out/Release").asFile
            outDir.mkdirs()
            val args = skiaDir.file("../../native-shim/args.gn").asFile.readText()
            val platformCflags = if (hostOs == "windows") "\nextra_cflags = [\"/MD\"]\n" else ""
            outDir.resolve("args.gn").writeText(args + platformCflags)
        }
        val gnName = if (hostOs == "windows") "bin/gn.exe" else "bin/gn"
        val gn = skiaDir.file(gnName).asFile.absolutePath
        commandLine(gn, "gen", "out/Release")
    }

val buildSkia =
    tasks.register<Exec>("buildSkia") {
        dependsOn(setupSkia)
        workingDir = skiaDir.asFile
        val ninja = if (hostOs == "windows") "ninja.exe" else "ninja"
        commandLine(
            ninja,
            "-C",
            "out/Release",
            "skia",
            "skparagraph",
            "skshaper",
            "skunicode_core",
            "skunicode_icu",
            "skcms",
            "libpng",
            "zlib",
            "expat",
            "harfbuzz",
            "icu",
            "pathops",
            "svg",
            "skresources",
            "skottie",
            "sksg",
            "jsonreader",
        )
    }

val skiaLibDir =
    providers
        .gradleProperty("skialin.skiaLibDir")
        .orElse(providers.environmentVariable("SKIALIN_SKIA_LIB_DIR"))
        .orElse(skiaDir.dir("out/Release").asFile.absolutePath)

val cargoBuild =
    tasks.register<Exec>("cargoBuild") {
        onlyIf { buildNative }
        workingDir = rustDir.asFile
        environment("SKIALIN_SKIA_LIB_DIR", skiaLibDir.get())
        commandLine("cargo", "build", "-p", "skialin-jni", "--release")
    }

fun registerCopyNativeLib(
    name: String,
    destination: Provider<Directory>,
) = tasks.register<Copy>(name) {
    onlyIf { buildNative }
    dependsOn(cargoBuild)
    from(rustDir.dir("target/$cargoProfile")) {
        include(nativeLibName)
    }
    into(destination.map { it.dir("natives/$nativePlatformDir") })
}

fun registerCopyIcuData(
    name: String,
    destination: Provider<Directory>,
) = tasks.register<Copy>(name) {
    onlyIf { buildNative }
    dependsOn(cargoBuild)
    from(skiaLibDir) {
        include("icudtl.dat")
    }
    into(destination.map { it.dir("natives") })
}

val copyNativeLib = registerCopyNativeLib("copyNativeLib", layout.buildDirectory.dir("skialin-natives"))
val copyIcuData = registerCopyIcuData("copyIcuData", layout.buildDirectory.dir("resources/main"))
val copyNativeLibForTest = registerCopyNativeLib("copyNativeLibForTest", layout.buildDirectory.dir("resources/test"))
val copyIcuDataForTest = registerCopyIcuData("copyIcuDataForTest", layout.buildDirectory.dir("resources/test"))

tasks.named("processResources") {
    dependsOn(copyIcuData)
}

tasks.named("processTestResources") {
    dependsOn(copyNativeLibForTest, copyIcuDataForTest)
}

val nativesJar =
    tasks.register<Jar>("nativesJar") {
        onlyIf { buildNative }
        dependsOn(copyNativeLib)
        archiveClassifier.set("natives-$nativePlatformDir")
        from(layout.buildDirectory.dir("skialin-natives"))
    }

java {
    withSourcesJar()
}

publishing {
    publications {
        create<MavenPublication>("main") {
            from(components["java"])
            artifact(nativesJar)
        }
    }
    repositories {
        maven {
            name = "local"
            url = uri(layout.buildDirectory.dir("publishing-repo"))
        }
        val releaseUrl = System.getenv("SKIALIN_MAVEN_REPO_URL")
        if (releaseUrl != null) {
            maven {
                name = "release"
                url = uri(releaseUrl)
                credentials {
                    username = System.getenv("SKIALIN_MAVEN_REPO_USERNAME")
                    password = System.getenv("SKIALIN_MAVEN_REPO_PASSWORD")
                }
            }
        }
    }
}
