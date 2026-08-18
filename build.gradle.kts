plugins {
    kotlin("jvm") version "2.4.10"
}

group = "dev.lunasa"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
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

val copyNativeLib by tasks.registering(Copy::class) {
    onlyIf { buildNative }
    dependsOn(cargoBuild)
    from(rustDir.dir("target/$cargoProfile")) {
        include(nativeLibName)
    }
    into(layout.buildDirectory.dir("resources/main/natives/$nativePlatformDir"))
}

tasks.named("processResources") {
    dependsOn(copyNativeLib)
}