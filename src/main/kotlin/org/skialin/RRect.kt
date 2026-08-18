package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** A rectangle with per-corner rounding. Mirrors Skia's `SkRRect`. */
class RRect internal constructor(ptr: Long) : Managed(ptr, RRectNative::nRelease) {
    enum class Type { EMPTY, RECT, OVAL, SIMPLE, NINE_PATCH, COMPLEX }

    fun cloneRRect(): RRect = RRect(RRectNative.nClone(nativePtr))

    val rect: Rect
        get() = RRectNative.nRect(nativePtr).let { Rect(it[0], it[1], it[2], it[3]) }

    /** The four corner radii, in `[upperLeft, upperRight, lowerRight, lowerLeft]` order. */
    val radii: Array<Point>
        get() = RRectNative.nRadii(nativePtr).let { r -> Array(4) { Point(r[it * 2], r[it * 2 + 1]) } }

    val type: Type get() = Type.entries[RRectNative.nType(nativePtr)]
    val isEmpty: Boolean get() = type == Type.EMPTY

    fun contains(point: Point): Boolean = RRectNative.nContainsPoint(nativePtr, point.x, point.y)
    fun contains(rect: Rect): Boolean = RRectNative.nContainsRect(nativePtr, floatArrayOf(rect.left, rect.top, rect.right, rect.bottom))
    val isValid: Boolean get() = RRectNative.nIsValid(nativePtr)

    fun inset(dx: Float, dy: Float): RRect = RRect(RRectNative.nInset(nativePtr, dx, dy))
    fun outset(dx: Float, dy: Float): RRect = RRect(RRectNative.nOutset(nativePtr, dx, dy))

    companion object {
        fun makeRect(rect: Rect): RRect = RRect(RRectNative.nMakeRect(floatArrayOf(rect.left, rect.top, rect.right, rect.bottom)))
        fun makeOval(oval: Rect): RRect = RRect(RRectNative.nMakeOval(floatArrayOf(oval.left, oval.top, oval.right, oval.bottom)))

        fun makeRectXY(rect: Rect, xRad: Float, yRad: Float): RRect =
            RRect(RRectNative.nMakeRectXY(floatArrayOf(rect.left, rect.top, rect.right, rect.bottom), xRad, yRad))

        /** [radii] is `[upperLeft, upperRight, lowerRight, lowerLeft]`. */
        fun makeRectRadii(rect: Rect, radii: Array<Point>): RRect {
            val flat = FloatArray(8)
            radii.forEachIndexed { i, p -> flat[i * 2] = p.x; flat[i * 2 + 1] = p.y }
            return RRect(RRectNative.nMakeRectRadii(floatArrayOf(rect.left, rect.top, rect.right, rect.bottom), flat))
        }
    }
}

private object RRectNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeRect(rect: FloatArray): Long
    external fun nMakeOval(oval: FloatArray): Long
    external fun nMakeRectXY(rect: FloatArray, xRad: Float, yRad: Float): Long
    external fun nMakeRectRadii(rect: FloatArray, radii: FloatArray): Long
    external fun nRelease(ptr: Long)
    external fun nClone(ptr: Long): Long
    external fun nRect(ptr: Long): FloatArray
    external fun nRadii(ptr: Long): FloatArray
    external fun nType(ptr: Long): Int
    external fun nContainsPoint(ptr: Long, x: Float, y: Float): Boolean
    external fun nContainsRect(ptr: Long, rect: FloatArray): Boolean
    external fun nIsValid(ptr: Long): Boolean
    external fun nInset(ptr: Long, dx: Float, dy: Float): Long
    external fun nOutset(ptr: Long, dx: Float, dy: Float): Long
}
