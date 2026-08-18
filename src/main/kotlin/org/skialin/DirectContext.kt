package org.skialin

import org.skialin.impl.NativeLoader

/**
 * Wraps a native GrDirectContext (Ganesh + OpenGL). The calling thread must
 * already have a native GL context current (e.g. via LWJGL/GLFW) before
 * calling [makeGL]. GrDirectContext is thread-affine after creation: this
 * object, and every [Surface] created from it, must only ever be used from
 * that same thread, for as long as the GL context stays current there.
 *
 * Deliberately does not extend [org.skialin.impl.Managed]: Managed's
 * Cleaner-based safety net runs its release callback on an arbitrary JVM
 * thread, which is unsafe here since tearing down the underlying GL state
 * must happen on the thread the GL context is current on. [close] must be
 * called explicitly, from that thread; skipping it leaks the native
 * GrDirectContext rather than being caught by GC.
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
