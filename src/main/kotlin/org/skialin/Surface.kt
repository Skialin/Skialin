package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Surface private constructor(ptr: Long) : Managed(ptr, SurfaceNative::nRelease) {
    fun canvas(): Canvas = Canvas(SurfaceNative.nGetCanvas(nativePtr))

    fun imageSnapshot(): Image? {
        val ptr = SurfaceNative.nMakeImageSnapshot(nativePtr)
        return if (ptr == 0L) null else Image(ptr)
    }

    companion object {
        fun makeRasterN32Premul(width: Int, height: Int): Surface? {
            val ptr = SurfaceNative.nMakeRasterN32Premul(width, height)
            return if (ptr == 0L) null else Surface(ptr)
        }

        fun makeRaster(info: ImageInfo): Surface? {
            val ptr = SurfaceNative.nMakeRaster(info.nativePtr)
            return if (ptr == 0L) null else Surface(ptr)
        }
    }
}

private object SurfaceNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeRasterN32Premul(width: Int, height: Int): Long
    external fun nMakeRaster(infoPtr: Long): Long
    external fun nRelease(ptr: Long)
    external fun nGetCanvas(ptr: Long): Long
    external fun nMakeImageSnapshot(ptr: Long): Long
}
