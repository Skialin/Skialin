package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class RenderNodeContext(
    measureDrawBounds: Boolean = false,
    snapshotCache: Boolean = true,
) : Managed(RenderNodeContextNative.nMake(measureDrawBounds, snapshotCache), RenderNodeContextNative::nRelease) {
    fun setLightingInfo(
        centerX: Float = 0f,
        centerY: Float = 0f,
        centerZ: Float = 0f,
        radius: Float = 0f,
        ambientShadowAlpha: Float = 0f,
        spotShadowAlpha: Float = 0f,
    ) {
        RenderNodeContextNative.nSetLightingInfo(nativePtr, centerX, centerY, centerZ, radius, ambientShadowAlpha, spotShadowAlpha)
    }
}

private object RenderNodeContextNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(
        measureDrawBounds: Boolean,
        snapshotCache: Boolean,
    ): Long

    external fun nRelease(ptr: Long)

    external fun nSetLightingInfo(
        ptr: Long,
        centerX: Float,
        centerY: Float,
        centerZ: Float,
        radius: Float,
        ambientShadowAlpha: Float,
        spotShadowAlpha: Float,
    )
}
