package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Paragraph internal constructor(
    ptr: Long,
) : Managed(ptr, ParagraphNative::nRelease) {
    enum class Affinity { UPSTREAM, DOWNSTREAM }

    data class GlyphPosition(
        val position: Int,
        val affinity: Affinity,
    )

    data class GlyphInfo(
        val bounds: Rect,
        val graphemeClusterRange: LongRange,
        val direction: ParagraphStyle.TextDirection,
        val isEllipsis: Boolean,
    )

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

    val unresolvedGlyphs: Int?
        get() = ParagraphNative.nUnresolvedGlyphs(nativePtr).takeIf { it >= 0 }

    /** The codepoints skparagraph could not resolve to a glyph during shaping - the actual
     * characters behind [unresolvedGlyphs]'s count, for font-fallback-registry lookups. */
    val unresolvedCodepoints: IntArray
        get() = ParagraphNative.nUnresolvedCodepoints(nativePtr)

    /** Invalidates cached layout state so the next [layout] call redoes shaping/positioning
     * instead of being a no-op for an unchanged width. */
    fun markDirty() = ParagraphNative.nMarkDirty(nativePtr)

    fun glyphPositionAtCoordinate(
        dx: Float,
        dy: Float,
    ): GlyphPosition {
        val packed = ParagraphNative.nGlyphPositionAtCoordinate(nativePtr, dx, dy)
        return GlyphPosition((packed shr 32).toInt(), if ((packed and 1) == 0L) Affinity.UPSTREAM else Affinity.DOWNSTREAM)
    }

    fun wordBoundary(offset: Int): LongRange {
        val range = ParagraphNative.nWordBoundary(nativePtr, offset)
        return range[0] until range[1]
    }

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

    fun lineMetrics(): List<LineMetrics> = (0 until lineNumber.toInt()).mapNotNull { lineMetricsAt(it) }

    private fun flatToTextBoxes(flat: FloatArray): List<TextBox> =
        (0 until flat.size / 5).map { i ->
            val direction = if (flat[i * 5 + 4] > 0.5f) ParagraphStyle.TextDirection.LTR else ParagraphStyle.TextDirection.RTL
            TextBox(Rect(flat[i * 5], flat[i * 5 + 1], flat[i * 5 + 2], flat[i * 5 + 3]), direction)
        }

    fun getRectsForRange(
        start: Int,
        end: Int,
        heightStyle: RectHeightStyle = RectHeightStyle.TIGHT,
        widthStyle: RectWidthStyle = RectWidthStyle.TIGHT,
    ): List<TextBox> = flatToTextBoxes(ParagraphNative.nGetRectsForRange(nativePtr, start, end, heightStyle.ordinal, widthStyle.ordinal))

    fun getRectsForPlaceholders(): List<TextBox> = flatToTextBoxes(ParagraphNative.nGetRectsForPlaceholders(nativePtr))

    private fun readGlyphInfo(out: DoubleArray): GlyphInfo =
        GlyphInfo(
            Rect(out[0].toFloat(), out[1].toFloat(), out[2].toFloat(), out[3].toFloat()),
            out[4].toLong() until out[5].toLong(),
            if (out[6] > 0.5) ParagraphStyle.TextDirection.LTR else ParagraphStyle.TextDirection.RTL,
            out[7] > 0.5,
        )

    fun glyphInfoAt(codeUnitIndex: Long): GlyphInfo? {
        val out = DoubleArray(8)
        if (!ParagraphNative.nGlyphInfoAtUTF16Offset(nativePtr, codeUnitIndex, out)) return null
        return readGlyphInfo(out)
    }

    fun closestGlyphInfoAt(
        dx: Float,
        dy: Float,
    ): GlyphInfo? {
        val out = DoubleArray(8)
        if (!ParagraphNative.nClosestGlyphInfoAt(nativePtr, dx, dy, out)) return null
        return readGlyphInfo(out)
    }

    fun updateFontSize(
        from: Long,
        to: Long,
        fontSize: Float,
    ) = ParagraphNative.nUpdateFontSize(nativePtr, from, to, fontSize)

    fun updateForegroundPaint(
        from: Long,
        to: Long,
        paint: Paint,
    ) = ParagraphNative.nUpdateForegroundPaint(nativePtr, from, to, paint.nativePtr)

    fun updateBackgroundPaint(
        from: Long,
        to: Long,
        paint: Paint,
    ) = ParagraphNative.nUpdateBackgroundPaint(nativePtr, from, to, paint.nativePtr)
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

    external fun nUnresolvedCodepoints(ptr: Long): IntArray

    external fun nMarkDirty(ptr: Long)

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

    external fun nGetRectsForRange(
        ptr: Long,
        start: Int,
        end: Int,
        heightStyle: Int,
        widthStyle: Int,
    ): FloatArray

    external fun nGetRectsForPlaceholders(ptr: Long): FloatArray

    external fun nGlyphInfoAtUTF16Offset(
        ptr: Long,
        codeUnitIndex: Long,
        out: DoubleArray,
    ): Boolean

    external fun nClosestGlyphInfoAt(
        ptr: Long,
        dx: Float,
        dy: Float,
        out: DoubleArray,
    ): Boolean

    external fun nUpdateFontSize(
        ptr: Long,
        from: Long,
        to: Long,
        fontSize: Float,
    )

    external fun nUpdateForegroundPaint(
        ptr: Long,
        from: Long,
        to: Long,
        paintPtr: Long,
    )

    external fun nUpdateBackgroundPaint(
        ptr: Long,
        from: Long,
        to: Long,
        paintPtr: Long,
    )
}
