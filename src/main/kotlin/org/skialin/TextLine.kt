package org.skialin

class TextLine private constructor(
    val glyphs: ShortArray,
    val positions: FloatArray,
    val width: Float,
    val textBlob: TextBlob?,
    metrics: FontMetrics,
) {
    val ascent: Float = -metrics.ascent
    val descent: Float = metrics.descent
    val leading: Float = metrics.leading
    val capHeight: Float = metrics.capHeight
    val xHeight: Float = metrics.xHeight
    val height: Float = descent - metrics.ascent + leading

    fun getCoordAtOffset(offset: Int): Float {
        if (offset <= 0 || positions.isEmpty()) return 0f
        if (offset >= positions.size) return width
        return positions[offset]
    }

    fun getOffsetAtCoord(x: Float): Int {
        if (positions.isEmpty() || x <= 0f) return 0
        val index = positions.indexOfFirst { it > x }
        return if (index == -1) positions.size else index
    }

    fun getLeftOffsetAtCoord(x: Float): Int = getOffsetAtCoord(x)

    companion object {
        fun make(
            text: String,
            font: Font,
        ): TextLine {
            val glyphs = font.textToGlyphs(text)
            val widths = font.widths(glyphs)
            val positions = FloatArray(glyphs.size)
            var x = 0f
            widths.forEachIndexed { i, w ->
                positions[i] = x
                x += w
            }
            val blob = TextBlob.makeFromText(text, font)
            return TextLine(glyphs, positions, x, blob, font.metrics())
        }
    }
}
