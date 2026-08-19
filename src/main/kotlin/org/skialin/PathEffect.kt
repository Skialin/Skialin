package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class PathEffect internal constructor(
    ptr: Long,
) : Managed(ptr, PathEffectNative::nRelease) {
    enum class TrimMode { NORMAL, INVERTED }

    companion object {
        fun makeDash(
            intervals: FloatArray,
            phase: Float = 0f,
        ): PathEffect? = PathEffectNative.nDash(intervals, phase).takeIf { it != 0L }?.let { PathEffect(it) }

        fun makeCorner(radius: Float): PathEffect? = PathEffectNative.nCorner(radius).takeIf { it != 0L }?.let { PathEffect(it) }

        fun makeDiscrete(
            segLength: Float,
            deviation: Float,
            seedAssist: Int = 0,
        ): PathEffect? = PathEffectNative.nDiscrete(segLength, deviation, seedAssist).takeIf { it != 0L }?.let { PathEffect(it) }

        fun makeTrim(
            startT: Float,
            stopT: Float,
            mode: TrimMode = TrimMode.NORMAL,
        ): PathEffect? = PathEffectNative.nTrim(startT, stopT, mode.ordinal).takeIf { it != 0L }?.let { PathEffect(it) }

        fun makeCompose(
            outer: PathEffect,
            inner: PathEffect,
        ): PathEffect? = PathEffectNative.nCompose(outer.nativePtr, inner.nativePtr).takeIf { it != 0L }?.let { PathEffect(it) }

        fun makeSum(
            first: PathEffect,
            second: PathEffect,
        ): PathEffect? = PathEffectNative.nSum(first.nativePtr, second.nativePtr).takeIf { it != 0L }?.let { PathEffect(it) }

        fun makePath1D(
            path: Path,
            advance: Float,
            phase: Float,
            style: Path1DStyle = Path1DStyle.TRANSLATE,
        ): PathEffect? = PathEffectNative.nPath1D(path.nativePtr, advance, phase, style.ordinal).takeIf { it != 0L }?.let { PathEffect(it) }

        fun makePath2D(
            matrix: Matrix33,
            path: Path,
        ): PathEffect? = PathEffectNative.nPath2D(matrix.values, path.nativePtr).takeIf { it != 0L }?.let { PathEffect(it) }

        fun makeLine2D(
            width: Float,
            matrix: Matrix33,
        ): PathEffect? = PathEffectNative.nLine2D(width, matrix.values).takeIf { it != 0L }?.let { PathEffect(it) }
    }
}

private object PathEffectNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)

    external fun nDash(
        intervals: FloatArray,
        phase: Float,
    ): Long

    external fun nCorner(radius: Float): Long

    external fun nDiscrete(
        segLength: Float,
        deviation: Float,
        seedAssist: Int,
    ): Long

    external fun nTrim(
        startT: Float,
        stopT: Float,
        mode: Int,
    ): Long

    external fun nCompose(
        outerPtr: Long,
        innerPtr: Long,
    ): Long

    external fun nSum(
        firstPtr: Long,
        secondPtr: Long,
    ): Long

    external fun nPath1D(
        pathPtr: Long,
        advance: Float,
        phase: Float,
        style: Int,
    ): Long

    external fun nPath2D(
        matrix: FloatArray,
        pathPtr: Long,
    ): Long

    external fun nLine2D(
        width: Float,
        matrix: FloatArray,
    ): Long
}
