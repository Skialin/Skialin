package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class MaskFilter internal constructor(
    ptr: Long,
) : Managed(ptr, MaskFilterNative::nRelease) {
    enum class BlurStyle { NORMAL, SOLID, OUTER, INNER }

    companion object {
        fun makeBlur(
            style: BlurStyle,
            sigma: Float,
            respectCtm: Boolean = true,
        ): MaskFilter? = MaskFilterNative.nBlur(style.ordinal, sigma, respectCtm).takeIf { it != 0L }?.let { MaskFilter(it) }

        private const val BLUR_SIGMA_SCALE = 0.57735f

        fun convertRadiusToSigma(radius: Float): Float = if (radius > 0) BLUR_SIGMA_SCALE * radius + 0.5f else 0f

        fun convertSigmaToRadius(sigma: Float): Float = if (sigma > 0.5f) (sigma - 0.5f) / BLUR_SIGMA_SCALE else 0f
    }
}

private object MaskFilterNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)

    external fun nBlur(
        style: Int,
        sigma: Float,
        respectCtm: Boolean,
    ): Long
}
