plugins {
    kotlin("jvm") version "2.4.10"
}

group = "dev.lunasa"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

val osName = run {
    val os = System.getProperty("os.name").lowercase()
    when {
        os.contains("win") -> "windows"
        os.contains("mac") -> "macos"
        os.contains("linux") -> "linux"
        else -> error("skialin: unsupported OS $os")
    }
}

val lwjglVersion = "3.3.4"
val lwjglNatives = "natives-$osName"

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
    val archProp = System.getProperty("os.arch").lowercase()
    val arch = when (archProp) {
        "amd64", "x86_64" -> "x64"
        "aarch64", "arm64" -> "arm64"
        else -> archProp
    }
    "$osName-$arch"
}

val nativeLibName = when (osName) {
    "windows" -> "skialin_jni.dll"
    "macos" -> "libskialin_jni.dylib"
    else -> "libskialin_jni.so"
}

val skiaDir = layout.projectDirectory.dir("external/skia")

/**
 * Regenerates external/skia/out/Release from the tracked args.gn in
 * native-shim/, then builds the static libs skialin-sys links against.
 * Needed after a fresh clone or after `git submodule update` bumps skia
 * (out/ is gitignored inside the skia submodule itself). Requires
 * depot_tools (for ninja) on PATH.
 */
val setupSkia by tasks.registering(Exec::class) {
    workingDir = skiaDir.asFile
    doFirst {
        val outDir = skiaDir.dir("out/Release").asFile
        outDir.mkdirs()
        skiaDir.file("../../native-shim/args.gn").asFile.copyTo(outDir.resolve("args.gn"), overwrite = true)
    }
    val gn = if (osName == "windows") "bin/gn.exe" else "bin/gn"
    commandLine(gn, "gen", "out/Release")
}

val buildSkia by tasks.registering(Exec::class) {
    dependsOn(setupSkia)
    workingDir = skiaDir.asFile
    val ninja = if (osName == "windows") "ninja.exe" else "ninja"
    commandLine(
        ninja, "-C", "out/Release",
        "skia", "skparagraph", "skshaper", "skunicode_core", "skunicode_icu", "skcms",
        "libpng", "zlib", "expat", "harfbuzz", "icu", "pathops",
    )
}

val skiaLibDir = providers.gradleProperty("skialin.skiaLibDir")
    .orElse(providers.environmentVariable("SKIALIN_SKIA_LIB_DIR"))
    .orElse(skiaDir.dir("out/Release").asFile.absolutePath)

val cargoBuild by tasks.registering(Exec::class) {
    onlyIf { buildNative }
    workingDir = rustDir.asFile
    environment("SKIALIN_SKIA_LIB_DIR", skiaLibDir.get())
    commandLine("cargo", "build", "-p", "skialin-jni", "--release")
}

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