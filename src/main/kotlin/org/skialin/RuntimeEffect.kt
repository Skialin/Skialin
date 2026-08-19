package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class RuntimeEffect private constructor(
    ptr: Long,
) : Managed(ptr, RuntimeEffectNative::nRelease) {
    fun makeShader(
        uniforms: ByteArray? = null,
        children: Array<Shader> = emptyArray(),
        localMatrix: Matrix33? = null,
    ): Shader? {
        val childPtrs = LongArray(children.size) { children[it].nativePtr }
        val ptr = RuntimeEffectNative.nMakeShader(nativePtr, uniforms, childPtrs, localMatrix?.values)
        return if (ptr == 0L) null else Shader(ptr)
    }

    fun makeColorFilter(
        uniforms: ByteArray? = null,
        children: Array<ColorFilter> = emptyArray(),
    ): ColorFilter? {
        val childPtrs = LongArray(children.size) { children[it].nativePtr }
        val ptr = RuntimeEffectNative.nMakeColorFilter(nativePtr, uniforms, childPtrs)
        return if (ptr == 0L) null else ColorFilter(ptr)
    }

    fun makeBlender(
        uniforms: ByteArray? = null,
        children: Array<Shader> = emptyArray(),
    ): Blender? {
        val childPtrs = LongArray(children.size) { children[it].nativePtr }
        val ptr = RuntimeEffectNative.nMakeBlender(nativePtr, uniforms, childPtrs)
        return if (ptr == 0L) null else Blender(ptr)
    }

    companion object {
        fun makeForShader(sksl: String): RuntimeEffect = RuntimeEffect(RuntimeEffectNative.nMakeForShader(sksl))

        fun makeForColorFilter(sksl: String): RuntimeEffect = RuntimeEffect(RuntimeEffectNative.nMakeForColorFilter(sksl))

        fun makeForBlender(sksl: String): RuntimeEffect = RuntimeEffect(RuntimeEffectNative.nMakeForBlender(sksl))
    }
}

private object RuntimeEffectNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeForShader(sksl: String): Long

    external fun nMakeForColorFilter(sksl: String): Long

    external fun nMakeForBlender(sksl: String): Long

    external fun nRelease(ptr: Long)

    external fun nMakeShader(
        ptr: Long,
        uniforms: ByteArray?,
        children: LongArray,
        localMatrix: FloatArray?,
    ): Long

    external fun nMakeColorFilter(
        ptr: Long,
        uniforms: ByteArray?,
        children: LongArray,
    ): Long

    external fun nMakeBlender(
        ptr: Long,
        uniforms: ByteArray?,
        children: LongArray,
    ): Long
}
