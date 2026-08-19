package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Transforms colors as they're drawn. Mirrors Skia's `SkColorFilter`. */
class ColorFilter internal constructor(
    ptr: Long,
) : Managed(ptr, ColorFilterNative::nRelease) {
    companion object {
        fun makeBlend(
            color: Color,
            mode: BlendMode = BlendMode.SRC_OVER,
        ): ColorFilter? = ColorFilterNative.nBlend(color, mode.ordinal).takeIf { it != 0L }?.let { ColorFilter(it) }

        /** [rowMajor20] is a 4x5 row-major color matrix. */
        fun makeMatrix(
            rowMajor20: FloatArray,
            clamp: Boolean = true,
        ): ColorFilter? = ColorFilterNative.nMatrix(rowMajor20, clamp).takeIf { it != 0L }?.let { ColorFilter(it) }

        fun makeCompose(
            outer: ColorFilter,
            inner: ColorFilter,
        ): ColorFilter? = ColorFilterNative.nCompose(outer.nativePtr, inner.nativePtr).takeIf { it != 0L }?.let { ColorFilter(it) }

        fun makeLerp(
            t: Float,
            dst: ColorFilter,
            src: ColorFilter,
        ): ColorFilter? = ColorFilterNative.nLerp(t, dst.nativePtr, src.nativePtr).takeIf { it != 0L }?.let { ColorFilter(it) }
    }
}

private object ColorFilterNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)

    external fun nBlend(
        color: Int,
        mode: Int,
    ): Long

    external fun nMatrix(
        rowMajor20: FloatArray,
        clamp: Boolean,
    ): Long

    external fun nCompose(
        outerPtr: Long,
        innerPtr: Long,
    ): Long

    external fun nLerp(
        t: Float,
        dstPtr: Long,
        srcPtr: Long,
    ): Long
}
