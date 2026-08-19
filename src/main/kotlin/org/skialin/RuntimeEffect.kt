package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/**
 * A compiled SkSL shader or color-filter effect. Mirrors Skia's `SkRuntimeEffect`.
 *
 * @throws IllegalArgumentException if [sksl] fails to compile.
 */
class RuntimeEffect private constructor(
    ptr: Long,
) : Managed(ptr, RuntimeEffectNative::nRelease) {
    /**
     * [uniforms] is a raw byte buffer packed to match the SkSL uniform block's layout
     * (the caller is responsible for knowing that layout); pass `null` if the effect
     * declares no uniforms.
     */
    fun makeShader(
        uniforms: ByteArray? = null,
        children: Array<Shader> = emptyArray(),
        localMatrix: Matrix33? = null,
    ): Shader? {
        val childPtrs = LongArray(children.size) { children[it].nativePtr }
        val ptr = RuntimeEffectNative.nMakeShader(nativePtr, uniforms, childPtrs, localMatrix?.values)
        return if (ptr == 0L) null else Shader(ptr)
    }

    /**
     * [uniforms] is a raw byte buffer packed to match the SkSL uniform block's layout
     * (the caller is responsible for knowing that layout); pass `null` if the effect
     * declares no uniforms.
     */
    fun makeColorFilter(
        uniforms: ByteArray? = null,
        children: Array<ColorFilter> = emptyArray(),
    ): ColorFilter? {
        val childPtrs = LongArray(children.size) { children[it].nativePtr }
        val ptr = RuntimeEffectNative.nMakeColorFilter(nativePtr, uniforms, childPtrs)
        return if (ptr == 0L) null else ColorFilter(ptr)
    }

    companion object {
        /** [sksl] must define `vec4 main(vec2 coord) { ... }` returning a premultiplied color. */
        fun makeForShader(sksl: String): RuntimeEffect = RuntimeEffect(RuntimeEffectNative.nMakeForShader(sksl))

        /** [sksl] must define `vec4 main(vec4 inColor) { ... }`. */
        fun makeForColorFilter(sksl: String): RuntimeEffect = RuntimeEffect(RuntimeEffectNative.nMakeForColorFilter(sksl))
    }
}

private object RuntimeEffectNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeForShader(sksl: String): Long

    external fun nMakeForColorFilter(sksl: String): Long

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
}
