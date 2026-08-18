package org.skialin

/**
 * Reserves space in a paragraph for the caller to draw a custom inline object into.
 * Mirrors skparagraph's `PlaceholderStyle`.
 */
data class PlaceholderStyle(
    val width: Float,
    val height: Float,
    val alignment: Alignment = Alignment.BASELINE,
    val baseline: Baseline = Baseline.ALPHABETIC,
    val baselineOffset: Float = 0f,
) {
    enum class Alignment {
        /** Match the baseline of the placeholder with the text baseline. */
        BASELINE,

        /** The placeholder sits on top of the baseline. */
        ABOVE_BASELINE,

        /** The placeholder hangs below the baseline. */
        BELOW_BASELINE,

        /** Aligned with the top edge of the font. */
        TOP,

        /** Aligned with the bottom edge of the font. */
        BOTTOM,

        /** Centered within the line. */
        MIDDLE,
    }

    enum class Baseline { ALPHABETIC, IDEOGRAPHIC }
}
