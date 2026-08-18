package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** A 4x4 transform matrix. Mirrors Skia's `SkM44`. */
class M44 internal constructor(ptr: Long) : Managed(ptr, M44Native::nRelease) {
    fun cloneM44(): M44 = M44(M44Native.nClone(nativePtr))

    /** 16 floats, row by row. */
    val rowMajor: FloatArray get() = M44Native.nRowMajor(nativePtr)

    /** `null` if this matrix isn't invertible. */
    fun invert(): M44? = M44Native.nInvert(nativePtr).takeIf { it != 0L }?.let { M44(it) }

    /** Transforms the 4-component vector `[x, y, z, w]`. */
    fun map(v: FloatArray): FloatArray = M44Native.nMap(nativePtr, v)

    override fun equals(other: Any?): Boolean = other is M44 && M44Native.nEquals(nativePtr, other.nativePtr)
    override fun hashCode(): Int = rowMajor.contentHashCode()

    companion object {
        fun identity(): M44 = M44(M44Native.nMakeIdentity())

        /** [rowMajor] is 16 floats, row by row. */
        fun makeFromRowMajor(rowMajor: FloatArray): M44 = M44(M44Native.nMakeFromRowMajor(rowMajor))

        fun makeTranslate(x: Float, y: Float, z: Float = 0f): M44 = M44(M44Native.nMakeTranslate(x, y, z))
        fun makeScale(x: Float, y: Float, z: Float = 1f): M44 = M44(M44Native.nMakeScale(x, y, z))

        /** [axis] need not be normalized. */
        fun makeRotate(axisX: Float, axisY: Float, axisZ: Float, radians: Float): M44 =
            M44(M44Native.nMakeRotate(axisX, axisY, axisZ, radians))

        fun concat(a: M44, b: M44): M44 = M44(M44Native.nConcat(a.nativePtr, b.nativePtr))
    }
}

private object M44Native {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeIdentity(): Long
    external fun nMakeFromRowMajor(rowMajor: FloatArray): Long
    external fun nMakeTranslate(x: Float, y: Float, z: Float): Long
    external fun nMakeScale(x: Float, y: Float, z: Float): Long
    external fun nMakeRotate(axisX: Float, axisY: Float, axisZ: Float, radians: Float): Long
    external fun nRelease(ptr: Long)
    external fun nClone(ptr: Long): Long
    external fun nRowMajor(ptr: Long): FloatArray
    external fun nConcat(aPtr: Long, bPtr: Long): Long
    external fun nInvert(ptr: Long): Long
    external fun nMap(ptr: Long, v: FloatArray): FloatArray
    external fun nEquals(aPtr: Long, bPtr: Long): Boolean
}
