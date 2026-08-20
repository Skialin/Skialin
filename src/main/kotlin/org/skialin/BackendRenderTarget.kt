package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class BackendRenderTarget internal constructor(
    ptr: Long,
) : Managed(ptr, BackendRenderTargetNative::nRelease) {
    val width: Int get() = BackendRenderTargetNative.nWidth(nativePtr)
    val height: Int get() = BackendRenderTargetNative.nHeight(nativePtr)
    val sampleCnt: Int get() = BackendRenderTargetNative.nSampleCnt(nativePtr)
    val stencilBits: Int get() = BackendRenderTargetNative.nStencilBits(nativePtr)
    val isValid: Boolean get() = BackendRenderTargetNative.nIsValid(nativePtr)
    val isProtected: Boolean get() = BackendRenderTargetNative.nIsProtected(nativePtr)
    val isFramebufferOnly: Boolean get() = BackendRenderTargetNative.nIsFramebufferOnly(nativePtr)

    companion object {
        /**
         * Wraps a caller-owned VkImage-backed render target (not allocated
         * or freed by Skia), e.g. a swapchain image. `image`/`allocMemory`
         * etc are native handles; `imageTiling`, `imageLayout`, `format`,
         * `sharingMode` are raw Vulkan enum values (e.g. from LWJGL's
         * `VK10` constants). `sampleCount` and `stencilBits` are ignored by
         * Skia for Vulkan render targets (sample count comes from the image
         * itself, stencil bits is always reported as 0).
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
        ): BackendRenderTarget {
            val ptr =
                BackendRenderTargetNative.nMakeVk(
                    width,
                    height,
                    image,
                    imageTiling,
                    imageLayout,
                    format,
                    imageUsageFlags,
                    sampleCount,
                    levelCount,
                    currentQueueFamily,
                    isProtected,
                    sharingMode,
                )
            return BackendRenderTarget(ptr)
        }

        /**
         * Wraps a caller-owned GL framebuffer (not allocated or freed by
         * Skia), e.g. FBO 0 for the window-system framebuffer or an
         * app-managed multisampled renderbuffer. `format` is a sized
         * internal format like `GL_RGBA8` (0x8058).
         */
        fun makeGL(
            width: Int,
            height: Int,
            sampleCount: Int,
            stencilBits: Int,
            fboId: Int,
            format: Int,
            isProtected: Boolean = false,
        ): BackendRenderTarget {
            val ptr = BackendRenderTargetNative.nMakeGL(width, height, sampleCount, stencilBits, fboId, format, isProtected)
            return BackendRenderTarget(ptr)
        }
    }
}

private object BackendRenderTargetNative {
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
    ): Long

    external fun nMakeGL(
        width: Int,
        height: Int,
        sampleCount: Int,
        stencilBits: Int,
        fboId: Int,
        format: Int,
        isProtected: Boolean,
    ): Long

    external fun nRelease(ptr: Long)

    external fun nWidth(ptr: Long): Int

    external fun nHeight(ptr: Long): Int

    external fun nSampleCnt(ptr: Long): Int

    external fun nStencilBits(ptr: Long): Int

    external fun nIsValid(ptr: Long): Boolean

    external fun nIsProtected(ptr: Long): Boolean

    external fun nIsFramebufferOnly(ptr: Long): Boolean
}
