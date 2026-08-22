package org.skialin

import org.skialin.impl.NativeLoader

/**
 * Wraps a native GrDirectContext (Ganesh + OpenGL or Vulkan). For GL, the
 * caller must make a native GL context current on this thread first (e.g.
 * via LWJGL/GLFW); the resulting object, and any [Surface] made from it,
 * must then stay on that thread. Vulkan has no such requirement -- only
 * the instance/device/queue must outlive this context.
 *
 * Doesn't extend [org.skialin.impl.Managed]: its Cleaner runs release on an
 * arbitrary thread, which would tear down GL state from the wrong one for
 * the GL case. [close] must be called explicitly; skipping it leaks.
 */
class DirectContext private constructor(
    ptr: Long,
) : AutoCloseable {
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

    fun abandonContext() {
        DirectContextNative.nAbandonContext(nativePtr)
    }

    /**
     * Invalidates Ganesh's cached GL state (texture bindings, blend state,
     * etc.), needed whenever code outside Skia's control (e.g. the host
     * application's own GL renderer) may have changed GL state since the
     * last draw through this context. No-op for Vulkan contexts.
     */
    fun resetAll() {
        DirectContextNative.nResetAll(nativePtr)
    }

    var resourceCacheLimit: Long
        get() = DirectContextNative.nGetResourceCacheLimit(nativePtr)
        set(value) = DirectContextNative.nSetResourceCacheLimit(nativePtr, value)

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

        /**
         * instance/physicalDevice/device/queue are native VkInstance/
         * VkPhysicalDevice/VkDevice/VkQueue handles (e.g. from LWJGL's
         * `.address()`); they must outlive this context and everything
         * made from it.
         *
         * [getInstanceProcAddr] and [getDeviceProcAddr] are native
         * `vkGetInstanceProcAddr` / `vkGetDeviceProcAddr` function-pointer
         * addresses, used to resolve every other entry point. Leave either
         * at 0 for the default: the platform's own Vulkan loader, loaded
         * natively -- independent of any loader the caller used to create
         * the instance/device -- for [getInstanceProcAddr], and a lookup
         * through the instance-level entry point for [getDeviceProcAddr].
         * Both must stay valid for as long as this context is alive.
         */
        fun makeVulkan(
            instance: Long,
            physicalDevice: Long,
            device: Long,
            queue: Long,
            graphicsQueueIndex: Int,
            maxApiVersion: Int,
            protectedContext: Boolean = false,
            getInstanceProcAddr: Long = 0L,
            getDeviceProcAddr: Long = 0L,
        ): DirectContext? {
            val ptr =
                DirectContextNative.nMakeVulkan(
                    instance,
                    physicalDevice,
                    device,
                    queue,
                    graphicsQueueIndex,
                    maxApiVersion,
                    protectedContext,
                    getInstanceProcAddr,
                    getDeviceProcAddr,
                )
            return if (ptr == 0L) null else DirectContext(ptr)
        }

        /**
         * Same, but resolving every entry point through [getProc] instead of
         * a pair of native function pointers. Each lookup crosses back into
         * the JVM, so prefer the pointer-based overload unless the extra
         * control is needed; [getProc] is retained for as long as this
         * context is alive.
         */
        fun makeVulkan(
            instance: Long,
            physicalDevice: Long,
            device: Long,
            queue: Long,
            graphicsQueueIndex: Int,
            maxApiVersion: Int,
            getProc: VulkanGetProc,
            protectedContext: Boolean = false,
        ): DirectContext? {
            val ptr =
                DirectContextNative.nMakeVulkanWithGetProc(
                    instance,
                    physicalDevice,
                    device,
                    queue,
                    graphicsQueueIndex,
                    maxApiVersion,
                    protectedContext,
                    getProc,
                )
            return if (ptr == 0L) null else DirectContext(ptr)
        }
    }
}

private object DirectContextNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeGL(): Long

    external fun nMakeVulkan(
        instance: Long,
        physicalDevice: Long,
        device: Long,
        queue: Long,
        graphicsQueueIndex: Int,
        maxApiVersion: Int,
        protectedContext: Boolean,
        getInstanceProcAddr: Long,
        getDeviceProcAddr: Long,
    ): Long

    external fun nMakeVulkanWithGetProc(
        instance: Long,
        physicalDevice: Long,
        device: Long,
        queue: Long,
        graphicsQueueIndex: Int,
        maxApiVersion: Int,
        protectedContext: Boolean,
        getProc: VulkanGetProc,
    ): Long

    external fun nRelease(ptr: Long)

    external fun nFlush(ptr: Long)

    external fun nSubmit(
        ptr: Long,
        syncCpu: Boolean,
    )

    external fun nAbandonContext(ptr: Long)

    external fun nResetAll(ptr: Long)

    external fun nGetResourceCacheLimit(ptr: Long): Long

    external fun nSetResourceCacheLimit(
        ptr: Long,
        maxResourceBytes: Long,
    )
}
