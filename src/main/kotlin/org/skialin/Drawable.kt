package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/**
 * A custom, caller-implemented draw command Skia can defer and replay later
 * (e.g. when recorded into a [Picture] or a GPU-recorded scene). Mirrors
 * Skia's `SkDrawable`. Override [onDraw] and [onGetBounds]; Skia calls them
 * back whenever it needs to (re)draw or measure this drawable, which may
 * happen on a different thread than the one that created it.
 */
abstract class Drawable : Managed(DrawableNative.nMake(), DrawableNative::nRelease) {
    init {
        DrawableNative.nBindCallback(nativePtr, this)
    }

    protected abstract fun onDraw(canvas: Canvas)

    protected abstract fun onGetBounds(): Rect

    private fun onDrawNative(canvasPtr: Long) = onDraw(Canvas(canvasPtr))

    private fun onGetBoundsNative(): FloatArray = onGetBounds().let { floatArrayOf(it.left, it.top, it.right, it.bottom) }

    fun makePictureSnapshot(): Picture? = DrawableNative.nMakePictureSnapshot(nativePtr).takeIf { it != 0L }?.let { Picture(it) }

    val bounds: Rect
        get() {
            val b = DrawableNative.nBounds(nativePtr)
            return Rect(b[0], b[1], b[2], b[3])
        }

    val generationId: Int get() = DrawableNative.nGenerationId(nativePtr)

    fun notifyDrawingChanged() = DrawableNative.nNotifyDrawingChanged(nativePtr)
}

private object DrawableNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(): Long

    external fun nRelease(ptr: Long)

    external fun nBindCallback(
        ptr: Long,
        self: Drawable,
    )

    external fun nMakePictureSnapshot(ptr: Long): Long

    external fun nBounds(ptr: Long): FloatArray

    external fun nGenerationId(ptr: Long): Int

    external fun nNotifyDrawingChanged(ptr: Long)
}
