package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Path internal constructor(
    ptr: Long,
) : Managed(ptr, PathNative::nRelease) {
    val isEmpty: Boolean get() = PathNative.nIsEmpty(nativePtr)

    val bounds: Rect
        get() {
            val b = PathNative.nGetBounds(nativePtr)
            return Rect(b[0], b[1], b[2], b[3])
        }

    fun contains(point: Point): Boolean = PathNative.nContains(nativePtr, point.x, point.y)

    /** A path with the same non-overlapping-contour area as this one, with self-intersections removed. `null` on failure. */
    fun simplify(): Path? = PathNative.nSimplify(nativePtr).takeIf { it != 0L }?.let { Path(it) }

    val fillType: PathFillType get() = PathFillType.entries[PathNative.nFillType(nativePtr)]

    val isConvex: Boolean get() = PathNative.nIsConvex(nativePtr)

    val isOval: Rect?
        get() {
            val b = PathNative.nIsOval(nativePtr) ?: return null
            return Rect(b[0], b[1], b[2], b[3])
        }

    val isRRect: RRect?
        get() = PathNative.nIsRRect(nativePtr).takeIf { it != 0L }?.let { RRect(it) }

    fun computeTightBounds(): Rect {
        val b = PathNative.nComputeTightBounds(nativePtr)
        return Rect(b[0], b[1], b[2], b[3])
    }

    val pointsCount: Int get() = PathNative.nPointsCount(nativePtr)

    val points: Array<Point>
        get() {
            val flat = PathNative.nPoints(nativePtr)
            return Array(flat.size / 2) { i -> Point(flat[i * 2], flat[i * 2 + 1]) }
        }

    val generationId: Int get() = PathNative.nGenerationId(nativePtr)

    companion object {
        /** Combines [one] and [two] with the given boolean operation. `null` if the operation couldn't produce a result. */
        fun op(
            one: Path,
            two: Path,
            op: PathOp,
        ): Path? = PathNative.nOp(one.nativePtr, two.nativePtr, op.ordinal).takeIf { it != 0L }?.let { Path(it) }
    }
}

private object PathNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)

    external fun nIsEmpty(ptr: Long): Boolean

    external fun nGetBounds(ptr: Long): FloatArray

    external fun nContains(
        ptr: Long,
        x: Float,
        y: Float,
    ): Boolean

    external fun nOp(
        onePtr: Long,
        twoPtr: Long,
        op: Int,
    ): Long

    external fun nSimplify(ptr: Long): Long

    external fun nFillType(ptr: Long): Int

    external fun nIsConvex(ptr: Long): Boolean

    external fun nIsOval(ptr: Long): FloatArray?

    external fun nIsRRect(ptr: Long): Long

    external fun nComputeTightBounds(ptr: Long): FloatArray

    external fun nPointsCount(ptr: Long): Int

    external fun nPoints(ptr: Long): FloatArray

    external fun nGenerationId(ptr: Long): Int
}
