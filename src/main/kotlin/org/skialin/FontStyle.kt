package org.skialin

/** Weight, width, and slant of a typeface. Mirrors Skia's `SkFontStyle`. */
data class FontStyle(val weight: Int, val width: Int, val slant: Slant) {
    enum class Slant { UPRIGHT, ITALIC, OBLIQUE }

    companion object {
        val NORMAL = FontStyle(400, 5, Slant.UPRIGHT)
        val BOLD = FontStyle(700, 5, Slant.UPRIGHT)
        val ITALIC = FontStyle(400, 5, Slant.ITALIC)
        val BOLD_ITALIC = FontStyle(700, 5, Slant.ITALIC)
    }
}
