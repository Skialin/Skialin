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
}
