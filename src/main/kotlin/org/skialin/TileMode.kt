package org.skialin

enum class TileMode {
    /** Replicates the edge color if the shader draws outside of its original bounds. */
    CLAMP,

    /** Repeats the shader's image horizontally and vertically. */
    REPEAT,

    /** Repeats the shader's image, alternating mirror images so adjacent images always seam. */
    MIRROR,

    /** Only draws within the original domain; transparent-black everywhere else. */
    DECAL,
}
