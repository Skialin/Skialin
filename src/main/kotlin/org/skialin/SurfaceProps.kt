package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class SurfaceProps internal constructor(ptr: Long) : Managed(ptr, SurfacePropsNative::nRelease) {
    constructor(
        flags: Int = SurfacePropsFlags.DEFAULT,
        pixelGeometry: PixelGeometry = PixelGeometry.UNKNOWN,
        textContrast: Float = 0f,
        textGamma: Float = 0f,
    ) : this(SurfacePropsNative.nMake(flags, pixelGeometry.ordinal, textContrast, textGamma))

    val flags: Int get() = SurfacePropsNative.nFlags(nativePtr)
    val pixelGeometry: PixelGeometry get() = PixelGeometry.entries[SurfacePropsNative.nPixelGeometry(nativePtr)]
    val textContrast: Float get() = SurfacePropsNative.nTextContrast(nativePtr)
    val textGamma: Float get() = SurfacePropsNative.nTextGamma(nativePtr)

    fun cloneSurfaceProps(): SurfaceProps = SurfaceProps(SurfacePropsNative.nClone(nativePtr))

    fun cloneWithPixelGeometry(pixelGeometry: PixelGeometry): SurfaceProps =
        SurfaceProps(SurfacePropsNative.nCloneWithPixelGeometry(nativePtr, pixelGeometry.ordinal))

    override fun equals(other: Any?): Boolean = other is SurfaceProps && SurfacePropsNative.nEquals(nativePtr, other.nativePtr)
    override fun hashCode(): Int = flags * 31 + pixelGeometry.hashCode()
}

private object SurfacePropsNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(flags: Int, pixelGeometry: Int, textContrast: Float, textGamma: Float): Long
    external fun nRelease(ptr: Long)
    external fun nClone(ptr: Long): Long
    external fun nCloneWithPixelGeometry(ptr: Long, pixelGeometry: Int): Long
    external fun nFlags(ptr: Long): Int
    external fun nPixelGeometry(ptr: Long): Int
    external fun nTextContrast(ptr: Long): Float
    external fun nTextGamma(ptr: Long): Float
    external fun nEquals(aPtr: Long, bPtr: Long): Boolean
}
