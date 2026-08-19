package org.skialin

import org.skialin.impl.NativeLoader

/**
 * Wraps a skgpu::graphite::Context (Vulkan only). Thread-safe and
 * long-lived, unlike a [GraphiteRecorder] made from it.
 *
 * Doesn't extend [org.skialin.impl.Managed]: same rationale as
 * [DirectContext] -- native teardown shouldn't run on an arbitrary
 * Cleaner thread. [close] must be called explicitly.
 */
class GraphiteContext private constructor(ptr: Long) : AutoCloseable {
    @Volatile
    private var ptr: Long = ptr

    val nativePtr: Long
        get() {
            check(ptr != 0L) { "GraphiteContext is closed" }
            return ptr
        }

    fun makeRecorder(): GraphiteRecorder? {
        val recorderPtr = GraphiteContextNative.nMakeRecorder(nativePtr)
        return if (recorderPtr == 0L) null else GraphiteRecorder(recorderPtr)
    }

    /** Returns the real `skgpu::graphite::InsertStatus::V` value (0 == success). */
    fun insertRecording(recording: GraphiteRecording, targetSurface: Surface): Int =
        GraphiteContextNative.nInsertRecording(nativePtr, recording.nativePtr, targetSurface.nativePtr)

    fun submit(syncToCpu: Boolean = false): Boolean = GraphiteContextNative.nSubmit(nativePtr, syncToCpu)

    override fun close() {
        if (ptr != 0L) {
            GraphiteContextNative.nRelease(ptr)
            ptr = 0L
        }
    }

    companion object {
        fun makeVulkan(
            instance: Long,
            physicalDevice: Long,
            device: Long,
            queue: Long,
            graphicsQueueIndex: Int,
            maxApiVersion: Int,
            protectedContext: Boolean = false,
        ): GraphiteContext? {
            val ptr = GraphiteContextNative.nMakeVulkan(instance, physicalDevice, device, queue, graphicsQueueIndex, maxApiVersion, protectedContext)
            return if (ptr == 0L) null else GraphiteContext(ptr)
        }
    }
}

private object GraphiteContextNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeVulkan(
        instance: Long,
        physicalDevice: Long,
        device: Long,
        queue: Long,
        graphicsQueueIndex: Int,
        maxApiVersion: Int,
        protectedContext: Boolean,
    ): Long
    external fun nRelease(ptr: Long)
    external fun nMakeRecorder(ptr: Long): Long
    external fun nInsertRecording(ptr: Long, recordingPtr: Long, targetSurfacePtr: Long): Int
    external fun nSubmit(ptr: Long, syncToCpu: Boolean): Boolean
}
