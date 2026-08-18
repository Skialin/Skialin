plugins {
    kotlin("jvm") version "2.4.10"
}

group = "dev.lunasa"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

val lwjglVersion = "3.3.4"
val lwjglNatives = "natives-windows"

dependencies {
    testImplementation(kotlin("test"))

    testImplementation(platform("org.lwjgl:lwjgl-bom:$lwjglVersion"))
    testImplementation("org.lwjgl", "lwjgl")
    testImplementation("org.lwjgl", "lwjgl-glfw")
    testImplementation("org.lwjgl", "lwjgl-opengl")
    testRuntimeOnly("org.lwjgl", "lwjgl", classifier = lwjglNatives)
    testRuntimeOnly("org.lwjgl", "lwjgl-glfw", classifier = lwjglNatives)
    testRuntimeOnly("org.lwjgl", "lwjgl-opengl", classifier = lwjglNatives)
}

kotlin {
    jvmToolchain(25)
}

tasks.test {
    useJUnitPlatform()
}

val buildNative = (findProperty("skialin.buildNative") as String?).toBoolean()
val rustDir = layout.projectDirectory.dir("rust")
val cargoProfile = "release"

val nativePlatformDir = run {
    val os = System.getProperty("os.name").lowercase()
    val archProp = System.getProperty("os.arch").lowercase()
    val arch = when (archProp) {
        "amd64", "x86_64" -> "x64"
        "aarch64", "arm64" -> "arm64"
        else -> archProp
    }
    val osName = when {
        os.contains("win") -> "windows"
        os.contains("mac") -> "macos"
        os.contains("linux") -> "linux"
        else -> error("skialin: unsupported OS $os")
    }
    "$osName-$arch"
}

val nativeLibName = run {
    val os = System.getProperty("os.name").lowercase()
    when {
        os.contains("win") -> "skialin_jni.dll"
        os.contains("mac") -> "libskialin_jni.dylib"
        else -> "libskialin_jni.so"
    }
}

val cargoBuild by tasks.registering(Exec::class) {
    onlyIf { buildNative }
    workingDir = rustDir.asFile
    commandLine("cargo", "build", "-p", "skialin-jni", "--release")
}

val skiaLibDir = providers.gradleProperty("skialin.skiaLibDir")
    .orElse(providers.environmentVariable("SKIALIN_SKIA_LIB_DIR"))
    .orElse(rustDir.dir("../external/skia/out/Release").asFile.absolutePath)

/**
 * SkLoadICU() (third_party/icu/SkLoadICU.cpp) looks for icudtl.dat next to
 * the module it's compiled into -- here, wherever NativeLoader extracts
 * skialin_jni's .dll/.so to at runtime (a JVM temp dir, not this build
 * directory). Bundling it as a resource alongside the native lib lets
 * NativeLoader extract both into the same temp directory.
 */
fun registerCopyNativeLib(name: String, destination: String) = tasks.register<Copy>(name) {
    onlyIf { buildNative }
    dependsOn(cargoBuild)
    from(rustDir.dir("target/$cargoProfile")) {
        include(nativeLibName)
    }
    from(skiaLibDir) {
        include("icudtl.dat")
    }
    into(layout.buildDirectory.dir("$destination/natives/$nativePlatformDir"))
}

val copyNativeLib = registerCopyNativeLib("copyNativeLib", "resources/main")
val copyNativeLibForTest = registerCopyNativeLib("copyNativeLibForTest", "resources/test")

tasks.named("processResources") {
    dependsOn(copyNativeLib)
}

tasks.named("processTestResources") {
    dependsOn(copyNativeLibForTest)
}