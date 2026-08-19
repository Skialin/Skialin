package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class PathBuilder : Managed(PathBuilderNative.nMake(), PathBuilderNative::nRelease) {
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

    external fun nRelease(ptr: Long)

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
