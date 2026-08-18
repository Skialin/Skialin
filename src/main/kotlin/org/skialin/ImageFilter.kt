package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** A graph of image-space effects (blur, shadow, offset, ...) applied before drawing. Mirrors Skia's `SkImageFilter`. */
class ImageFilter internal constructor(ptr: Long) : Managed(ptr, ImageFilterNative::nRelease) {
    companion object {
        fun makeBlur(sigmaX: Float, sigmaY: Float, tileMode: TileMode = TileMode.DECAL, input: ImageFilter? = null): ImageFilter? =
            ImageFilterNative.nBlur(sigmaX, sigmaY, tileMode.ordinal, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeDropShadow(dx: Float, dy: Float, sigmaX: Float, sigmaY: Float, color: Color, input: ImageFilter? = null): ImageFilter? =
            ImageFilterNative.nDropShadow(dx, dy, sigmaX, sigmaY, color, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        /** Renders the drop shadow without the input content, so callers can compose the shadow and input in their own filter graph. */
        fun makeDropShadowOnly(dx: Float, dy: Float, sigmaX: Float, sigmaY: Float, color: Color, input: ImageFilter? = null): ImageFilter? =
            ImageFilterNative.nDropShadowOnly(dx, dy, sigmaX, sigmaY, color, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeOffset(dx: Float, dy: Float, input: ImageFilter? = null): ImageFilter? =
            ImageFilterNative.nOffset(dx, dy, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeColorFilter(colorFilter: ColorFilter, input: ImageFilter? = null): ImageFilter? =
            ImageFilterNative.nColorFilter(colorFilter.nativePtr, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        /** `result = outer(inner(source))`. */
        fun makeCompose(outer: ImageFilter, inner: ImageFilter): ImageFilter? =
            ImageFilterNative.nCompose(outer.nativePtr, inner.nativePtr).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeMatrixTransform(matrix: Matrix33, sampling: SamplingOptions = SamplingOptions.NEAREST, input: ImageFilter? = null): ImageFilter? =
            ImageFilterNative.nMatrixTransform(
                matrix.values, sampling.maxAniso, sampling.useCubic, sampling.cubicB ?: 0f, sampling.cubicC ?: 0f,
                sampling.filter.ordinal, sampling.mipmap.ordinal, input?.nativePtr ?: 0L,
            ).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeDilate(radiusX: Float, radiusY: Float, input: ImageFilter? = null): ImageFilter? =
            ImageFilterNative.nDilate(radiusX, radiusY, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeErode(radiusX: Float, radiusY: Float, input: ImageFilter? = null): ImageFilter? =
            ImageFilterNative.nErode(radiusX, radiusY, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }
    }
}

private object ImageFilterNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)
    external fun nBlur(sigmaX: Float, sigmaY: Float, tileMode: Int, inputPtr: Long): Long
    external fun nDropShadow(dx: Float, dy: Float, sigmaX: Float, sigmaY: Float, color: Int, inputPtr: Long): Long
    external fun nDropShadowOnly(dx: Float, dy: Float, sigmaX: Float, sigmaY: Float, color: Int, inputPtr: Long): Long
    external fun nOffset(dx: Float, dy: Float, inputPtr: Long): Long
    external fun nColorFilter(colorFilterPtr: Long, inputPtr: Long): Long
    external fun nCompose(outerPtr: Long, innerPtr: Long): Long
    external fun nMatrixTransform(
        matrix: FloatArray, maxAniso: Int, useCubic: Boolean, cubicB: Float, cubicC: Float, filter: Int, mipmap: Int,
        inputPtr: Long,
    ): Long
    external fun nDilate(radiusX: Float, radiusY: Float, inputPtr: Long): Long
    external fun nErode(radiusX: Float, radiusY: Float, inputPtr: Long): Long
}
