package org.skialin

data class IRect(
    val left: Int,
    val top: Int,
    val right: Int,
    val bottom: Int,
) {
    val width: Int get() = right - left
    val height: Int get() = bottom - top

    companion object {
        fun makeXYWH(
            left: Int,
            top: Int,
            width: Int,
            height: Int,
        ): IRect = IRect(left, top, left + width, top + height)
    }
}
