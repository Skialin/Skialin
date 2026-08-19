package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader
import java.nio.ByteBuffer

data class CodecFrameInfo(
    val durationMs: Int,
    /** The earliest frame this one can be blended with, if any. */
    val requiredFrame: Int?,
    val fullyReceived: Boolean,
)

/**
 * Decodes an image, exposing multi-frame (animated GIF/WEBP) introspection
 * and explicit per-frame decoding beyond [Image.decode]'s implicit
 * first-frame-only decode. Mirrors Skia's `SkCodec`.
 */
class Codec private constructor(
    ptr: Long,
) : Managed(ptr, CodecNative::nRelease) {
    val dimensions: ISize
        get() = CodecNative.nDimensions(nativePtr).let { ISize(it[0], it[1]) }

    /** The container format's `SkEncodedImageFormat` ordinal (0 = BMP, 1 = GIF, 2 = ICO,
     * 3 = JPEG, 4 = PNG, 5 = WBMP, 6 = WEBP, 7 = PKM, 8 = KTX, 9 = ASTC, 10 = DNG,
     * 11 = HEIF, 12 = AVIF, 13 = JPEGXL). */
    val encodedFormat: Int get() = CodecNative.nGetEncodedFormat(nativePtr)

    /** 1 for a static image; the number of frames for an animated one. */
    val frameCount: Int get() = CodecNative.nGetFrameCount(nativePtr)

    fun frameInfo(index: Int): CodecFrameInfo? =
        CodecNative.nGetFrameInfo(nativePtr, index)?.let {
            CodecFrameInfo(durationMs = it[0], requiredFrame = it[1].takeIf { r -> r >= 0 }, fullyReceived = it[2] != 0)
        }

    /** Decodes [frameIndex] (`0` for static images) into [dst], a direct [ByteBuffer] at
     * least `dstRowBytes * height` bytes matching [dstInfo]. */
    fun getPixels(
        dstInfo: ImageInfo,
        dst: ByteBuffer,
        dstRowBytes: Long,
        frameIndex: Int = 0,
    ): Boolean {
        require(dst.isDirect) { "getPixels requires a direct ByteBuffer" }
        val addr = CodecNative.nBufferAddress(dst)
        return CodecNative.nGetPixels(nativePtr, dstInfo.nativePtr, addr, dstRowBytes, frameIndex)
    }

    companion object {
        /** `null` if [bytes] isn't a recognized image format. */
        fun makeFromBytes(bytes: ByteArray): Codec? = CodecNative.nMakeFromBytes(bytes).takeIf { it != 0L }?.let { Codec(it) }
    }
}

private object CodecNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeFromBytes(bytes: ByteArray): Long

    external fun nRelease(ptr: Long)

    external fun nDimensions(ptr: Long): IntArray

    external fun nGetEncodedFormat(ptr: Long): Int

    external fun nGetFrameCount(ptr: Long): Int

    external fun nGetFrameInfo(
        ptr: Long,
        index: Int,
    ): IntArray?

    external fun nBufferAddress(buffer: ByteBuffer): Long

    external fun nGetPixels(
        ptr: Long,
        dstInfoPtr: Long,
        dstAddr: Long,
        dstRowBytes: Long,
        frameIndex: Int,
    ): Boolean
}
