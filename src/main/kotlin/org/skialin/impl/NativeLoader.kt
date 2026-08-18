package org.skialin.impl

import java.nio.file.Files
import java.nio.file.StandardCopyOption

internal object NativeLoader {
    private var loaded = false

    @Synchronized
    fun ensureLoaded() {
        if (loaded) return
        val libName = System.mapLibraryName("skialin_jni")
        val resourcePath = "/natives/${platformDir()}/$libName"
        val resource = NativeLoader::class.java.getResourceAsStream(resourcePath)
            ?: throw UnsatisfiedLinkError("skialin: no bundled native library at $resourcePath")

        val tempFile = Files.createTempFile("skialin_jni", suffixFor(libName)).toFile()
        tempFile.deleteOnExit()
        resource.use { input ->
            Files.copy(input, tempFile.toPath(), StandardCopyOption.REPLACE_EXISTING)
        }
        System.load(tempFile.absolutePath)
        loaded = true
    }

    private fun suffixFor(libName: String): String {
        val dot = libName.lastIndexOf('.')
        return if (dot >= 0) libName.substring(dot) else ""
    }

    private fun platformDir(): String {
        val osName = System.getProperty("os.name").lowercase()
        val arch = when (val a = System.getProperty("os.arch").lowercase()) {
            "amd64", "x86_64" -> "x64"
            "aarch64", "arm64" -> "arm64"
            else -> a
        }
        val os = when {
            osName.contains("win") -> "windows"
            osName.contains("mac") -> "macos"
            osName.contains("linux") -> "linux"
            else -> throw UnsatisfiedLinkError("skialin: unsupported OS $osName")
        }
        return "$os-$arch"
    }
}
