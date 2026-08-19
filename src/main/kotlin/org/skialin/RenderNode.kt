package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class RenderNode(
    context: RenderNodeContext,
) : Managed(RenderNodeNative.nMake(context.nativePtr), RenderNodeNative::nRelease) {
    var layerPaint: Paint?
        get() = RenderNodeNative.nGetLayerPaint(nativePtr).let { if (it == 0L) null else Paint(it) }
        set(value) = RenderNodeNative.nSetLayerPaint(nativePtr, value?.nativePtr ?: 0L)

    var bounds: Rect
        get() = RenderNodeNative.nGetBounds(nativePtr).let { Rect(it[0], it[1], it[2], it[3]) }
        set(value) = RenderNodeNative.nSetBounds(nativePtr, value.left, value.top, value.right, value.bottom)

    var pivot: Point?
        get() {
            val x = RenderNodeNative.nGetPivotX(nativePtr)
            return if (x.isNaN()) null else Point(x, RenderNodeNative.nGetPivotY(nativePtr))
        }
        set(value) = RenderNodeNative.nSetPivot(nativePtr, value?.x ?: Float.NaN, value?.y ?: Float.NaN)

    var alpha: Float
        get() = RenderNodeNative.nGetAlpha(nativePtr)
        set(value) = RenderNodeNative.nSetAlpha(nativePtr, value)

    var scaleX: Float
        get() = RenderNodeNative.nGetScaleX(nativePtr)
        set(value) = RenderNodeNative.nSetScaleX(nativePtr, value)

    var scaleY: Float
        get() = RenderNodeNative.nGetScaleY(nativePtr)
        set(value) = RenderNodeNative.nSetScaleY(nativePtr, value)

    var translationX: Float
        get() = RenderNodeNative.nGetTranslationX(nativePtr)
        set(value) = RenderNodeNative.nSetTranslationX(nativePtr, value)

    var translationY: Float
        get() = RenderNodeNative.nGetTranslationY(nativePtr)
        set(value) = RenderNodeNative.nSetTranslationY(nativePtr, value)

    var shadowElevation: Float
        get() = RenderNodeNative.nGetShadowElevation(nativePtr)
        set(value) = RenderNodeNative.nSetShadowElevation(nativePtr, value)

    var ambientShadowColor: Int
        get() = RenderNodeNative.nGetAmbientShadowColor(nativePtr)
        set(value) = RenderNodeNative.nSetAmbientShadowColor(nativePtr, value)

    var spotShadowColor: Int
        get() = RenderNodeNative.nGetSpotShadowColor(nativePtr)
        set(value) = RenderNodeNative.nSetSpotShadowColor(nativePtr, value)

    var rotationX: Float
        get() = RenderNodeNative.nGetRotationX(nativePtr)
        set(value) = RenderNodeNative.nSetRotationX(nativePtr, value)

    var rotationY: Float
        get() = RenderNodeNative.nGetRotationY(nativePtr)
        set(value) = RenderNodeNative.nSetRotationY(nativePtr, value)

    var rotationZ: Float
        get() = RenderNodeNative.nGetRotationZ(nativePtr)
        set(value) = RenderNodeNative.nSetRotationZ(nativePtr, value)

    var cameraDistance: Float
        get() = RenderNodeNative.nGetCameraDistance(nativePtr)
        set(value) = RenderNodeNative.nSetCameraDistance(nativePtr, value)

    var clip: Boolean
        get() = RenderNodeNative.nGetClip(nativePtr)
        set(value) = RenderNodeNative.nSetClip(nativePtr, value)

    fun setClipRect(
        r: Rect,
        mode: ClipOp = ClipOp.INTERSECT,
    ) = RenderNodeNative.nSetClipRect(nativePtr, r.left, r.top, r.right, r.bottom, mode.ordinal)

    fun setClipRRect(
        r: RRect,
        mode: ClipOp = ClipOp.INTERSECT,
    ) = RenderNodeNative.nSetClipRRect(nativePtr, r.nativePtr, mode.ordinal)

    fun beginRecording(): Canvas = Canvas(RenderNodeNative.nBeginRecording(nativePtr))

    fun endRecording() = RenderNodeNative.nEndRecording(nativePtr)

    fun drawInto(canvas: Canvas) = RenderNodeNative.nDrawInto(nativePtr, canvas.ptr)
}

private object RenderNodeNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(contextPtr: Long): Long

    external fun nRelease(ptr: Long)

    external fun nGetLayerPaint(ptr: Long): Long

    external fun nSetLayerPaint(
        ptr: Long,
        paintPtr: Long,
    )

    external fun nGetBounds(ptr: Long): FloatArray

    external fun nSetBounds(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
    )

    external fun nGetPivotX(ptr: Long): Float

    external fun nGetPivotY(ptr: Long): Float

    external fun nSetPivot(
        ptr: Long,
        x: Float,
        y: Float,
    )

    external fun nGetAlpha(ptr: Long): Float

    external fun nSetAlpha(
        ptr: Long,
        value: Float,
    )

    external fun nGetScaleX(ptr: Long): Float

    external fun nSetScaleX(
        ptr: Long,
        value: Float,
    )

    external fun nGetScaleY(ptr: Long): Float

    external fun nSetScaleY(
        ptr: Long,
        value: Float,
    )

    external fun nGetTranslationX(ptr: Long): Float

    external fun nSetTranslationX(
        ptr: Long,
        value: Float,
    )

    external fun nGetTranslationY(ptr: Long): Float

    external fun nSetTranslationY(
        ptr: Long,
        value: Float,
    )

    external fun nGetShadowElevation(ptr: Long): Float

    external fun nSetShadowElevation(
        ptr: Long,
        value: Float,
    )

    external fun nGetAmbientShadowColor(ptr: Long): Int

    external fun nSetAmbientShadowColor(
        ptr: Long,
        value: Int,
    )

    external fun nGetSpotShadowColor(ptr: Long): Int

    external fun nSetSpotShadowColor(
        ptr: Long,
        value: Int,
    )

    external fun nGetRotationX(ptr: Long): Float

    external fun nSetRotationX(
        ptr: Long,
        value: Float,
    )

    external fun nGetRotationY(ptr: Long): Float

    external fun nSetRotationY(
        ptr: Long,
        value: Float,
    )

    external fun nGetRotationZ(ptr: Long): Float

    external fun nSetRotationZ(
        ptr: Long,
        value: Float,
    )

    external fun nGetCameraDistance(ptr: Long): Float

    external fun nSetCameraDistance(
        ptr: Long,
        value: Float,
    )

    external fun nSetClipRect(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        mode: Int,
    )

    external fun nSetClipRRect(
        ptr: Long,
        rrectPtr: Long,
        mode: Int,
    )

    external fun nGetClip(ptr: Long): Boolean

    external fun nSetClip(
        ptr: Long,
        clip: Boolean,
    )

    external fun nBeginRecording(ptr: Long): Long

    external fun nEndRecording(ptr: Long)

    external fun nDrawInto(
        ptr: Long,
        canvasPtr: Long,
    )
}
