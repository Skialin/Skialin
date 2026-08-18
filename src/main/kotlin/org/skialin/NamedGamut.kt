package org.skialin

/** A row-major 3x3 matrix to XYZ D50. */
object NamedGamut {
    val SRGB = floatArrayOf(
        0.436065674f, 0.385147095f, 0.143066406f,
        0.222488403f, 0.716873169f, 0.060607910f,
        0.013916016f, 0.097076416f, 0.714096069f,
    )
    val ADOBE_RGB = floatArrayOf(
        0.60974f, 0.20528f, 0.14919f,
        0.31111f, 0.62567f, 0.06322f,
        0.01947f, 0.06087f, 0.74457f,
    )
    val DISPLAY_P3 = floatArrayOf(
        0.515102f, 0.291965f, 0.157153f,
        0.241182f, 0.692236f, 0.0665819f,
        -0.00104941f, 0.0418818f, 0.784378f,
    )
    val REC2020 = floatArrayOf(
        0.673459f, 0.165661f, 0.125100f,
        0.279033f, 0.675338f, 0.0456288f,
        -0.00193139f, 0.0299794f, 0.797162f,
    )
    val XYZ = floatArrayOf(
        1f, 0f, 0f,
        0f, 1f, 0f,
        0f, 0f, 1f,
    )
}
