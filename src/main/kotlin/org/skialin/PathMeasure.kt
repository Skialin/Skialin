package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Measures distance along a path (length, position/tangent, matrix at a point). Mirrors Skia's `SkPathMeasure`. */
class PathMeasure(
    path: Path,
    forceClosed: Boolean = false,
    resScale: Float = 1f,
) : Managed(PathMeasureNative.nNew(path.nativePtr, forceClosed, resScale), PathMeasureNative::nRelease) {
    data class PosTan(
        val position: Point,
        val tangent: Point,
    )

    /** `path` of `null` clears the current path. */
    fun setPath(
        path: Path?,
        forceClosed: Boolean = false,
    ) = PathMeasureNative.nSetPath(nativePtr, path?.nativePtr ?: 0L, forceClosed)

    /** The length of the current contour, or 0 if there's no path. */
    fun length(): Float = PathMeasureNative.nLength(nativePtr)

    /** [distance] is pinned to `[0, length]`. `null` if there's no path or it's zero-length. */
    fun posTan(distance: Float): PosTan? {
        val out = FloatArray(4)
        if (!PathMeasureNative.nPosTan(nativePtr, distance, out)) return null
        return PosTan(Point(out[0], out[1]), Point(out[2], out[3]))
    }

    /** [distance] is pinned to `[0, length]`. `null` if there's no path or it's zero-length. */
    fun matrix(distance: Float): Matrix33? {
        val out = FloatArray(9)
        if (!PathMeasureNative.nMatrix(nativePtr, distance, out)) return null
        return Matrix33(out)
    }

    /**
     * Appends the `[startD, stopD]` segment of the current contour to [dst]. Returns false
     * ([dst] untouched) if the segment is zero-length or `startD > stopD`.
     */
    fun segment(
        startD: Float,
        stopD: Float,
        dst: PathBuilder,
        startWithMoveTo: Boolean,
    ): Boolean = PathMeasureNative.nSegment(nativePtr, startD, stopD, dst.nativePtr, startWithMoveTo)

    val isClosed: Boolean get() = PathMeasureNative.nIsClosed(nativePtr)

    /** Advances to the next contour in the path. `true` if one exists. */
    fun nextContour(): Boolean = PathMeasureNative.nNextContour(nativePtr)
}

private object PathMeasureNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nNew(
        pathPtr: Long,
        forceClosed: Boolean,
        resScale: Float,
    ): Long

    external fun nRelease(ptr: Long)

    external fun nSetPath(
        ptr: Long,
        pathPtr: Long,
        forceClosed: Boolean,
    )

    external fun nLength(ptr: Long): Float

    external fun nPosTan(
        ptr: Long,
        distance: Float,
        out: FloatArray,
    ): Boolean

    external fun nMatrix(
        ptr: Long,
        distance: Float,
        out: FloatArray,
    ): Boolean

    external fun nSegment(
        ptr: Long,
        startD: Float,
        stopD: Float,
        dstPtr: Long,
        startWithMoveTo: Boolean,
    ): Boolean

    external fun nIsClosed(ptr: Long): Boolean

    external fun nNextContour(ptr: Long): Boolean
}
