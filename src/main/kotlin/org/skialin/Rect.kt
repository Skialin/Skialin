package org.skialin

data class Rect(
    val left: Float,
    val top: Float,
    val right: Float,
    val bottom: Float,
) {
    val width: Float get() = right - left
    val height: Float get() = bottom - top
    val isEmpty: Boolean get() = !(left < right && top < bottom)

    companion object {
        fun makeWH(
            width: Float,
            height: Float,
        ): Rect = Rect(0f, 0f, width, height)

        fun makeXYWH(
            left: Float,
            top: Float,
            width: Float,
            height: Float,
        ): Rect = Rect(left, top, left + width, top + height)

        fun makeLTRB(
            left: Float,
            top: Float,
            right: Float,
            bottom: Float,
        ): Rect = Rect(left, top, right, bottom)
    }
}
