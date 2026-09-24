package org.skialin

import org.skialin.impl.NativeLoader

/**
 * Wraps a skgpu::graphite::Context (Vulkan, or D3D12 through Dawn on
 * Windows -- see [makeDawnD3D12]). Thread-safe and
 * long-lived, unlike a [GraphiteRecorder] made from it.
 *
 * Doesn't extend [org.skialin.impl.Managed]: same rationale as
 * [DirectContext] -- native teardown shouldn't run on an arbitrary
 * Cleaner thread. [close] must be called explicitly.
 */
class GraphiteContext private constructor(
    ptr: Long,
) : AutoCloseable {
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
    fun insertRecording(
        recording: GraphiteRecording,
        targetSurface: Surface,
    ): Int = GraphiteContextNative.nInsertRecording(nativePtr, recording.nativePtr, targetSurface.nativePtr)

    fun submit(syncToCpu: Boolean = false): Boolean = GraphiteContextNative.nSubmit(nativePtr, syncToCpu)

    override fun close() {
        if (ptr != 0L) {
            GraphiteContextNative.nRelease(ptr)
            ptr = 0L
        }
    }

    companion object {
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
        ): GraphiteContext? {
            val ptr =
                GraphiteContextNative.nMakeVulkan(
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
            return if (ptr == 0L) null else GraphiteContext(ptr)
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
        ): GraphiteContext? {
            val ptr =
                GraphiteContextNative.nMakeVulkanWithGetProc(
                    instance,
                    physicalDevice,
                    device,
                    queue,
                    graphicsQueueIndex,
                    maxApiVersion,
                    protectedContext,
                    getProc,
                )
            return if (ptr == 0L) null else GraphiteContext(ptr)
        }

        /**
         * Dawn/D3D12 only. Unlike [makeVulkan], Dawn creates and owns the ID3D12Device itself --
         * there's no way to hand it a caller-created one -- so this enumerates D3D12 adapters on
         * its own and picks [adapterIndex] (0 for the default/first). Always null off
         * Windows, where Dawn isn't built.
         *
         * The returned [DawnD3D12Context.d3d12Device]/[d3d12CommandQueue] are the raw COM objects
         * Dawn is driving underneath, valid for as long as the context is open; a caller that
         * wants zero-copy interop creates its own ID3D12Resources on that device and imports them
         * with [makeD3D12BackendTexture]. Null on failure.
         */
        fun makeDawnD3D12(adapterIndex: Int = 0): DawnD3D12Context? {
            val result = GraphiteContextNative.nMakeDawnD3D12(adapterIndex)
            val ptr = result[0]
            return if (ptr == 0L) null else DawnD3D12Context(GraphiteContext(ptr), result[1], result[2])
        }
    }

    /**
     * Wraps a caller-owned ID3D12Resource (created on a [DawnD3D12Context.d3d12Device] from the
     * same [makeDawnD3D12] call this context came from) as a [GraphiteBackendTexture], via Dawn's
     * SharedTextureMemory import -- no copy, no shared handle, since the resource already lives on
     * Dawn's own device. [dawnTextureFormat]/[dawnTextureUsage] are `wgpu::TextureFormat`/
     * `wgpu::TextureUsage` values, not `DXGI_FORMAT`.
     *
     * There is no fence-based synchronization yet, so the caller must make sure nothing else
     * touches [d3d12Resource] while this texture is in use. Returns null if this context wasn't
     * made with [makeDawnD3D12], or the import fails.
     */
    @Suppress("LongParameterList")
    fun makeD3D12BackendTexture(
        d3d12Resource: Long,
        width: Int,
        height: Int,
        sampleCount: Int,
        mipmapped: Boolean,
        dawnTextureFormat: Int,
        dawnTextureUsage: Int,
    ): GraphiteBackendTexture? {
        val ptr =
            GraphiteContextNative.nMakeD3D12BackendTexture(
                nativePtr,
                d3d12Resource,
                width,
                height,
                sampleCount,
                mipmapped,
                dawnTextureFormat,
                dawnTextureUsage,
            )
        return if (ptr == 0L) null else GraphiteBackendTexture(ptr)
    }
}

/**
 * [context] was made with [GraphiteContext.makeDawnD3D12]; [d3d12Device]/[d3d12CommandQueue] are
 * the raw `ID3D12Device*`/`ID3D12CommandQueue*` Dawn created and is using for it.
 */
class DawnD3D12Context(
    val context: GraphiteContext,
    val d3d12Device: Long,
    val d3d12CommandQueue: Long,
)

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

    /** [0] is the context pointer (0 on failure), [1] is the ID3D12Device*, [2] the ID3D12CommandQueue*. */
    external fun nMakeDawnD3D12(adapterIndex: Int): LongArray

    @Suppress("LongParameterList")
    external fun nMakeD3D12BackendTexture(
        ptr: Long,
        d3d12Resource: Long,
        width: Int,
        height: Int,
        sampleCount: Int,
        mipmapped: Boolean,
        dawnTextureFormat: Int,
        dawnTextureUsage: Int,
    ): Long

    external fun nMakeRecorder(ptr: Long): Long

    external fun nInsertRecording(
        ptr: Long,
        recordingPtr: Long,
        targetSurfacePtr: Long,
    ): Int

    external fun nSubmit(
        ptr: Long,
        syncToCpu: Boolean,
    ): Boolean
}
