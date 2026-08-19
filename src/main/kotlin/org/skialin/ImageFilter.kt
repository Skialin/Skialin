package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class ImageFilter internal constructor(
    ptr: Long,
) : Managed(ptr, ImageFilterNative::nRelease) {
    companion object {
        fun makeBlur(
            sigmaX: Float,
            sigmaY: Float,
            tileMode: TileMode = TileMode.DECAL,
            input: ImageFilter? = null,
        ): ImageFilter? =
            ImageFilterNative.nBlur(sigmaX, sigmaY, tileMode.ordinal, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeDropShadow(
            dx: Float,
            dy: Float,
            sigmaX: Float,
            sigmaY: Float,
            color: Color,
            input: ImageFilter? = null,
        ): ImageFilter? =
            ImageFilterNative
                .nDropShadow(
                    dx,
                    dy,
                    sigmaX,
                    sigmaY,
                    color,
                    input?.nativePtr ?: 0L,
                ).takeIf { it != 0L }
                ?.let { ImageFilter(it) }

        fun makeDropShadowOnly(
            dx: Float,
            dy: Float,
            sigmaX: Float,
            sigmaY: Float,
            color: Color,
            input: ImageFilter? = null,
        ): ImageFilter? =
            ImageFilterNative
                .nDropShadowOnly(
                    dx,
                    dy,
                    sigmaX,
                    sigmaY,
                    color,
                    input?.nativePtr ?: 0L,
                ).takeIf { it != 0L }
                ?.let { ImageFilter(it) }

        fun makeOffset(
            dx: Float,
            dy: Float,
            input: ImageFilter? = null,
        ): ImageFilter? = ImageFilterNative.nOffset(dx, dy, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeColorFilter(
            colorFilter: ColorFilter,
            input: ImageFilter? = null,
        ): ImageFilter? =
            ImageFilterNative.nColorFilter(colorFilter.nativePtr, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeCompose(
            outer: ImageFilter,
            inner: ImageFilter,
        ): ImageFilter? = ImageFilterNative.nCompose(outer.nativePtr, inner.nativePtr).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeMatrixTransform(
            matrix: Matrix33,
            sampling: SamplingOptions = SamplingOptions.NEAREST,
            input: ImageFilter? = null,
        ): ImageFilter? =
            ImageFilterNative
                .nMatrixTransform(
                    matrix.values,
                    sampling.maxAniso,
                    sampling.useCubic,
                    sampling.cubicB ?: 0f,
                    sampling.cubicC ?: 0f,
                    sampling.filter.ordinal,
                    sampling.mipmap.ordinal,
                    input?.nativePtr ?: 0L,
                ).takeIf { it != 0L }
                ?.let { ImageFilter(it) }

        fun makeDilate(
            radiusX: Float,
            radiusY: Float,
            input: ImageFilter? = null,
        ): ImageFilter? = ImageFilterNative.nDilate(radiusX, radiusY, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeErode(
            radiusX: Float,
            radiusY: Float,
            input: ImageFilter? = null,
        ): ImageFilter? = ImageFilterNative.nErode(radiusX, radiusY, input?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeBlend(
            mode: BlendMode,
            background: ImageFilter? = null,
            foreground: ImageFilter? = null,
        ): ImageFilter? =
            ImageFilterNative.nBlend(mode.ordinal, background?.nativePtr ?: 0L, foreground?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeMerge(
            first: ImageFilter? = null,
            second: ImageFilter? = null,
        ): ImageFilter? = ImageFilterNative.nMerge(first?.nativePtr ?: 0L, second?.nativePtr ?: 0L).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeShader(shader: Shader): ImageFilter? = ImageFilterNative.nShader(shader.nativePtr).takeIf { it != 0L }?.let { ImageFilter(it) }

        fun makeTile(
            src: Rect,
            dst: Rect,
            input: ImageFilter? = null,
        ): ImageFilter? =
            ImageFilterNative
                .nTile(src.left, src.top, src.right, src.bottom, dst.left, dst.top, dst.right, dst.bottom, input?.nativePtr ?: 0L)
                .takeIf { it != 0L }
                ?.let { ImageFilter(it) }
    }
}

private object ImageFilterNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)

    external fun nBlur(
        sigmaX: Float,
        sigmaY: Float,
        tileMode: Int,
        inputPtr: Long,
    ): Long

    external fun nDropShadow(
        dx: Float,
        dy: Float,
        sigmaX: Float,
        sigmaY: Float,
        color: Int,
        inputPtr: Long,
    ): Long

    external fun nDropShadowOnly(
        dx: Float,
        dy: Float,
        sigmaX: Float,
        sigmaY: Float,
        color: Int,
        inputPtr: Long,
    ): Long

    external fun nOffset(
        dx: Float,
        dy: Float,
        inputPtr: Long,
    ): Long

    external fun nColorFilter(
        colorFilterPtr: Long,
        inputPtr: Long,
    ): Long

    external fun nCompose(
        outerPtr: Long,
        innerPtr: Long,
    ): Long

    external fun nMatrixTransform(
        matrix: FloatArray,
        maxAniso: Int,
        useCubic: Boolean,
        cubicB: Float,
        cubicC: Float,
        filter: Int,
        mipmap: Int,
        inputPtr: Long,
    ): Long

    external fun nDilate(
        radiusX: Float,
        radiusY: Float,
        inputPtr: Long,
    ): Long

    external fun nErode(
        radiusX: Float,
        radiusY: Float,
        inputPtr: Long,
    ): Long

    external fun nBlend(
        mode: Int,
        backgroundPtr: Long,
        foregroundPtr: Long,
    ): Long

    external fun nMerge(
        firstPtr: Long,
        secondPtr: Long,
    ): Long

    external fun nShader(shaderPtr: Long): Long

    external fun nTile(
        srcLeft: Float,
        srcTop: Float,
        srcRight: Float,
        srcBottom: Float,
        dstLeft: Float,
        dstTop: Float,
        dstRight: Float,
        dstBottom: Float,
        inputPtr: Long,
    ): Long
}
