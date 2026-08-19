package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class GraphiteRecording internal constructor(ptr: Long) : Managed(ptr, GraphiteRecordingNative::nRelease)

private object GraphiteRecordingNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nRelease(ptr: Long)
}
