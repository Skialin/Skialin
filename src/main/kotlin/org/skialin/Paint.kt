package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Paint : Managed(PaintNative.nMake(), PaintNative::nRelease) {
    var color: Color
        get() = PaintNative.nGetColor(nativePtr)
        set(value) = PaintNative.nSetColor(nativePtr, value)

    var isAntiAlias: Boolean
        get() = PaintNative.nIsAntiAlias(nativePtr)
        set(value) = PaintNative.nSetAntiAlias(nativePtr, value)

    var style: PaintStyle
        get() = PaintStyle.entries[PaintNative.nGetStyle(nativePtr)]
        set(value) = PaintNative.nSetStyle(nativePtr, value.ordinal)

    var strokeWidth: Float
        get() = PaintNative.nGetStrokeWidth(nativePtr)
        set(value) = PaintNative.nSetStrokeWidth(nativePtr, value)

    var strokeCap: StrokeCap
        get() = StrokeCap.entries[PaintNative.nGetStrokeCap(nativePtr)]
        set(value) = PaintNative.nSetStrokeCap(nativePtr, value.ordinal)

    var strokeJoin: StrokeJoin
        get() = StrokeJoin.entries[PaintNative.nGetStrokeJoin(nativePtr)]
        set(value) = PaintNative.nSetStrokeJoin(nativePtr, value.ordinal)

    fun setBlendMode(mode: BlendMode) {
        PaintNative.nSetBlendMode(nativePtr, mode.ordinal)
    }

    fun setShader(shader: Shader?) {
        PaintNative.nSetShader(nativePtr, shader?.nativePtr ?: 0L)
    }
}

private object PaintNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(): Long
    external fun nRelease(ptr: Long)
    external fun nGetColor(ptr: Long): Int
    external fun nSetColor(ptr: Long, color: Int)
    external fun nIsAntiAlias(ptr: Long): Boolean
    external fun nSetAntiAlias(ptr: Long, antiAlias: Boolean)
    external fun nGetStyle(ptr: Long): Int
    external fun nSetStyle(ptr: Long, style: Int)
    external fun nGetStrokeWidth(ptr: Long): Float
    external fun nSetStrokeWidth(ptr: Long, width: Float)
    external fun nGetStrokeCap(ptr: Long): Int
    external fun nSetStrokeCap(ptr: Long, cap: Int)
    external fun nGetStrokeJoin(ptr: Long): Int
    external fun nSetStrokeJoin(ptr: Long, join: Int)
    external fun nSetBlendMode(ptr: Long, mode: Int)
    external fun nSetShader(ptr: Long, shaderPtr: Long)
}
