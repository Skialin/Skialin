package org.skialin

/** Mirrors Skia's `SkSamplingOptions`. A plain value type, never native-backed. */
data class SamplingOptions(
    val maxAniso: Int = 0,
    val cubicB: Float? = null,
    val cubicC: Float? = null,
    val filter: FilterMode = FilterMode.NEAREST,
    val mipmap: MipmapMode = MipmapMode.NONE,
) {
    val useCubic: Boolean get() = cubicB != null && cubicC != null
    val isAniso: Boolean get() = maxAniso != 0

    companion object {
        val NEAREST = SamplingOptions(filter = FilterMode.NEAREST, mipmap = MipmapMode.NONE)
        val LINEAR = SamplingOptions(filter = FilterMode.LINEAR, mipmap = MipmapMode.NONE)
        val MITCHELL = SamplingOptions(cubicB = 1f / 3f, cubicC = 1f / 3f)
        val CATMULL_ROM = SamplingOptions(cubicB = 0f, cubicC = 1f / 2f)

        fun make(filter: FilterMode, mipmap: MipmapMode = MipmapMode.NONE) = SamplingOptions(filter = filter, mipmap = mipmap)
        fun cubic(b: Float, c: Float) = SamplingOptions(cubicB = b, cubicC = c)
        fun aniso(maxAniso: Int) = SamplingOptions(maxAniso = maxAniso.coerceAtLeast(1))
    }
}
