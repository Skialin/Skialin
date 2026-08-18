package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Image internal constructor(ptr: Long) : Managed(ptr, ImageNative::nRelease) {
    val width: Int get() = ImageNative.nWidth(nativePtr)
    val height: Int get() = ImageNative.nHeight(nativePtr)

    fun encodeToPng(): ByteArray? = ImageNative.nEncodeToPng(nativePtr)

    companion object {
        fun decode(bytes: ByteArray): Image? {
            val ptr = ImageNative.nDecode(bytes)
            return if (ptr == 0L) null else Image(ptr)
        }
    }
}

private object ImageNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)
    external fun nWidth(ptr: Long): Int
    external fun nHeight(ptr: Long): Int
    external fun nDecode(bytes: ByteArray): Long
    external fun nEncodeToPng(ptr: Long): ByteArray?
}
