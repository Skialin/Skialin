package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader


class ColorSpace internal constructor(ptr: Long) : Managed(ptr, ColorSpaceNative::nRelease) {
    val gammaCloseToSrgb: Boolean get() = ColorSpaceNative.nGammaCloseToSrgb(nativePtr)
    val gammaIsLinear: Boolean get() = ColorSpaceNative.nGammaIsLinear(nativePtr)
    val isSrgb: Boolean get() = ColorSpaceNative.nIsSrgb(nativePtr)

    /** Row-major 3x3 matrix to XYZ D50, or null if this color space has no such matrix. */
    fun toXyzD50(): FloatArray? = ColorSpaceNative.nToXyzD50(nativePtr)

    fun toXyzD50Hash(): Int = ColorSpaceNative.nToXyzD50Hash(nativePtr)

    /** The 7 transfer-function coefficients (g, a, b, c, d, e, f). */
    fun transferFn(): FloatArray = ColorSpaceNative.nTransferFn(nativePtr)

    fun invTransferFn(): FloatArray = ColorSpaceNative.nInvTransferFn(nativePtr)

    /** Null if the transfer function can't be represented by the standard ICC 7-parameter equation (e.g. PQ, HLG). */
    fun numericalTransferFn(): FloatArray? = ColorSpaceNative.nNumericalTransferFn(nativePtr)

    fun transferFnHash(): Int = ColorSpaceNative.nTransferFnHash(nativePtr)
    fun hash(): Long = ColorSpaceNative.nHash(nativePtr)

    fun gamutTransformTo(dst: ColorSpace): FloatArray = ColorSpaceNative.nGamutTransformTo(nativePtr, dst.nativePtr)

    fun makeLinearGamma(): ColorSpace = ColorSpace(ColorSpaceNative.nMakeLinearGamma(nativePtr))
    fun makeSRGBGamma(): ColorSpace = ColorSpace(ColorSpaceNative.nMakeSRGBGamma(nativePtr))
    fun makeColorSpin(): ColorSpace = ColorSpace(ColorSpaceNative.nMakeColorSpin(nativePtr))

    fun serialize(): Data = Data(ColorSpaceNative.nSerialize(nativePtr))

    fun contentEquals(other: ColorSpace): Boolean = ColorSpaceNative.nEquals(nativePtr, other.nativePtr)

    companion object {
        fun makeSRGB(): ColorSpace = ColorSpace(ColorSpaceNative.nMakeSRGB())
        fun makeSRGBLinear(): ColorSpace = ColorSpace(ColorSpaceNative.nMakeSRGBLinear())

        fun makeRGB(transferFn: FloatArray, toXyzD50: FloatArray): ColorSpace =
            ColorSpace(ColorSpaceNative.nMakeRGB(transferFn, toXyzD50))

        /** Null for an invalid or unsupported combination of code points. */
        fun makeCICP(primaries: CicpPrimaries, transferFn: CicpTransferFn): ColorSpace? =
            ColorSpaceNative.nMakeCICP(primaries.code, transferFn.code).takeIf { it != 0L }?.let { ColorSpace(it) }

        /** Null if [bytes] doesn't parse as an ICC profile. */
        fun makeFromIccProfile(bytes: ByteArray): ColorSpace? =
            ColorSpaceNative.nMakeFromIccProfile(bytes).takeIf { it != 0L }?.let { ColorSpace(it) }

        fun deserialize(bytes: ByteArray): ColorSpace? =
            ColorSpaceNative.nDeserialize(bytes).takeIf { it != 0L }?.let { ColorSpace(it) }
    }
}

private object ColorSpaceNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeSRGB(): Long
    external fun nMakeSRGBLinear(): Long
    external fun nMakeRGB(transferFn: FloatArray, toXyzD50: FloatArray): Long
    external fun nMakeCICP(primaries: Int, transferFn: Int): Long
    external fun nMakeFromIccProfile(bytes: ByteArray): Long
    external fun nDeserialize(bytes: ByteArray): Long
    external fun nRelease(ptr: Long)
    external fun nGammaCloseToSrgb(ptr: Long): Boolean
    external fun nGammaIsLinear(ptr: Long): Boolean
    external fun nIsSrgb(ptr: Long): Boolean
    external fun nToXyzD50(ptr: Long): FloatArray?
    external fun nToXyzD50Hash(ptr: Long): Int
    external fun nTransferFn(ptr: Long): FloatArray
    external fun nInvTransferFn(ptr: Long): FloatArray
    external fun nNumericalTransferFn(ptr: Long): FloatArray?
    external fun nTransferFnHash(ptr: Long): Int
    external fun nHash(ptr: Long): Long
    external fun nGamutTransformTo(ptr: Long, dstPtr: Long): FloatArray
    external fun nMakeLinearGamma(ptr: Long): Long
    external fun nMakeSRGBGamma(ptr: Long): Long
    external fun nMakeColorSpin(ptr: Long): Long
    external fun nSerialize(ptr: Long): Long
    external fun nEquals(ptr: Long, otherPtr: Long): Boolean
}
