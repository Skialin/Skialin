package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Bitmap : Managed(BitmapNative.nMake(), BitmapNative::nRelease) {
    fun allocPixels(
        width: Int,
        height: Int,
        colorType: ColorType = ColorType.RGBA_8888,
        alphaType: AlphaType = AlphaType.PREMUL,
    ) {
        BitmapNative.nAllocPixels(nativePtr, width, height, colorType.ordinal, alphaType.ordinal)
    }

    val width: Int get() = BitmapNative.nWidth(nativePtr)
    val height: Int get() = BitmapNative.nHeight(nativePtr)
    val rowBytes: Long get() = BitmapNative.nRowBytes(nativePtr)

    fun eraseColor(color: Color) = BitmapNative.nEraseColor(nativePtr, color)

    /**
     * @return copy of the current pixel buffer
     */
    fun readPixels(): ByteArray = BitmapNative.nReadPixels(nativePtr)

    fun asImage(): Image? {
        val ptr = BitmapNative.nAsImage(nativePtr)
        return if (ptr == 0L) null else Image(ptr)
    }
}

private object BitmapNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(): Long
    external fun nRelease(ptr: Long)
    external fun nAllocPixels(ptr: Long, width: Int, height: Int, colorType: Int, alphaType: Int)
    external fun nWidth(ptr: Long): Int
    external fun nHeight(ptr: Long): Int
    external fun nRowBytes(ptr: Long): Long
    external fun nEraseColor(ptr: Long, color: Int)
    external fun nReadPixels(ptr: Long): ByteArray
    external fun nAsImage(ptr: Long): Long
}
