package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** A triangle mesh for [Canvas.drawVertices]. Mirrors Skia's `SkVertices`. */
class Vertices internal constructor(ptr: Long) : Managed(ptr, VerticesNative::nRelease) {
    companion object {
        /**
         * [positions] and (if given) [texs] hold `x0, y0, x1, y1, ...` pairs, so their
         * sizes must be even and equal. [texs], [colors], and [indices] may be empty.
         */
        fun makeCopy(
            mode: VertexMode, positions: FloatArray, texs: FloatArray = FloatArray(0),
            colors: IntArray = IntArray(0), indices: ShortArray = ShortArray(0),
        ): Vertices? {
            val ptr = VerticesNative.nMakeCopy(mode.ordinal, positions, texs, colors, indices)
            return if (ptr == 0L) null else Vertices(ptr)
        }
    }
}

private object VerticesNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)
    external fun nMakeCopy(mode: Int, positions: FloatArray, texs: FloatArray, colors: IntArray, indices: ShortArray): Long
}
