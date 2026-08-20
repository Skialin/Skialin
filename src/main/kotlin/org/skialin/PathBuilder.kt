package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class PathBuilder private constructor(ptr: Long) : Managed(ptr, PathBuilderNative::nRelease) {
    constructor() : this(PathBuilderNative.nMake())

    /** Seeds this builder with a copy of [path]'s fill type and verbs, so building can continue on top of it. */
    constructor(path: Path) : this(PathBuilderNative.nMakeFromPath(path.nativePtr))

    /** The fill type that will be baked into [Path]s produced by [snapshot]/[detach]. */
    var fillType: PathFillType
        get() = PathFillType.entries[PathBuilderNative.nFillType(nativePtr)]
        set(value) {
            PathBuilderNative.nSetFillType(nativePtr, value.ordinal)
        }

    fun moveTo(
        x: Float,
        y: Float,
    ): PathBuilder = apply { PathBuilderNative.nMoveTo(nativePtr, x, y) }

    fun lineTo(
        x: Float,
        y: Float,
    ): PathBuilder = apply { PathBuilderNative.nLineTo(nativePtr, x, y) }

    fun quadTo(
        x1: Float,
        y1: Float,
        x2: Float,
        y2: Float,
    ): PathBuilder = apply { PathBuilderNative.nQuadTo(nativePtr, x1, y1, x2, y2) }

    fun cubicTo(
        x1: Float,
        y1: Float,
        x2: Float,
        y2: Float,
        x3: Float,
        y3: Float,
    ): PathBuilder = apply { PathBuilderNative.nCubicTo(nativePtr, x1, y1, x2, y2, x3, y3) }

    fun closePath(): PathBuilder = apply { PathBuilderNative.nClose(nativePtr) }

    /** Appends an elliptical arc; mirrors `SkPathBuilder::arcTo(const SkRect&, float, float, bool)`. */
    fun arcTo(
        oval: Rect,
        startAngleDegrees: Float,
        sweepAngleDegrees: Float,
        forceMoveTo: Boolean,
    ): PathBuilder =
        apply {
            PathBuilderNative.nArcTo(nativePtr, oval.left, oval.top, oval.right, oval.bottom, startAngleDegrees, sweepAngleDegrees, forceMoveTo)
        }

    fun rMoveTo(
        dx: Float,
        dy: Float,
    ): PathBuilder = apply { PathBuilderNative.nRMoveTo(nativePtr, dx, dy) }

    fun rLineTo(
        dx: Float,
        dy: Float,
    ): PathBuilder = apply { PathBuilderNative.nRLineTo(nativePtr, dx, dy) }

    fun rQuadTo(
        dx1: Float,
        dy1: Float,
        dx2: Float,
        dy2: Float,
    ): PathBuilder = apply { PathBuilderNative.nRQuadTo(nativePtr, dx1, dy1, dx2, dy2) }

    fun rCubicTo(
        dx1: Float,
        dy1: Float,
        dx2: Float,
        dy2: Float,
        dx3: Float,
        dy3: Float,
    ): PathBuilder = apply { PathBuilderNative.nRCubicTo(nativePtr, dx1, dy1, dx2, dy2, dx3, dy3) }

    fun conicTo(
        x1: Float,
        y1: Float,
        x2: Float,
        y2: Float,
        w: Float,
    ): PathBuilder = apply { PathBuilderNative.nConicTo(nativePtr, x1, y1, x2, y2, w) }

    fun rConicTo(
        dx1: Float,
        dy1: Float,
        dx2: Float,
        dy2: Float,
        w: Float,
    ): PathBuilder = apply { PathBuilderNative.nRConicTo(nativePtr, dx1, dy1, dx2, dy2, w) }

    fun addRRect(
        rrect: RRect,
        direction: PathDirection = PathDirection.CLOCKWISE,
    ): PathBuilder = apply { PathBuilderNative.nAddRRect(nativePtr, rrect.nativePtr, direction.ordinal) }

    fun addPoly(
        points: Array<Point>,
        close: Boolean,
    ): PathBuilder {
        val flat = FloatArray(points.size * 2)
        points.forEachIndexed { i, p ->
            flat[i * 2] = p.x
            flat[i * 2 + 1] = p.y
        }
        PathBuilderNative.nAddPoly(nativePtr, flat, close)
        return this
    }

    fun addPath(
        src: Path,
        dx: Float = 0f,
        dy: Float = 0f,
        mode: PathAddMode = PathAddMode.APPEND,
    ): PathBuilder = apply { PathBuilderNative.nAddPath(nativePtr, src.nativePtr, dx, dy, mode.ordinal) }

    fun addPath(
        src: Path,
        matrix: Matrix33,
        mode: PathAddMode = PathAddMode.APPEND,
    ): PathBuilder = apply { PathBuilderNative.nAddPathMatrix(nativePtr, src.nativePtr, matrix.values, mode.ordinal) }

    fun transform(matrix: Matrix33): PathBuilder = apply { PathBuilderNative.nTransform(nativePtr, matrix.values) }

    fun setLastPt(
        x: Float,
        y: Float,
    ): PathBuilder = apply { PathBuilderNative.nSetLastPt(nativePtr, x, y) }

    fun reset(): PathBuilder = apply { PathBuilderNative.nReset(nativePtr) }

    fun addRect(
        rect: Rect,
        direction: PathDirection = PathDirection.CLOCKWISE,
    ): PathBuilder =
        apply {
            PathBuilderNative.nAddRect(nativePtr, rect.left, rect.top, rect.right, rect.bottom, direction.ordinal)
        }

    fun addOval(
        oval: Rect,
        direction: PathDirection = PathDirection.CLOCKWISE,
    ): PathBuilder =
        apply {
            PathBuilderNative.nAddOval(nativePtr, oval.left, oval.top, oval.right, oval.bottom, direction.ordinal)
        }

    fun addCircle(
        center: Point,
        radius: Float,
        direction: PathDirection = PathDirection.CLOCKWISE,
    ): PathBuilder = apply { PathBuilderNative.nAddCircle(nativePtr, center.x, center.y, radius, direction.ordinal) }

    fun offset(
        dx: Float,
        dy: Float,
    ): PathBuilder = apply { PathBuilderNative.nOffset(nativePtr, dx, dy) }

    val isEmpty: Boolean get() = PathBuilderNative.nIsEmpty(nativePtr)

    fun snapshot(): Path = Path(PathBuilderNative.nSnapshot(nativePtr))

    fun detach(): Path = Path(PathBuilderNative.nDetach(nativePtr))
}

private object PathBuilderNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(): Long

    external fun nMakeFromPath(pathPtr: Long): Long

    external fun nRelease(ptr: Long)

    external fun nSetFillType(
        ptr: Long,
        fillType: Int,
    )

    external fun nFillType(ptr: Long): Int

    external fun nArcTo(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        startAngleDeg: Float,
        sweepAngleDeg: Float,
        forceMoveTo: Boolean,
    )

    external fun nMoveTo(
        ptr: Long,
        x: Float,
        y: Float,
    )

    external fun nLineTo(
        ptr: Long,
        x: Float,
        y: Float,
    )

    external fun nQuadTo(
        ptr: Long,
        x1: Float,
        y1: Float,
        x2: Float,
        y2: Float,
    )

    external fun nCubicTo(
        ptr: Long,
        x1: Float,
        y1: Float,
        x2: Float,
        y2: Float,
        x3: Float,
        y3: Float,
    )

    external fun nClose(ptr: Long)

    external fun nRMoveTo(
        ptr: Long,
        dx: Float,
        dy: Float,
    )

    external fun nRLineTo(
        ptr: Long,
        dx: Float,
        dy: Float,
    )

    external fun nRQuadTo(
        ptr: Long,
        dx1: Float,
        dy1: Float,
        dx2: Float,
        dy2: Float,
    )

    external fun nRCubicTo(
        ptr: Long,
        dx1: Float,
        dy1: Float,
        dx2: Float,
        dy2: Float,
        dx3: Float,
        dy3: Float,
    )

    external fun nConicTo(
        ptr: Long,
        x1: Float,
        y1: Float,
        x2: Float,
        y2: Float,
        w: Float,
    )

    external fun nRConicTo(
        ptr: Long,
        dx1: Float,
        dy1: Float,
        dx2: Float,
        dy2: Float,
        w: Float,
    )

    external fun nAddRRect(
        ptr: Long,
        rrectPtr: Long,
        direction: Int,
    )

    external fun nAddPoly(
        ptr: Long,
        points: FloatArray,
        close: Boolean,
    )

    external fun nAddPath(
        ptr: Long,
        srcPtr: Long,
        dx: Float,
        dy: Float,
        mode: Int,
    )

    external fun nAddPathMatrix(
        ptr: Long,
        srcPtr: Long,
        matrix: FloatArray,
        mode: Int,
    )

    external fun nTransform(
        ptr: Long,
        matrix: FloatArray,
    )

    external fun nSetLastPt(
        ptr: Long,
        x: Float,
        y: Float,
    )

    external fun nReset(ptr: Long)

    external fun nAddRect(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        direction: Int,
    )

    external fun nAddOval(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        direction: Int,
    )

    external fun nAddCircle(
        ptr: Long,
        cx: Float,
        cy: Float,
        radius: Float,
        direction: Int,
    )

    external fun nOffset(
        ptr: Long,
        dx: Float,
        dy: Float,
    )

    external fun nIsEmpty(ptr: Long): Boolean

    external fun nSnapshot(ptr: Long): Long

    external fun nDetach(ptr: Long): Long
}
