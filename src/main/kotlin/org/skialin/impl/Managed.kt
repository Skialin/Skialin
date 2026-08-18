package org.skialin.impl

import java.lang.ref.Cleaner

private val cleaner: Cleaner = Cleaner.create()

/**
 * Base class for types backed by a native Skia object. Holds the native
 * pointer and guarantees release exactly once, either via [close] or,
 * as a safety net, when the Kotlin object is garbage collected.
 */
abstract class Managed internal constructor(
    ptr: Long,
    release: (Long) -> Unit,
) : AutoCloseable {
    init {
        require(ptr != 0L) { "${this::class.simpleName}: native allocation failed" }
    }

    @Volatile
    private var ptr: Long = ptr
    private val cleanable: Cleaner.Cleanable = cleaner.register(this, ReleaseAction(ptr, release))

    val nativePtr: Long
        get() {
            check(ptr != 0L) { "${this::class.simpleName} is closed" }
            return ptr
        }

    override fun close() {
        if (ptr != 0L) {
            ptr = 0L
            cleanable.clean()
        }
    }

    private class ReleaseAction(private val ptr: Long, private val release: (Long) -> Unit) : Runnable {
        override fun run() {
            release(ptr)
        }
    }
}
