package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** A Lottie/Bodymovin JSON animation player. Mirrors Skia's `skottie::Animation`. */
class SkottieAnimation private constructor(
    ptr: Long,
) : Managed(ptr, SkottieAnimationNative::nRelease) {
    /** Renders at [dst], or at [size] at the origin when `null`. */
    fun render(
        canvas: Canvas,
        dst: Rect? = null,
    ) = SkottieAnimationNative.nRender(nativePtr, canvas.ptr, dst?.let { floatArrayOf(it.left, it.top, it.right, it.bottom) })

    /** [t] is normalized progress in `[0, 1]`. */
    fun seek(t: Float) = SkottieAnimationNative.nSeek(nativePtr, t)

    fun seekFrame(frame: Double) = SkottieAnimationNative.nSeekFrame(nativePtr, frame)

    /** Total duration in seconds. */
    val duration: Double get() = SkottieAnimationNative.nDuration(nativePtr)

    val fps: Double get() = SkottieAnimationNative.nFps(nativePtr)

    val size: Pair<Float, Float>
        get() = SkottieAnimationNative.nSize(nativePtr).let { it[0] to it[1] }

    companion object {
        /** `null` if [bytes] doesn't parse as a valid animation. */
        fun makeFromBytes(bytes: ByteArray): SkottieAnimation? =
            SkottieAnimationNative.nMakeFromBytes(bytes).takeIf { it != 0L }?.let { SkottieAnimation(it) }
    }
}

private object SkottieAnimationNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeFromBytes(bytes: ByteArray): Long

    external fun nRelease(ptr: Long)

    external fun nRender(
        ptr: Long,
        canvasPtr: Long,
        dst: FloatArray?,
    )

    external fun nSeek(
        ptr: Long,
        t: Float,
    )

    external fun nSeekFrame(
        ptr: Long,
        frame: Double,
    )

    external fun nDuration(ptr: Long): Double

    external fun nFps(ptr: Long): Double

    external fun nSize(ptr: Long): FloatArray
}
