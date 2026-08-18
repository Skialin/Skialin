package org.skialin

data class Rect(val left: Float, val top: Float, val right: Float, val bottom: Float) {
    val width: Float get() = right - left
    val height: Float get() = bottom - top

    companion object {
        fun makeWH(width: Float, height: Float): Rect = Rect(0f, 0f, width, height)
    }
}