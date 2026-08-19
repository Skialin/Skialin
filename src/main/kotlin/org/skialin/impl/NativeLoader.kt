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
        val resource =
            NativeLoader::class.java.getResourceAsStream(resourcePath)
                ?: throw UnsatisfiedLinkError("skialin: no bundled native library at $resourcePath")

        val tempFile = Files.createTempFile("skialin_jni", suffixFor(libName)).toFile()
        tempFile.deleteOnExit()
        resource.use { input ->
            Files.copy(input, tempFile.toPath(), StandardCopyOption.REPLACE_EXISTING)
        }
        extractIcuData(tempFile.parentFile)
        System.load(tempFile.absolutePath)
        loaded = true
    }

    /**
     * SkLoadICU() (third_party/icu/SkLoadICU.cpp) looks for icudtl.dat next
     * to the module it's compiled into, i.e. next to [tempFile] once
     * loaded. icudtl.dat is bundled once at a common, non-platform-specific
     * resource path since its contents don't vary by platform.
     */
    private fun extractIcuData(destDir: java.io.File) {
        val dest = destDir.resolve("icudtl.dat")
        if (dest.isFile) return
        val resource = NativeLoader::class.java.getResourceAsStream("/natives/icudtl.dat") ?: return
        resource.use { input ->
            Files.copy(input, dest.toPath(), StandardCopyOption.REPLACE_EXISTING)
        }
    }

    private fun suffixFor(libName: String): String {
        val dot = libName.lastIndexOf('.')
        return if (dot >= 0) libName.substring(dot) else ""
    }

    private fun platformDir(): String {
        val osName = System.getProperty("os.name").lowercase()
        val arch =
            when (val a = System.getProperty("os.arch").lowercase()) {
                "amd64", "x86_64" -> "x64"
                "aarch64", "arm64" -> "arm64"
                else -> a
            }
        val os =
            when {
                osName.contains("win") -> "windows"
                osName.contains("mac") -> "macos"
                osName.contains("linux") -> "linux"
                else -> throw UnsatisfiedLinkError("skialin: unsupported OS $osName")
            }
        return "$os-$arch"
    }
}
