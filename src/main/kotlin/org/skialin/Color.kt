package org.skialin

typealias Color = Int

object Colors {
    fun argb(
        a: Int,
        r: Int,
        g: Int,
        b: Int,
    ): Color = (a shl 24) or (r shl 16) or (g shl 8) or b

    fun rgb(
        r: Int,
        g: Int,
        b: Int,
    ): Color = argb(0xff, r, g, b)

    val BLACK: Color = rgb(0, 0, 0)
    val WHITE: Color = rgb(0xff, 0xff, 0xff)
    val TRANSPARENT: Color = argb(0, 0, 0, 0)
    val RED: Color = rgb(0xff, 0, 0)
    val GREEN: Color = rgb(0, 0xff, 0)
    val BLUE: Color = rgb(0, 0, 0xff)
}
