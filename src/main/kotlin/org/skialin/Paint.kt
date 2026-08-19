package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Paint internal constructor(
    ptr: Long,
) : Managed(ptr, PaintNative::nRelease) {
    constructor() : this(PaintNative.nMake())

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

    fun setColorFilter(filter: ColorFilter?) {
        PaintNative.nSetColorFilter(nativePtr, filter?.nativePtr ?: 0L)
    }

    fun setImageFilter(filter: ImageFilter?) {
        PaintNative.nSetImageFilter(nativePtr, filter?.nativePtr ?: 0L)
    }

    fun setMaskFilter(filter: MaskFilter?) {
        PaintNative.nSetMaskFilter(nativePtr, filter?.nativePtr ?: 0L)
    }

    fun setPathEffect(effect: PathEffect?) {
        PaintNative.nSetPathEffect(nativePtr, effect?.nativePtr ?: 0L)
    }

    fun getShader(): Shader? = PaintNative.nGetShader(nativePtr).takeIf { it != 0L }?.let { Shader(it) }

    fun getColorFilter(): ColorFilter? = PaintNative.nGetColorFilter(nativePtr).takeIf { it != 0L }?.let { ColorFilter(it) }

    fun getImageFilter(): ImageFilter? = PaintNative.nGetImageFilter(nativePtr).takeIf { it != 0L }?.let { ImageFilter(it) }

    fun getMaskFilter(): MaskFilter? = PaintNative.nGetMaskFilter(nativePtr).takeIf { it != 0L }?.let { MaskFilter(it) }

    fun getPathEffect(): PathEffect? = PaintNative.nGetPathEffect(nativePtr).takeIf { it != 0L }?.let { PathEffect(it) }

    val blendMode: BlendMode get() = BlendMode.entries[PaintNative.nGetBlendMode(nativePtr)]

    /** Resets this paint to its default (freshly-constructed) state. */
    fun reset() = PaintNative.nReset(nativePtr)

    var isDither: Boolean
        get() = PaintNative.nIsDither(nativePtr)
        set(value) = PaintNative.nSetDither(nativePtr, value)

    var alpha: Int
        get() = PaintNative.nGetAlpha(nativePtr)
        set(value) = PaintNative.nSetAlpha(nativePtr, value)

    var alphaf: Float
        get() = PaintNative.nGetAlphaf(nativePtr)
        set(value) = PaintNative.nSetAlphaf(nativePtr, value)

    fun setARGB(
        a: Int,
        r: Int,
        g: Int,
        b: Int,
    ) = PaintNative.nSetARGB(nativePtr, a, r, g, b)

    var strokeMiter: Float
        get() = PaintNative.nGetStrokeMiter(nativePtr)
        set(value) = PaintNative.nSetStrokeMiter(nativePtr, value)

    /** `true` if this paint is guaranteed to draw nothing (e.g. a fully transparent color with default blend mode). */
    val nothingToDraw: Boolean get() = PaintNative.nNothingToDraw(nativePtr)

    /** `true` if this paint's blend mode is the default `SrcOver`. */
    val isSrcOver: Boolean get() = PaintNative.nIsSrcOver(nativePtr)
}

private object PaintNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(): Long

    external fun nRelease(ptr: Long)

    external fun nGetColor(ptr: Long): Int

    external fun nSetColor(
        ptr: Long,
        color: Int,
    )

    external fun nIsAntiAlias(ptr: Long): Boolean

    external fun nSetAntiAlias(
        ptr: Long,
        antiAlias: Boolean,
    )

    external fun nGetStyle(ptr: Long): Int

    external fun nSetStyle(
        ptr: Long,
        style: Int,
    )

    external fun nGetStrokeWidth(ptr: Long): Float

    external fun nSetStrokeWidth(
        ptr: Long,
        width: Float,
    )

    external fun nGetStrokeCap(ptr: Long): Int

    external fun nSetStrokeCap(
        ptr: Long,
        cap: Int,
    )

    external fun nGetStrokeJoin(ptr: Long): Int

    external fun nSetStrokeJoin(
        ptr: Long,
        join: Int,
    )

    external fun nSetBlendMode(
        ptr: Long,
        mode: Int,
    )

    external fun nSetShader(
        ptr: Long,
        shaderPtr: Long,
    )

    external fun nSetColorFilter(
        ptr: Long,
        filterPtr: Long,
    )

    external fun nSetImageFilter(
        ptr: Long,
        filterPtr: Long,
    )

    external fun nSetMaskFilter(
        ptr: Long,
        filterPtr: Long,
    )

    external fun nSetPathEffect(
        ptr: Long,
        effectPtr: Long,
    )

    external fun nGetShader(ptr: Long): Long

    external fun nGetColorFilter(ptr: Long): Long

    external fun nGetImageFilter(ptr: Long): Long

    external fun nGetMaskFilter(ptr: Long): Long

    external fun nGetPathEffect(ptr: Long): Long

    external fun nGetBlendMode(ptr: Long): Int

    external fun nReset(ptr: Long)

    external fun nIsDither(ptr: Long): Boolean

    external fun nSetDither(
        ptr: Long,
        dither: Boolean,
    )

    external fun nGetAlpha(ptr: Long): Int

    external fun nSetAlpha(
        ptr: Long,
        alpha: Int,
    )

    external fun nGetAlphaf(ptr: Long): Float

    external fun nSetAlphaf(
        ptr: Long,
        alpha: Float,
    )

    external fun nSetARGB(
        ptr: Long,
        a: Int,
        r: Int,
        g: Int,
        b: Int,
    )

    external fun nGetStrokeMiter(ptr: Long): Float

    external fun nSetStrokeMiter(
        ptr: Long,
        miterLimit: Float,
    )

    external fun nNothingToDraw(ptr: Long): Boolean

    external fun nIsSrcOver(ptr: Long): Boolean
}
