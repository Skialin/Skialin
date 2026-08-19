package org.skialin

import org.skialin.impl.NativeLoader

/**
 * Records [Canvas] draw calls as SVG XML. Mirrors Skia's `SkSVGCanvas`. Call
 * [canvas] to draw into it, then [finish] to flush and obtain the recorded
 * SVG XML, which also releases the native canvas.
 */
class SVGCanvas(
    bounds: Rect,
    convertTextToPaths: Boolean = false,
    noPrettyXml: Boolean = false,
    relativePathEncoding: Boolean = false,
) : AutoCloseable {
    private var ptr: Long =
        SVGCanvasNative.nMake(bounds.left, bounds.top, bounds.right, bounds.bottom, convertTextToPaths, noPrettyXml, relativePathEncoding)

    fun canvas(): Canvas {
        check(ptr != 0L) { "SVGCanvas already finished" }
        return Canvas(SVGCanvasNative.nGetCanvas(ptr))
    }

    /** Flushes and returns the recorded SVG XML, releasing the native canvas. */
    fun finish(): ByteArray {
        check(ptr != 0L) { "SVGCanvas already finished" }
        val bytes = SVGCanvasNative.nFinish(ptr)
        ptr = 0L
        return bytes
    }

    override fun close() {
        if (ptr != 0L) finish()
    }
}

private object SVGCanvasNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        convertTextToPaths: Boolean,
        noPrettyXml: Boolean,
        relativePathEncoding: Boolean,
    ): Long

    external fun nGetCanvas(ptr: Long): Long

    external fun nFinish(ptr: Long): ByteArray
}
