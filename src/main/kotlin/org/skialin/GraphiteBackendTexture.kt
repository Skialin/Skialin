package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class GraphiteBackendTexture internal constructor(
    ptr: Long,
) : Managed(ptr, GraphiteBackendTextureNative::nRelease) {
    val isValid: Boolean get() = GraphiteBackendTextureNative.nIsValid(nativePtr)

    companion object {
        /** Wraps a caller-owned VkImage (not allocated or freed by Skia). */
        @Suppress("LongParameterList")
        fun makeVk(
            width: Int,
            height: Int,
            sampleCount: Int,
            mipmapped: Boolean,
            imageCreateFlags: Int,
            format: Int,
            imageTiling: Int,
            imageUsageFlags: Int,
            sharingMode: Int,
            aspectMask: Int,
            currentLayout: Int,
            queueFamilyIndex: Int,
            image: Long,
            allocMemory: Long = 0L,
            allocOffset: Long = 0L,
            allocSize: Long = 0L,
            allocFlags: Int = 0,
        ): GraphiteBackendTexture {
            val ptr =
                GraphiteBackendTextureNative.nMakeVk(
                    width,
                    height,
                    sampleCount,
                    mipmapped,
                    imageCreateFlags,
                    format,
                    imageTiling,
                    imageUsageFlags,
                    sharingMode,
                    aspectMask,
                    currentLayout,
                    queueFamilyIndex,
                    image,
                    allocMemory,
                    allocOffset,
                    allocSize,
                    allocFlags,
                )
            return GraphiteBackendTexture(ptr)
        }
    }
}

private object GraphiteBackendTextureNative {
    init {
        NativeLoader.ensureLoaded()
    }

    @Suppress("LongParameterList")
    external fun nMakeVk(
        width: Int,
        height: Int,
        sampleCount: Int,
        mipmapped: Boolean,
        imageCreateFlags: Int,
        format: Int,
        imageTiling: Int,
        imageUsageFlags: Int,
        sharingMode: Int,
        aspectMask: Int,
        currentLayout: Int,
        queueFamilyIndex: Int,
        image: Long,
        allocMemory: Long,
        allocOffset: Long,
        allocSize: Long,
        allocFlags: Int,
    ): Long

    external fun nRelease(ptr: Long)

    external fun nIsValid(ptr: Long): Boolean
}
