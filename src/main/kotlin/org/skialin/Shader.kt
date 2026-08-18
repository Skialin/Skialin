package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Specifies the source color(s) for a [Paint]. Mirrors Skia's `SkShader`. */
class Shader internal constructor(ptr: Long) : Managed(ptr, ShaderNative::nRelease) {
    val isOpaque: Boolean get() = ShaderNative.nIsOpaque(nativePtr)

    fun withLocalMatrix(matrix: Matrix33): Shader = Shader(ShaderNative.nWithLocalMatrix(nativePtr, matrix.values))

    companion object {
        fun makeEmpty(): Shader = Shader(ShaderNative.nMakeEmpty())
        fun makeColor(color: Color): Shader = Shader(ShaderNative.nMakeColor(color))
    }
}

private object ShaderNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeEmpty(): Long
    external fun nMakeColor(color: Int): Long
    external fun nRelease(ptr: Long)
    external fun nIsOpaque(ptr: Long): Boolean
    external fun nWithLocalMatrix(ptr: Long, matrix: FloatArray): Long
}
