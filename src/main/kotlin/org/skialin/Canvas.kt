package org.skialin

import org.skialin.impl.NativeLoader

class Canvas internal constructor(internal val ptr: Long) {
    fun clear(color: Color) = CanvasNative.nClear(ptr, color)

    fun drawColor(color: Color, mode: BlendMode = BlendMode.SRC_OVER) =
        CanvasNative.nDrawColor(ptr, color, mode.ordinal)

    fun drawPaint(paint: Paint) = CanvasNative.nDrawPaint(ptr, paint.nativePtr)

    fun drawLine(p0: Point, p1: Point, paint: Paint) =
        CanvasNative.nDrawLine(ptr, p0.x, p0.y, p1.x, p1.y, paint.nativePtr)

    fun drawRect(rect: Rect, paint: Paint) =
        CanvasNative.nDrawRect(ptr, rect.left, rect.top, rect.right, rect.bottom, paint.nativePtr)

    fun drawOval(rect: Rect, paint: Paint) =
        CanvasNative.nDrawOval(ptr, rect.left, rect.top, rect.right, rect.bottom, paint.nativePtr)

    fun drawCircle(center: Point, radius: Float, paint: Paint) =
        CanvasNative.nDrawCircle(ptr, center.x, center.y, radius, paint.nativePtr)

    fun drawPath(path: Path, paint: Paint) = CanvasNative.nDrawPath(ptr, path.nativePtr, paint.nativePtr)

    fun drawTextBlob(blob: TextBlob, x: Float, y: Float, paint: Paint) =
        CanvasNative.nDrawTextBlob(ptr, blob.nativePtr, x, y, paint.nativePtr)

    fun save(): Int = CanvasNative.nSave(ptr)
    fun restore() = CanvasNative.nRestore(ptr)
    fun restoreToCount(saveCount: Int) = CanvasNative.nRestoreToCount(ptr, saveCount)

    fun translate(dx: Float, dy: Float) = CanvasNative.nTranslate(ptr, dx, dy)
    fun scale(sx: Float, sy: Float) = CanvasNative.nScale(ptr, sx, sy)
    fun rotate(degrees: Float) = CanvasNative.nRotate(ptr, degrees)

    fun clipRect(rect: Rect, op: ClipOp = ClipOp.INTERSECT) =
        CanvasNative.nClipRect(ptr, rect.left, rect.top, rect.right, rect.bottom, op.ordinal)

    fun clipPath(path: Path, op: ClipOp = ClipOp.INTERSECT) =
        CanvasNative.nClipPath(ptr, path.nativePtr, op.ordinal)

    /** Runs [block] between [save] and [restore]. */
    inline fun withSave(block: Canvas.() -> Unit) {
        save()
        try {
            block()
        } finally {
            restore()
        }
    }
}

private object CanvasNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nClear(ptr: Long, color: Int)
    external fun nDrawColor(ptr: Long, color: Int, mode: Int)
    external fun nDrawPaint(ptr: Long, paintPtr: Long)
    external fun nDrawLine(ptr: Long, x0: Float, y0: Float, x1: Float, y1: Float, paintPtr: Long)
    external fun nDrawRect(ptr: Long, left: Float, top: Float, right: Float, bottom: Float, paintPtr: Long)
    external fun nDrawOval(ptr: Long, left: Float, top: Float, right: Float, bottom: Float, paintPtr: Long)
    external fun nDrawCircle(ptr: Long, cx: Float, cy: Float, radius: Float, paintPtr: Long)
    external fun nDrawPath(ptr: Long, pathPtr: Long, paintPtr: Long)
    external fun nDrawTextBlob(ptr: Long, blobPtr: Long, x: Float, y: Float, paintPtr: Long)
    external fun nSave(ptr: Long): Int
    external fun nRestore(ptr: Long)
    external fun nRestoreToCount(ptr: Long, saveCount: Int)
    external fun nTranslate(ptr: Long, dx: Float, dy: Float)
    external fun nScale(ptr: Long, sx: Float, sy: Float)
    external fun nRotate(ptr: Long, degrees: Float)
    external fun nClipRect(ptr: Long, left: Float, top: Float, right: Float, bottom: Float, op: Int)
    external fun nClipPath(ptr: Long, pathPtr: Long, op: Int)
}
