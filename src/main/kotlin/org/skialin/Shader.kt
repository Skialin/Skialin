package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Shader internal constructor(
    ptr: Long,
) : Managed(ptr, ShaderNative::nRelease) {
    val isOpaque: Boolean get() = ShaderNative.nIsOpaque(nativePtr)

    fun withLocalMatrix(matrix: Matrix33): Shader = Shader(ShaderNative.nWithLocalMatrix(nativePtr, matrix.values))

    companion object {
        fun makeEmpty(): Shader = Shader(ShaderNative.nMakeEmpty())

        fun makeColor(color: Color): Shader = Shader(ShaderNative.nMakeColor(color))

        fun makeLinearGradient(
            p0: Point,
            p1: Point,
            colors: IntArray,
            positions: FloatArray? = null,
            tileMode: TileMode = TileMode.CLAMP,
            localMatrix: Matrix33? = null,
        ): Shader? {
            val ptr = ShaderNative.nMakeLinearGradient(p0.x, p0.y, p1.x, p1.y, colors, positions, tileMode.ordinal, localMatrix?.values)
            return if (ptr == 0L) null else Shader(ptr)
        }

        fun makeRadialGradient(
            center: Point,
            radius: Float,
            colors: IntArray,
            positions: FloatArray? = null,
            tileMode: TileMode = TileMode.CLAMP,
            localMatrix: Matrix33? = null,
        ): Shader? {
            val ptr = ShaderNative.nMakeRadialGradient(center.x, center.y, radius, colors, positions, tileMode.ordinal, localMatrix?.values)
            return if (ptr == 0L) null else Shader(ptr)
        }

        fun makeTwoPointConicalGradient(
            start: Point,
            startRadius: Float,
            end: Point,
            endRadius: Float,
            colors: IntArray,
            positions: FloatArray? = null,
            tileMode: TileMode = TileMode.CLAMP,
            localMatrix: Matrix33? = null,
        ): Shader? {
            val ptr =
                ShaderNative.nMakeTwoPointConicalGradient(
                    start.x,
                    start.y,
                    startRadius,
                    end.x,
                    end.y,
                    endRadius,
                    colors,
                    positions,
                    tileMode.ordinal,
                    localMatrix?.values,
                )
            return if (ptr == 0L) null else Shader(ptr)
        }

        fun makeSweepGradient(
            center: Point,
            colors: IntArray,
            positions: FloatArray? = null,
            startAngle: Float = 0f,
            endAngle: Float = 360f,
            tileMode: TileMode = TileMode.CLAMP,
            localMatrix: Matrix33? = null,
        ): Shader? {
            val ptr =
                ShaderNative.nMakeSweepGradient(
                    center.x,
                    center.y,
                    startAngle,
                    endAngle,
                    colors,
                    positions,
                    tileMode.ordinal,
                    localMatrix?.values,
                )
            return if (ptr == 0L) null else Shader(ptr)
        }

        fun makeBlend(
            mode: BlendMode,
            dst: Shader,
            src: Shader,
        ): Shader? = ShaderNative.nBlend(mode.ordinal, dst.nativePtr, src.nativePtr).takeIf { it != 0L }?.let { Shader(it) }

        fun makeFractalNoise(
            baseFreqX: Float,
            baseFreqY: Float,
            numOctaves: Int,
            seed: Float,
        ): Shader? = ShaderNative.nFractalNoise(baseFreqX, baseFreqY, numOctaves, seed).takeIf { it != 0L }?.let { Shader(it) }

        fun makeTurbulence(
            baseFreqX: Float,
            baseFreqY: Float,
            numOctaves: Int,
            seed: Float,
        ): Shader? = ShaderNative.nTurbulence(baseFreqX, baseFreqY, numOctaves, seed).takeIf { it != 0L }?.let { Shader(it) }
    }
}

private object ShaderNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeEmpty(): Long

    external fun nMakeColor(color: Int): Long

    external fun nRelease(ptr: Long)

    external fun nIsOpaque(ptr: Long): Boolean

    external fun nWithLocalMatrix(
        ptr: Long,
        matrix: FloatArray,
    ): Long

    external fun nMakeLinearGradient(
        x0: Float,
        y0: Float,
        x1: Float,
        y1: Float,
        colors: IntArray,
        positions: FloatArray?,
        tileMode: Int,
        localMatrix: FloatArray?,
    ): Long

    external fun nMakeRadialGradient(
        cx: Float,
        cy: Float,
        radius: Float,
        colors: IntArray,
        positions: FloatArray?,
        tileMode: Int,
        localMatrix: FloatArray?,
    ): Long

    external fun nMakeTwoPointConicalGradient(
        startX: Float,
        startY: Float,
        startRadius: Float,
        endX: Float,
        endY: Float,
        endRadius: Float,
        colors: IntArray,
        positions: FloatArray?,
        tileMode: Int,
        localMatrix: FloatArray?,
    ): Long

    external fun nMakeSweepGradient(
        cx: Float,
        cy: Float,
        startAngle: Float,
        endAngle: Float,
        colors: IntArray,
        positions: FloatArray?,
        tileMode: Int,
        localMatrix: FloatArray?,
    ): Long

    external fun nBlend(
        mode: Int,
        dstPtr: Long,
        srcPtr: Long,
    ): Long

    external fun nFractalNoise(
        baseFreqX: Float,
        baseFreqY: Float,
        numOctaves: Int,
        seed: Float,
    ): Long

    external fun nTurbulence(
        baseFreqX: Float,
        baseFreqY: Float,
        numOctaves: Int,
        seed: Float,
    ): Long
}
