package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Picture internal constructor(
    ptr: Long,
) : Managed(ptr, PictureNative::nRelease) {
    fun playback(canvas: Canvas) = PictureNative.nPlayback(nativePtr, canvas.ptr)

    val cullRect: Rect
        get() = PictureNative.nCullRect(nativePtr).let { Rect(it[0], it[1], it[2], it[3]) }

    val uniqueId: Long get() = PictureNative.nUniqueID(nativePtr)

    fun approximateOpCount(nested: Boolean = false): Int = PictureNative.nApproximateOpCount(nativePtr, nested)
}

private object PictureNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)

    external fun nPlayback(
        ptr: Long,
        canvasPtr: Long,
    )

    external fun nCullRect(ptr: Long): FloatArray

    external fun nUniqueID(ptr: Long): Long

    external fun nApproximateOpCount(
        ptr: Long,
        nested: Boolean,
    ): Int
}
