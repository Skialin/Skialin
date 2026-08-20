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

    /**
     * Walks this path's verbs (move/line/quad/conic/cubic/close), mirroring `SkPath::Iter`.
     *
     * When [convertConicsToQuads] is true, conic segments are approximated with one or more
     * [PathVerb.QUAD] segments instead of being reported as [PathVerb.CONIC]; [tolerance] controls
     * the maximum deviation of the approximation from the true conic.
     */
    fun segments(
        convertConicsToQuads: Boolean = false,
        tolerance: Float = 0.25f,
    ): List<PathSegment> {
        val flat = PathNative.nSegments(nativePtr, convertConicsToQuads, tolerance)
        val result = ArrayList<PathSegment>(flat.size / 10)
        var i = 0
        while (i < flat.size) {
            val verb = PathVerb.entries[flat[i].toInt()]
            val points =
                arrayOf(
                    Point(flat[i + 1], flat[i + 2]),
                    Point(flat[i + 3], flat[i + 4]),
                    Point(flat[i + 5], flat[i + 6]),
                    Point(flat[i + 7], flat[i + 8]),
                )
            val conicWeight = flat[i + 9]
            result.add(PathSegment(verb, points, conicWeight))
            i += 10
        }
        return result
    }

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

    external fun nSegments(
        ptr: Long,
        convertConicsToQuads: Boolean,
        tolerance: Float,
    ): FloatArray
}

/** A single verb in a [Path], as produced by [Path.segments]. Mirrors `SkPath::Verb`. */
enum class PathVerb { MOVE, LINE, QUAD, CONIC, CUBIC, CLOSE }

/**
 * One segment of a [Path], as produced by [Path.segments]. The meaningful prefix of [points]
 * depends on [verb]: MOVE=1, LINE=2, QUAD=3, CONIC=3, CUBIC=4, CLOSE=0. [conicWeight] is only
 * meaningful when [verb] is [PathVerb.CONIC].
 */
data class PathSegment(
    val verb: PathVerb,
    val points: Array<Point>,
    val conicWeight: Float,
) {
    override fun equals(other: Any?): Boolean =
        other is PathSegment && verb == other.verb && points.contentEquals(other.points) && conicWeight == other.conicWeight

    override fun hashCode(): Int = 31 * (31 * verb.hashCode() + points.contentHashCode()) + conicWeight.hashCode()
}
