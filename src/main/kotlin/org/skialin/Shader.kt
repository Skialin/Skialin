package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Specifies the source color(s) for a [Paint]. Mirrors Skia's `SkShader`. */
class Shader internal constructor(
    ptr: Long,
) : Managed(ptr, ShaderNative::nRelease) {
    val isOpaque: Boolean get() = ShaderNative.nIsOpaque(nativePtr)

    fun withLocalMatrix(matrix: Matrix33): Shader = Shader(ShaderNative.nWithLocalMatrix(nativePtr, matrix.values))

    companion object {
        fun makeEmpty(): Shader = Shader(ShaderNative.nMakeEmpty())

        fun makeColor(color: Color): Shader = Shader(ShaderNative.nMakeColor(color))

        /**
         * A gradient between [p0] and [p1]. [positions], if given, must have the same
         * size as [colors]: strictly increasing values in `[0, 1]`. `null` if
         * [colors] has fewer than 2 entries or [positions] is malformed.
         */
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

        /** A gradient radiating from [center] out to [radius]. [radius] must be positive. */
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

        /** A gradient between two circles; both radii must be non-negative. */
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

        /**
         * A gradient sweeping around [center] from [startAngle] to [endAngle] degrees
         * (0 = positive x axis). [startAngle] must be less than [endAngle].
         */
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
}
