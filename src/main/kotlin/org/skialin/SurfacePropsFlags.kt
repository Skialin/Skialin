package org.skialin

/** Matches SkSurfaceProps::Flags. */
object SurfacePropsFlags {
    const val DEFAULT = 0
    const val USE_DEVICE_INDEPENDENT_FONTS = 1 shl 0
    const val DYNAMIC_MSAA = 1 shl 1
    const val ALWAYS_DITHER = 1 shl 2
    const val PRESERVES_TRANSPARENT_DRAWS = 1 shl 3
}
