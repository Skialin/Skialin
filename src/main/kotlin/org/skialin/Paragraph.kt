package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Laid-out, paintable text. Mirrors skparagraph's `Paragraph`. */
class Paragraph internal constructor(
    ptr: Long,
) : Managed(ptr, ParagraphNative::nRelease) {
    enum class Affinity { UPSTREAM, DOWNSTREAM }

    data class GlyphPosition(
        val position: Int,
        val affinity: Affinity,
    )

    /** Line layout metrics. Mirrors skparagraph's `LineMetrics`, excluding its per-run font metrics map. */
    data class LineMetrics(
        val startIndex: Long,
        val endIndex: Long,
        val endExcludingWhitespaces: Long,
        val endIncludingNewline: Long,
        val hardBreak: Boolean,
        val ascent: Double,
        val descent: Double,
        val unscaledAscent: Double,
        val height: Double,
        val width: Double,
        val left: Double,
        val baseline: Double,
    )

    fun layout(width: Float) = ParagraphNative.nLayout(nativePtr, width)

    fun paint(
        canvas: Canvas,
        x: Float,
        y: Float,
    ) = ParagraphNative.nPaint(nativePtr, canvas.ptr, x, y)

    val maxWidth: Float get() = ParagraphNative.nMaxWidth(nativePtr)
    val height: Float get() = ParagraphNative.nHeight(nativePtr)
    val minIntrinsicWidth: Float get() = ParagraphNative.nMinIntrinsicWidth(nativePtr)
    val maxIntrinsicWidth: Float get() = ParagraphNative.nMaxIntrinsicWidth(nativePtr)
    val alphabeticBaseline: Float get() = ParagraphNative.nAlphabeticBaseline(nativePtr)
    val ideographicBaseline: Float get() = ParagraphNative.nIdeographicBaseline(nativePtr)
    val longestLine: Float get() = ParagraphNative.nLongestLine(nativePtr)
    val didExceedMaxLines: Boolean get() = ParagraphNative.nDidExceedMaxLines(nativePtr)
    val lineNumber: Long get() = ParagraphNative.nLineNumber(nativePtr)

    /** The number of unresolved glyphs, or `null` if the paragraph hasn't been shaped yet (i.e. before [layout]). */
    val unresolvedGlyphs: Int?
        get() = ParagraphNative.nUnresolvedGlyphs(nativePtr).takeIf { it >= 0 }

    /** The glyph at the given coordinate, with the paragraph's top-left as the origin and +y as down. */
    fun glyphPositionAtCoordinate(
        dx: Float,
        dy: Float,
    ): GlyphPosition {
        val packed = ParagraphNative.nGlyphPositionAtCoordinate(nativePtr, dx, dy)
        return GlyphPosition((packed shr 32).toInt(), if ((packed and 1) == 0L) Affinity.UPSTREAM else Affinity.DOWNSTREAM)
    }

    /** The `[start, end)` range of the word containing the glyph at [offset]. */
    fun wordBoundary(offset: Int): LongRange {
        val range = ParagraphNative.nWordBoundary(nativePtr, offset)
        return range[0] until range[1]
    }

    /** Layout metrics for line [lineNumber] (0-indexed), or `null` if out of range. */
    fun lineMetricsAt(lineNumber: Int): LineMetrics? {
        val out = DoubleArray(12)
        if (!ParagraphNative.nLineMetricsAt(nativePtr, lineNumber, out)) return null
        return LineMetrics(
            out[0].toLong(),
            out[1].toLong(),
            out[2].toLong(),
            out[3].toLong(),
            out[4] != 0.0,
            out[5],
            out[6],
            out[7],
            out[8],
            out[9],
            out[10],
            out[11],
        )
    }

    /** Layout metrics for every line, in order. */
    fun lineMetrics(): List<LineMetrics> = (0 until lineNumber.toInt()).mapNotNull { lineMetricsAt(it) }
}

private object ParagraphNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)

    external fun nLayout(
        ptr: Long,
        width: Float,
    )

    external fun nPaint(
        ptr: Long,
        canvasPtr: Long,
        x: Float,
        y: Float,
    )

    external fun nMaxWidth(ptr: Long): Float

    external fun nHeight(ptr: Long): Float

    external fun nMinIntrinsicWidth(ptr: Long): Float

    external fun nMaxIntrinsicWidth(ptr: Long): Float

    external fun nAlphabeticBaseline(ptr: Long): Float

    external fun nIdeographicBaseline(ptr: Long): Float

    external fun nLongestLine(ptr: Long): Float

    external fun nDidExceedMaxLines(ptr: Long): Boolean

    external fun nLineNumber(ptr: Long): Long

    external fun nUnresolvedGlyphs(ptr: Long): Int

    external fun nGlyphPositionAtCoordinate(
        ptr: Long,
        dx: Float,
        dy: Float,
    ): Long

    external fun nWordBoundary(
        ptr: Long,
        offset: Int,
    ): LongArray

    external fun nLineMetricsAt(
        ptr: Long,
        lineNumber: Int,
        out: DoubleArray,
    ): Boolean
}
