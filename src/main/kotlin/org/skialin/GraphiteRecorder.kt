package org.skialin

import org.skialin.impl.NativeLoader

/** Not thread-safe; one per thread/frame. Same close()-only rationale as [GraphiteContext]. */
class GraphiteRecorder internal constructor(
    ptr: Long,
) : AutoCloseable {
    @Volatile
    private var ptr: Long = ptr

    val nativePtr: Long
        get() {
            check(ptr != 0L) { "GraphiteRecorder is closed" }
            return ptr
        }

    fun snap(): GraphiteRecording? {
        val recordingPtr = GraphiteRecorderNative.nSnap(nativePtr)
        return if (recordingPtr == 0L) null else GraphiteRecording(recordingPtr)
    }

    override fun close() {
        if (ptr != 0L) {
            GraphiteRecorderNative.nRelease(ptr)
            ptr = 0L
        }
    }
}

private object GraphiteRecorderNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)

    external fun nSnap(ptr: Long): Long
}
