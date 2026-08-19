package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** A parsed SVG document. Mirrors Skia's `SkSVGDOM`. */
class SVGDOM private constructor(
    ptr: Long,
) : Managed(ptr, SVGDOMNative::nRelease) {
    /** The viewport used to resolve the root's width/height when specified in relative units. */
    var containerSize: Pair<Float, Float>
        get() = SVGDOMNative.nGetContainerSize(nativePtr).let { it[0] to it[1] }
        set(value) = SVGDOMNative.nSetContainerSize(nativePtr, value.first, value.second)

    fun render(canvas: Canvas) = SVGDOMNative.nRender(nativePtr, canvas.ptr)

    companion object {
        /** Parses [bytes] as SVG XML. `null` if it doesn't parse. */
        fun makeFromBytes(bytes: ByteArray): SVGDOM? = SVGDOMNative.nMakeFromBytes(bytes).takeIf { it != 0L }?.let { SVGDOM(it) }
    }
}

private object SVGDOMNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeFromBytes(bytes: ByteArray): Long

    external fun nRelease(ptr: Long)

    external fun nSetContainerSize(
        ptr: Long,
        width: Float,
        height: Float,
    )

    external fun nGetContainerSize(ptr: Long): FloatArray

    external fun nRender(
        ptr: Long,
        canvasPtr: Long,
    )
}
