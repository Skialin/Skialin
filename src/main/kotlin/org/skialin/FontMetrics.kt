package org.skialin

/** Font-wide metrics scaled by text size. Mirrors Skia's `SkFontMetrics`. */
data class FontMetrics(
    val top: Float,
    val ascent: Float,
    val descent: Float,
    val bottom: Float,
    val leading: Float,
    val avgCharWidth: Float,
    val maxCharWidth: Float,
    val xMin: Float,
    val xMax: Float,
    val xHeight: Float,
    val capHeight: Float,
)
