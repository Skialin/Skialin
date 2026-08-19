package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/**
 * [Bitmap] describes a two-dimensional raster pixel array.
 *
 * [Bitmap] is built on [ImageInfo], containing integer width and height, [ColorType] and [AlphaType] describing the pixel format, and [ColorSpace] describing the range of colors. [ImageInfo] bounds may be located anywhere fully inside SkPixelRef bounds.
 *
 * [Bitmap] can be drawn using [Canvas]. [Bitmap] can be a drawing destination for [Canvas] draw member functions. [Bitmap] flexibility as a pixel container limits some optimizations available to the target platform.
 *
 * If pixel array is primarily read-only, use SkImage for better performance. If pixel array is primarily written to, use [Surface] for better performance.
 *
 * Declaring [Bitmap] const prevents altering [ImageInfo]: the [Bitmap] height, width, and so on cannot change. Declaring [Bitmap] const affects [Bitmap] configuration, not its contents.
 *
 * [Bitmap] is not thread safe. Each thread must have its own copy of [Bitmap] fields, although threads may share the underlying pixel array.
 */
class Bitmap internal constructor(
    ptr: Long,
) : Managed(ptr, BitmapNative::nRelease) {
    constructor() : this(BitmapNative.nMake())

    fun allocPixels(info: ImageInfo) {
        BitmapNative.nAllocPixels(nativePtr, info.nativePtr)
    }

    fun allocPixels(
        width: Int,
        height: Int,
        colorType: ColorType = ColorType.N32,
        alphaType: AlphaType = AlphaType.PREMUL,
    ) {
        ImageInfo.make(width, height, colorType, alphaType).use { allocPixels(it) }
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

    fun extractSubset(
        dst: Bitmap,
        subset: IRect,
    ): Boolean = BitmapNative.nExtractSubset(nativePtr, dst.nativePtr, subset.left, subset.top, subset.right, subset.bottom)

    fun extractAlpha(dst: Bitmap): Boolean = BitmapNative.nExtractAlpha(nativePtr, dst.nativePtr)

    fun notifyPixelsChanged() = BitmapNative.nNotifyPixelsChanged(nativePtr)
}

private object BitmapNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(): Long

    external fun nRelease(ptr: Long)

    external fun nAllocPixels(
        ptr: Long,
        infoPtr: Long,
    )

    external fun nWidth(ptr: Long): Int

    external fun nHeight(ptr: Long): Int

    external fun nRowBytes(ptr: Long): Long

    external fun nEraseColor(
        ptr: Long,
        color: Int,
    )

    external fun nReadPixels(ptr: Long): ByteArray

    external fun nAsImage(ptr: Long): Long

    external fun nExtractSubset(
        ptr: Long,
        dstPtr: Long,
        left: Int,
        top: Int,
        right: Int,
        bottom: Int,
    ): Boolean

    external fun nExtractAlpha(
        ptr: Long,
        dstPtr: Long,
    ): Boolean

    external fun nNotifyPixelsChanged(ptr: Long)
}
