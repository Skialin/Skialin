package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Blender internal constructor(
    ptr: Long,
) : Managed(ptr, BlenderNative::nRelease) {
    companion object {
        fun mode(mode: BlendMode): Blender = Blender(BlenderNative.nMode(mode.ordinal))
    }
}

private object BlenderNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)

    external fun nMode(mode: Int): Long
}
