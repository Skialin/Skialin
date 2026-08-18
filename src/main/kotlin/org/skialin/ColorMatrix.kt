package org.skialin

import org.skialin.impl.NativeLoader

/**
 * A 4x5 row-major color matrix, mirroring Skia's `SkColorMatrix`. Feeds directly
 * into [ColorFilter.makeMatrix].
 */
object ColorMatrix {
    fun identity(): FloatArray = ColorMatrixNative.nIdentity()

    fun makeScale(r: Float, g: Float, b: Float, a: Float = 1f): FloatArray = ColorMatrixNative.nScale(r, g, b, a)

    /** `0` desaturates entirely (grayscale), `1` is a no-op. */
    fun makeSaturation(sat: Float): FloatArray = ColorMatrixNative.nSaturation(sat)

    fun postTranslate(mat20: FloatArray, dr: Float, dg: Float, db: Float, da: Float): FloatArray =
        ColorMatrixNative.nPostTranslate(mat20, dr, dg, db, da)

    /** `result = a * b` (`a` is applied after `b`). */
    fun concat(a: FloatArray, b: FloatArray): FloatArray = ColorMatrixNative.nConcat(a, b)
}

private object ColorMatrixNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nIdentity(): FloatArray
    external fun nScale(r: Float, g: Float, b: Float, a: Float): FloatArray
    external fun nSaturation(sat: Float): FloatArray
    external fun nPostTranslate(mat20: FloatArray, dr: Float, dg: Float, db: Float, da: Float): FloatArray
    external fun nConcat(a20: FloatArray, b20: FloatArray): FloatArray
}
