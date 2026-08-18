package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Path internal constructor(ptr: Long) : Managed(ptr, PathNative::nRelease) {
    val isEmpty: Boolean get() = PathNative.nIsEmpty(nativePtr)

    val bounds: Rect
        get() {
            val b = PathNative.nGetBounds(nativePtr)
            return Rect(b[0], b[1], b[2], b[3])
        }

    fun contains(point: Point): Boolean = PathNative.nContains(nativePtr, point.x, point.y)
}


private object PathNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)
    external fun nIsEmpty(ptr: Long): Boolean
    external fun nGetBounds(ptr: Long): FloatArray
    external fun nContains(ptr: Long, x: Float, y: Float): Boolean
}
