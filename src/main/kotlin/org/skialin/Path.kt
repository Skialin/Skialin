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
}
