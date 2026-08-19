package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class ColorFilter internal constructor(
    ptr: Long,
) : Managed(ptr, ColorFilterNative::nRelease) {
    enum class InvertStyle { NO_INVERT, INVERT_BRIGHTNESS, INVERT_LIGHTNESS }

    companion object {
        fun makeBlend(
            color: Color,
            mode: BlendMode = BlendMode.SRC_OVER,
        ): ColorFilter? = ColorFilterNative.nBlend(color, mode.ordinal).takeIf { it != 0L }?.let { ColorFilter(it) }

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

        fun makeHSLAMatrix(rowMajor20: FloatArray): ColorFilter? =
            ColorFilterNative.nHSLAMatrix(rowMajor20).takeIf { it != 0L }?.let { ColorFilter(it) }

        val linearToSRGBGamma: ColorFilter get() = ColorFilter(ColorFilterNative.nLinearToSRGBGamma())

        val sRGBToLinearGamma: ColorFilter get() = ColorFilter(ColorFilterNative.nSRGBToLinearGamma())

        fun makeTable(table256: ByteArray): ColorFilter {
            require(table256.size == 256) { "table256 must have exactly 256 entries" }
            return ColorFilter(ColorFilterNative.nTable(table256))
        }

        fun makeTableARGB(
            a: ByteArray? = null,
            r: ByteArray? = null,
            g: ByteArray? = null,
            b: ByteArray? = null,
        ): ColorFilter {
            listOf(a, r, g, b).forEach { require(it == null || it.size == 256) { "each table must have exactly 256 entries" } }
            return ColorFilter(ColorFilterNative.nTableARGB(a, r, g, b))
        }

        fun makeLighting(
            mul: Color,
            add: Color,
        ): ColorFilter = ColorFilter(ColorFilterNative.nLighting(mul, add))

        fun makeHighContrast(
            grayscale: Boolean,
            invertStyle: InvertStyle,
            contrast: Float,
        ): ColorFilter? =
            ColorFilterNative.nHighContrast(grayscale, invertStyle.ordinal, contrast).takeIf { it != 0L }?.let { ColorFilter(it) }

        val luma: ColorFilter get() = ColorFilter(ColorFilterNative.nLuma())
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

    external fun nHSLAMatrix(rowMajor20: FloatArray): Long

    external fun nLinearToSRGBGamma(): Long

    external fun nSRGBToLinearGamma(): Long

    external fun nTable(table256: ByteArray): Long

    external fun nTableARGB(
        a: ByteArray?,
        r: ByteArray?,
        g: ByteArray?,
        b: ByteArray?,
    ): Long

    external fun nLighting(
        mul: Int,
        add: Int,
    ): Long

    external fun nHighContrast(
        grayscale: Boolean,
        invertStyle: Int,
        contrast: Float,
    ): Long

    external fun nLuma(): Long
}
