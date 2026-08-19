package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Describes pixel dimensions and encoding. Mirrors Skia's `SkImageInfo`. */
class ImageInfo internal constructor(
    ptr: Long,
) : Managed(ptr, ImageInfoNative::nRelease) {
    val width: Int get() = ImageInfoNative.nWidth(nativePtr)
    val height: Int get() = ImageInfoNative.nHeight(nativePtr)
    val dimensions: ISize get() = ISize(width, height)
    val bounds: IRect get() = IRect(0, 0, width, height)

    val colorType: ColorType get() = ColorType.entries[ImageInfoNative.nColorType(nativePtr)]
    val alphaType: AlphaType get() = AlphaType.entries[ImageInfoNative.nAlphaType(nativePtr)]

    val isEmpty: Boolean get() = ImageInfoNative.nIsEmpty(nativePtr)
    val isOpaque: Boolean get() = ImageInfoNative.nIsOpaque(nativePtr)
    val gammaCloseToSrgb: Boolean get() = ImageInfoNative.nGammaCloseToSrgb(nativePtr)

    val bytesPerPixel: Int get() = ImageInfoNative.nBytesPerPixel(nativePtr)
    val shiftPerPixel: Int get() = ImageInfoNative.nShiftPerPixel(nativePtr)
    val minRowBytes: Long get() = ImageInfoNative.nMinRowBytes(nativePtr)

    fun computeMinByteSize(): Long = ImageInfoNative.nComputeMinByteSize(nativePtr)

    fun computeByteSize(rowBytes: Long): Long = ImageInfoNative.nComputeByteSize(nativePtr, rowBytes)

    fun validRowBytes(rowBytes: Long): Boolean = ImageInfoNative.nValidRowBytes(nativePtr, rowBytes)

    fun withWH(
        width: Int,
        height: Int,
    ): ImageInfo = ImageInfo(ImageInfoNative.nWithWH(nativePtr, width, height))

    fun withColorType(colorType: ColorType): ImageInfo = ImageInfo(ImageInfoNative.nWithColorType(nativePtr, colorType.ordinal))

    fun withAlphaType(alphaType: AlphaType): ImageInfo = ImageInfo(ImageInfoNative.nWithAlphaType(nativePtr, alphaType.ordinal))

    fun withColorSpace(colorSpace: ColorSpace?): ImageInfo =
        ImageInfo(ImageInfoNative.nWithColorSpace(nativePtr, colorSpace?.nativePtr ?: 0L))

    fun contentEquals(other: ImageInfo): Boolean = ImageInfoNative.nEquals(nativePtr, other.nativePtr)

    companion object {
        fun make(
            width: Int,
            height: Int,
            colorType: ColorType = ColorType.N32,
            alphaType: AlphaType = AlphaType.PREMUL,
            colorSpace: ColorSpace? = null,
        ): ImageInfo = ImageInfo(ImageInfoNative.nMake(width, height, colorType.ordinal, alphaType.ordinal, colorSpace?.nativePtr ?: 0L))

        fun makeN32Premul(
            width: Int,
            height: Int,
        ): ImageInfo = make(width, height, ColorType.N32, AlphaType.PREMUL)
    }
}

private object ImageInfoNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(
        width: Int,
        height: Int,
        colorType: Int,
        alphaType: Int,
        colorSpacePtr: Long,
    ): Long

    external fun nRelease(ptr: Long)

    external fun nWidth(ptr: Long): Int

    external fun nHeight(ptr: Long): Int

    external fun nColorType(ptr: Long): Int

    external fun nAlphaType(ptr: Long): Int

    external fun nIsEmpty(ptr: Long): Boolean

    external fun nIsOpaque(ptr: Long): Boolean

    external fun nGammaCloseToSrgb(ptr: Long): Boolean

    external fun nBytesPerPixel(ptr: Long): Int

    external fun nShiftPerPixel(ptr: Long): Int

    external fun nMinRowBytes(ptr: Long): Long

    external fun nComputeMinByteSize(ptr: Long): Long

    external fun nComputeByteSize(
        ptr: Long,
        rowBytes: Long,
    ): Long

    external fun nValidRowBytes(
        ptr: Long,
        rowBytes: Long,
    ): Boolean

    external fun nWithWH(
        ptr: Long,
        width: Int,
        height: Int,
    ): Long

    external fun nWithColorType(
        ptr: Long,
        colorType: Int,
    ): Long

    external fun nWithAlphaType(
        ptr: Long,
        alphaType: Int,
    ): Long

    external fun nWithColorSpace(
        ptr: Long,
        colorSpacePtr: Long,
    ): Long

    external fun nEquals(
        ptr: Long,
        otherPtr: Long,
    ): Boolean
}
