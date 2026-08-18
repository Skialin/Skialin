package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Transforms the alpha mask before it's drawn (e.g. a blur). Mirrors Skia's `SkMaskFilter`. */
class MaskFilter internal constructor(ptr: Long) : Managed(ptr, MaskFilterNative::nRelease) {
    enum class BlurStyle { NORMAL, SOLID, OUTER, INNER }

    companion object {
        fun makeBlur(style: BlurStyle, sigma: Float, respectCtm: Boolean = true): MaskFilter? =
            MaskFilterNative.nBlur(style.ordinal, sigma, respectCtm).takeIf { it != 0L }?.let { MaskFilter(it) }
    }
}

private object MaskFilterNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)
    external fun nBlur(style: Int, sigma: Float, respectCtm: Boolean): Long
}
