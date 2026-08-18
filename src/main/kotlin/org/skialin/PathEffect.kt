package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Transforms a path's geometry before it's stroked/filled (e.g. into a dashed line). Mirrors Skia's `SkPathEffect`. */
class PathEffect internal constructor(ptr: Long) : Managed(ptr, PathEffectNative::nRelease) {
    enum class TrimMode { NORMAL, INVERTED }

    companion object {
        /** `null` if [intervals] is empty, has an odd size, or any interval is negative. */
        fun makeDash(intervals: FloatArray, phase: Float = 0f): PathEffect? =
            PathEffectNative.nDash(intervals, phase).takeIf { it != 0L }?.let { PathEffect(it) }

        /** Rounds each corner of the path to [radius]. */
        fun makeCorner(radius: Float): PathEffect? = PathEffectNative.nCorner(radius).takeIf { it != 0L }?.let { PathEffect(it) }

        /** Roughens the path by displacing points along it. */
        fun makeDiscrete(segLength: Float, deviation: Float, seedAssist: Int = 0): PathEffect? =
            PathEffectNative.nDiscrete(segLength, deviation, seedAssist).takeIf { it != 0L }?.let { PathEffect(it) }

        /** Keeps only a `[startT, stopT]` subset of the path (or its complement in [TrimMode.INVERTED]). */
        fun makeTrim(startT: Float, stopT: Float, mode: TrimMode = TrimMode.NORMAL): PathEffect? =
            PathEffectNative.nTrim(startT, stopT, mode.ordinal).takeIf { it != 0L }?.let { PathEffect(it) }

        /** `result = outer(inner(path))`. */
        fun makeCompose(outer: PathEffect, inner: PathEffect): PathEffect? =
            PathEffectNative.nCompose(outer.nativePtr, inner.nativePtr).takeIf { it != 0L }?.let { PathEffect(it) }

        /** Applies [first] and [second] independently, then draws both results. */
        fun makeSum(first: PathEffect, second: PathEffect): PathEffect? =
            PathEffectNative.nSum(first.nativePtr, second.nativePtr).takeIf { it != 0L }?.let { PathEffect(it) }
    }
}

private object PathEffectNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)
    external fun nDash(intervals: FloatArray, phase: Float): Long
    external fun nCorner(radius: Float): Long
    external fun nDiscrete(segLength: Float, deviation: Float, seedAssist: Int): Long
    external fun nTrim(startT: Float, stopT: Float, mode: Int): Long
    external fun nCompose(outerPtr: Long, innerPtr: Long): Long
    external fun nSum(firstPtr: Long, secondPtr: Long): Long
}
