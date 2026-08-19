package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class PictureRecorder : Managed(PictureRecorderNative.nNew(), PictureRecorderNative::nRelease) {
    fun beginRecording(bounds: Rect): Canvas = Canvas(PictureRecorderNative.nBeginRecording(nativePtr, floatArrayOf(bounds.left, bounds.top, bounds.right, bounds.bottom)))

    fun recordingCanvas(): Canvas = Canvas(PictureRecorderNative.nGetRecordingCanvas(nativePtr))

    fun finishRecordingAsPicture(): Picture? {
        val ptr = PictureRecorderNative.nFinishRecordingAsPicture(nativePtr)
        return if (ptr == 0L) null else Picture(ptr)
    }
}

private object PictureRecorderNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nNew(): Long
    external fun nRelease(ptr: Long)
    external fun nBeginRecording(ptr: Long, bounds: FloatArray): Long
    external fun nGetRecordingCanvas(ptr: Long): Long
    external fun nFinishRecordingAsPicture(ptr: Long): Long
}
