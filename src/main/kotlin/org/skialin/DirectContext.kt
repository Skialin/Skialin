package org.skialin

import org.skialin.impl.NativeLoader

/**
 * Wraps a native GrDirectContext (Ganesh + OpenGL). Caller must make a
 * native GL context current on this thread first (e.g. via LWJGL/GLFW).
 * Thread-affine after creation: this object, and any [Surface] made from
 * it, must stay on that thread.
 *
 * Doesn't extend [org.skialin.impl.Managed]: its Cleaner runs release on an
 * arbitrary thread, which would tear down GL state from the wrong one.
 * [close] must be called explicitly; skipping it leaks.
 */
class DirectContext private constructor(ptr: Long) : AutoCloseable {
    @Volatile
    private var ptr: Long = ptr

    val nativePtr: Long
        get() {
            check(ptr != 0L) { "DirectContext is closed" }
            return ptr
        }

    fun flush() {
        DirectContextNative.nFlush(nativePtr)
    }

    fun submit(syncCpu: Boolean = false) {
        DirectContextNative.nSubmit(nativePtr, syncCpu)
    }

    override fun close() {
        if (ptr != 0L) {
            DirectContextNative.nRelease(ptr)
            ptr = 0L
        }
    }

    companion object {
        fun makeGL(): DirectContext? {
            val ptr = DirectContextNative.nMakeGL()
            return if (ptr == 0L) null else DirectContext(ptr)
        }
    }
}

private object DirectContextNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeGL(): Long
    external fun nRelease(ptr: Long)
    external fun nFlush(ptr: Long)
    external fun nSubmit(ptr: Long, syncCpu: Boolean)
}
