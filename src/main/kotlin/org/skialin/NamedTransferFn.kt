package org.skialin

/** The 7 coefficients (g, a, b, c, d, e, f) of Skia's piecewise transfer function. */
object NamedTransferFn {
    val SRGB = floatArrayOf(2.4f, (1 / 1.055).toFloat(), (0.055 / 1.055).toFloat(), (1 / 12.92).toFloat(), 0.04045f, 0f, 0f)
    val TWO_DOT_TWO = floatArrayOf(2.2f, 1f, 0f, 0f, 0f, 0f, 0f)
    val LINEAR = floatArrayOf(1f, 1f, 0f, 0f, 0f, 0f, 0f)
    val REC2020 = floatArrayOf(2.22222f, 0.909672f, 0.0903276f, 0.222222f, 0.0812429f, 0f, 0f)
    val PQ = floatArrayOf(-5.0f, 203f, 0f, 0f, 0f, 0f, 0f)
    val HLG = floatArrayOf(-6.0f, 203f, 1000f, 1.2f, 0f, 0f, 0f)
}
