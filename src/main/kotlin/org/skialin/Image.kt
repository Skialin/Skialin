package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader
import java.nio.ByteBuffer

/** A two-dimensional array of pixels to draw. Mirrors Skia's `SkImage`. */
class Image internal constructor(
    ptr: Long,
) : Managed(ptr, ImageNative::nRelease) {
    val width: Int get() = ImageNative.nWidth(nativePtr)
    val height: Int get() = ImageNative.nHeight(nativePtr)
    val dimensions: ISize get() = ISize(width, height)
    val bounds: IRect get() = IRect(0, 0, width, height)
    val uniqueId: Int get() = ImageNative.nUniqueId(nativePtr)
    val alphaType: AlphaType get() = AlphaType.entries[ImageNative.nAlphaType(nativePtr)]
    val colorType: ColorType get() = ColorType.entries[ImageNative.nColorType(nativePtr)]

    val colorSpace: ColorSpace?
        get() = ImageNative.nColorSpace(nativePtr).takeIf { it != 0L }?.let { ColorSpace(it) }

    val imageInfo: ImageInfo get() = ImageInfo(ImageNative.nImageInfo(nativePtr))

    val isAlphaOnly: Boolean get() = ImageNative.nIsAlphaOnly(nativePtr)
    val isOpaque: Boolean get() = ImageNative.nIsOpaque(nativePtr)
    val isTextureBacked: Boolean get() = ImageNative.nIsTextureBacked(nativePtr)
    val isLazyGenerated: Boolean get() = ImageNative.nIsLazyGenerated(nativePtr)
    val hasMipmaps: Boolean get() = ImageNative.nHasMipmaps(nativePtr)
    val isProtected: Boolean get() = ImageNative.nIsProtected(nativePtr)

    fun makeShader(
        tileX: TileMode = TileMode.CLAMP,
        tileY: TileMode = TileMode.CLAMP,
        sampling: SamplingOptions = SamplingOptions.NEAREST,
        localMatrix: Matrix33? = null,
    ): Shader? {
        val ptr =
            ImageNative.nMakeShader(
                nativePtr,
                tileX.ordinal,
                tileY.ordinal,
                sampling.maxAniso,
                sampling.useCubic,
                sampling.cubicB ?: 0f,
                sampling.cubicC ?: 0f,
                sampling.filter.ordinal,
                sampling.mipmap.ordinal,
                localMatrix?.values,
            )
        return if (ptr == 0L) null else Shader(ptr)
    }

    fun makeRawShader(
        tileX: TileMode = TileMode.CLAMP,
        tileY: TileMode = TileMode.CLAMP,
        sampling: SamplingOptions = SamplingOptions.NEAREST,
        localMatrix: Matrix33? = null,
    ): Shader? {
        val ptr =
            ImageNative.nMakeRawShader(
                nativePtr,
                tileX.ordinal,
                tileY.ordinal,
                sampling.maxAniso,
                sampling.useCubic,
                sampling.cubicB ?: 0f,
                sampling.cubicC ?: 0f,
                sampling.filter.ordinal,
                sampling.mipmap.ordinal,
                localMatrix?.values,
            )
        return if (ptr == 0L) null else Shader(ptr)
    }

    /** A [Pixmap] view over this image's pixels, if it has direct CPU access to them. */
    fun peekPixels(): Pixmap? {
        val ptr = ImageNative.nPeekPixels(nativePtr)
        return if (ptr == 0L) null else Pixmap.wrapNative(ptr, this)
    }

    /** `dst` must be a direct [ByteBuffer] at least `dstRowBytes * dstInfo.height` bytes. */
    fun readPixels(
        dstInfo: ImageInfo,
        dst: ByteBuffer,
        dstRowBytes: Long,
        srcX: Int = 0,
        srcY: Int = 0,
    ): Boolean {
        require(dst.isDirect) { "readPixels requires a direct ByteBuffer" }
        val addr = ImageNative.nBufferAddress(dst)
        return ImageNative.nReadPixels(nativePtr, dstInfo.nativePtr, addr, dstRowBytes, srcX, srcY)
    }

    fun scalePixels(
        dst: Pixmap,
        sampling: SamplingOptions = SamplingOptions.NEAREST,
    ): Boolean =
        ImageNative.nScalePixels(
            nativePtr,
            dst.nativePtr,
            sampling.maxAniso,
            sampling.useCubic,
            sampling.cubicB ?: 0f,
            sampling.cubicC ?: 0f,
            sampling.filter.ordinal,
            sampling.mipmap.ordinal,
        )

    fun makeScaled(
        info: ImageInfo,
        sampling: SamplingOptions = SamplingOptions.NEAREST,
    ): Image? {
        val ptr =
            ImageNative.nMakeScaled(
                nativePtr,
                info.nativePtr,
                sampling.maxAniso,
                sampling.useCubic,
                sampling.cubicB ?: 0f,
                sampling.cubicC ?: 0f,
                sampling.filter.ordinal,
                sampling.mipmap.ordinal,
            )
        return if (ptr == 0L) null else Image(ptr)
    }

    /** The original encoded bytes, if this image was created from an encoded stream (e.g. via [decode]). */
    fun refEncodedData(): Data? = ImageNative.nRefEncodedData(nativePtr).takeIf { it != 0L }?.let { Data(it) }

    fun encodeToPng(): ByteArray? = ImageNative.nEncodeToPng(nativePtr)

    /** [quality] is `[0, 100]`. */
    fun encodeToJpeg(quality: Int = 100): ByteArray? = ImageNative.nEncodeToJpeg(nativePtr, quality)

    /** [quality] is `[0, 100]`; for lossy encoding it's visual quality, for lossless it's
     * compression effort (higher = slower, smaller). */
    fun encodeToWebp(
        quality: Float = 100f,
        lossless: Boolean = false,
    ): ByteArray? = ImageNative.nEncodeToWebp(nativePtr, quality, lossless)

    fun makeSubset(
        subset: IRect,
        mipmapped: Boolean = false,
    ): Image? {
        val ptr = ImageNative.nMakeSubset(nativePtr, subset.left, subset.top, subset.right, subset.bottom, mipmapped)
        return if (ptr == 0L) null else Image(ptr)
    }

    fun withDefaultMipmaps(): Image? = ImageNative.nWithDefaultMipmaps(nativePtr).takeIf { it != 0L }?.let { Image(it) }

    fun makeNonTextureImage(): Image? = ImageNative.nMakeNonTextureImage(nativePtr).takeIf { it != 0L }?.let { Image(it) }

    fun makeRasterImage(allowCaching: Boolean = true): Image? =
        ImageNative
            .nMakeRasterImage(nativePtr, allowCaching)
            .takeIf {
                it != 0L
            }?.let { Image(it) }

    fun asLegacyBitmap(): Bitmap? = ImageNative.nAsLegacyBitmap(nativePtr).takeIf { it != 0L }?.let { Bitmap(it) }

    fun makeColorSpace(
        target: ColorSpace,
        mipmapped: Boolean = false,
    ): Image? = ImageNative.nMakeColorSpace(nativePtr, target.nativePtr, mipmapped).takeIf { it != 0L }?.let { Image(it) }

    fun makeColorTypeAndColorSpace(
        targetColorType: ColorType,
        targetColorSpace: ColorSpace,
        mipmapped: Boolean = false,
    ): Image? =
        ImageNative
            .nMakeColorTypeAndColorSpace(nativePtr, targetColorType.ordinal, targetColorSpace.nativePtr, mipmapped)
            .takeIf {
                it != 0L
            }?.let { Image(it) }

    fun reinterpretColorSpace(target: ColorSpace): Image? =
        ImageNative.nReinterpretColorSpace(nativePtr, target.nativePtr).takeIf { it != 0L }?.let { Image(it) }

    companion object {
        fun decode(bytes: ByteArray): Image? = ImageNative.nDecode(bytes).takeIf { it != 0L }?.let { Image(it) }

        /** A deep copy of `pixmap`'s pixels. */
        fun makeFromPixmapCopy(pixmap: Pixmap): Image? =
            ImageNative.nFromPixmapCopy(pixmap.nativePtr).takeIf { it != 0L }?.let { Image(it) }

        fun makeFromBitmap(bitmap: Bitmap): Image? = ImageNative.nFromBitmap(bitmap.nativePtr).takeIf { it != 0L }?.let { Image(it) }

        /**
         * `pixels`' bytes become the image's pixel storage; no copy is made.
         * `pixels` is ref'd, not consumed: it stays independently valid and
         * closeable afterward.
         */
        fun makeFromData(
            info: ImageInfo,
            pixels: Data,
            rowBytes: Long,
        ): Image? = ImageNative.nFromData(info.nativePtr, pixels.nativePtr, rowBytes).takeIf { it != 0L }?.let { Image(it) }

        /**
         * Wraps [backendTexture], transferring ownership to Skia: the
         * texture is destroyed when the returned image is. `backendTexture`
         * must not be used afterward.
         */
        fun adoptTextureFrom(
            context: DirectContext,
            backendTexture: BackendTexture,
            textureOrigin: SurfaceOrigin,
            colorType: ColorType,
            alphaType: AlphaType,
            colorSpace: ColorSpace? = null,
        ): Image? {
            val ptr =
                ImageNative.nAdoptTextureFrom(
                    context.nativePtr,
                    backendTexture.nativePtr,
                    textureOrigin.ordinal,
                    colorType.ordinal,
                    alphaType.ordinal,
                    colorSpace?.nativePtr ?: 0L,
                )
            return if (ptr == 0L) null else Image(ptr)
        }

        /** [backendTexture] must outlive the returned image. */
        fun wrapGraphiteTexture(
            recorder: GraphiteRecorder,
            backendTexture: GraphiteBackendTexture,
            alphaType: AlphaType,
            origin: SurfaceOrigin,
            colorSpace: ColorSpace? = null,
            generateMipmapsFromBase: Boolean = false,
        ): Image? {
            val ptr =
                ImageNative.nWrapGraphiteTexture(
                    recorder.nativePtr,
                    backendTexture.nativePtr,
                    alphaType.ordinal,
                    colorSpace?.nativePtr ?: 0L,
                    origin.ordinal,
                    generateMipmapsFromBase,
                )
            return if (ptr == 0L) null else Image(ptr)
        }
    }
}

private object ImageNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nBufferAddress(buffer: ByteBuffer): Long

    external fun nDecode(bytes: ByteArray): Long

    external fun nFromPixmapCopy(pixmapPtr: Long): Long

    external fun nFromBitmap(bitmapPtr: Long): Long

    external fun nFromData(
        infoPtr: Long,
        dataPtr: Long,
        rowBytes: Long,
    ): Long

    external fun nRelease(ptr: Long)

    external fun nWidth(ptr: Long): Int

    external fun nHeight(ptr: Long): Int

    external fun nUniqueId(ptr: Long): Int

    external fun nAlphaType(ptr: Long): Int

    external fun nColorType(ptr: Long): Int

    external fun nColorSpace(ptr: Long): Long

    external fun nImageInfo(ptr: Long): Long

    external fun nIsAlphaOnly(ptr: Long): Boolean

    external fun nIsOpaque(ptr: Long): Boolean

    external fun nIsTextureBacked(ptr: Long): Boolean

    external fun nIsLazyGenerated(ptr: Long): Boolean

    external fun nHasMipmaps(ptr: Long): Boolean

    external fun nIsProtected(ptr: Long): Boolean

    external fun nMakeShader(
        ptr: Long,
        tileX: Int,
        tileY: Int,
        maxAniso: Int,
        useCubic: Boolean,
        cubicB: Float,
        cubicC: Float,
        filter: Int,
        mipmap: Int,
        localMatrix: FloatArray?,
    ): Long

    external fun nMakeRawShader(
        ptr: Long,
        tileX: Int,
        tileY: Int,
        maxAniso: Int,
        useCubic: Boolean,
        cubicB: Float,
        cubicC: Float,
        filter: Int,
        mipmap: Int,
        localMatrix: FloatArray?,
    ): Long

    external fun nPeekPixels(ptr: Long): Long

    external fun nReadPixels(
        ptr: Long,
        dstInfoPtr: Long,
        dstAddr: Long,
        dstRowBytes: Long,
        srcX: Int,
        srcY: Int,
    ): Boolean

    external fun nScalePixels(
        ptr: Long,
        dstPixmapPtr: Long,
        maxAniso: Int,
        useCubic: Boolean,
        cubicB: Float,
        cubicC: Float,
        filter: Int,
        mipmap: Int,
    ): Boolean

    external fun nMakeScaled(
        ptr: Long,
        infoPtr: Long,
        maxAniso: Int,
        useCubic: Boolean,
        cubicB: Float,
        cubicC: Float,
        filter: Int,
        mipmap: Int,
    ): Long

    external fun nRefEncodedData(ptr: Long): Long

    external fun nEncodeToPng(ptr: Long): ByteArray?

    external fun nEncodeToJpeg(
        ptr: Long,
        quality: Int,
    ): ByteArray?

    external fun nEncodeToWebp(
        ptr: Long,
        quality: Float,
        lossless: Boolean,
    ): ByteArray?

    external fun nMakeSubset(
        ptr: Long,
        left: Int,
        top: Int,
        right: Int,
        bottom: Int,
        mipmapped: Boolean,
    ): Long

    external fun nWithDefaultMipmaps(ptr: Long): Long

    external fun nMakeNonTextureImage(ptr: Long): Long

    external fun nMakeRasterImage(
        ptr: Long,
        allowCaching: Boolean,
    ): Long

    external fun nAsLegacyBitmap(ptr: Long): Long

    external fun nMakeColorSpace(
        ptr: Long,
        targetPtr: Long,
        mipmapped: Boolean,
    ): Long

    external fun nMakeColorTypeAndColorSpace(
        ptr: Long,
        targetColorType: Int,
        targetColorSpacePtr: Long,
        mipmapped: Boolean,
    ): Long

    external fun nReinterpretColorSpace(
        ptr: Long,
        targetPtr: Long,
    ): Long

    external fun nAdoptTextureFrom(
        contextPtr: Long,
        backendTexturePtr: Long,
        textureOrigin: Int,
        colorType: Int,
        alphaType: Int,
        colorSpacePtr: Long,
    ): Long

    external fun nWrapGraphiteTexture(
        recorderPtr: Long,
        backendTexturePtr: Long,
        alphaType: Int,
        colorSpacePtr: Long,
        origin: Int,
        generateMipmapsFromBase: Boolean,
    ): Long
}
