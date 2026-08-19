package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class BackendTexture internal constructor(ptr: Long) : Managed(ptr, BackendTextureNative::nRelease) {
    val width: Int get() = BackendTextureNative.nWidth(nativePtr)
    val height: Int get() = BackendTextureNative.nHeight(nativePtr)
    val isValid: Boolean get() = BackendTextureNative.nIsValid(nativePtr)
    val isProtected: Boolean get() = BackendTextureNative.nIsProtected(nativePtr)
    val hasMipmaps: Boolean get() = BackendTextureNative.nHasMipmaps(nativePtr)

    companion object {
        /**
         * Wraps a caller-owned VkImage (not allocated or freed by Skia).
         * `image`/`allocMemory` etc are native handles; `imageTiling`,
         * `imageLayout`, `format`, `sharingMode` are raw Vulkan enum values
         * (e.g. from LWJGL's `VK10` constants).
         */
        @Suppress("LongParameterList")
        fun makeVk(
            width: Int,
            height: Int,
            image: Long,
            imageTiling: Int,
            imageLayout: Int,
            format: Int,
            imageUsageFlags: Int,
            sampleCount: Int = 1,
            levelCount: Int = 1,
            currentQueueFamily: Int = -1,
            isProtected: Boolean = false,
            sharingMode: Int = 0,
            label: String = "",
        ): BackendTexture {
            val ptr = BackendTextureNative.nMakeVk(
                width, height, image, imageTiling, imageLayout, format, imageUsageFlags,
                sampleCount, levelCount, currentQueueFamily, isProtected, sharingMode, label,
            )
            return BackendTexture(ptr)
        }

        /**
         * Wraps a caller-owned GL texture (not allocated or freed by Skia).
         * `target` is typically `GL_TEXTURE_2D` (0x0DE1); `format` a sized
         * internal format like `GL_RGBA8` (0x8058).
         */
        fun makeGL(width: Int, height: Int, mipmapped: Boolean, target: Int, id: Int, format: Int, isProtected: Boolean = false, label: String = ""): BackendTexture {
            val ptr = BackendTextureNative.nMakeGL(width, height, mipmapped, target, id, format, isProtected, label)
            return BackendTexture(ptr)
        }
    }
}

private object BackendTextureNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeVk(
        width: Int,
        height: Int,
        image: Long,
        imageTiling: Int,
        imageLayout: Int,
        format: Int,
        imageUsageFlags: Int,
        sampleCount: Int,
        levelCount: Int,
        currentQueueFamily: Int,
        isProtected: Boolean,
        sharingMode: Int,
        label: String,
    ): Long
    external fun nMakeGL(width: Int, height: Int, mipmapped: Boolean, target: Int, id: Int, format: Int, isProtected: Boolean, label: String): Long
    external fun nRelease(ptr: Long)
    external fun nWidth(ptr: Long): Int
    external fun nHeight(ptr: Long): Int
    external fun nIsValid(ptr: Long): Boolean
    external fun nIsProtected(ptr: Long): Boolean
    external fun nHasMipmaps(ptr: Long): Boolean
}
