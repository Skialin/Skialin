package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/**
 * [Surface] is responsible for managing the pixels that a canvas draws into.
 * 
 * The pixels can be allocated either in CPU memory (a raster surface) or on the GPU. [Surface] takes care of allocating a [Canvas] that will draw into the surface. Call [Surface.canvas] to use that canvas (but don't delete it, it is owned by the surface). [Surface] always has non-zero dimensions. If there is a request for a new surface, and either of the requested dimensions are zero, then nullptr will be returned.
 * 
 * Clients should not subclass [Surface] as there is a lot of internal machinery that is not publicly accessible.
 */
class Surface private constructor(
    ptr: Long,
) : Managed(ptr, SurfaceNative::nRelease) {
    /**
     * Returns [Canvas] that draws into [Surface].
     * 
     * Subsequent calls return the same [Canvas]. [Canvas] returned is managed and owned by [Surface], and is deleted when [Surface] is deleted.
     */
    val canvas get() = Canvas(SurfaceNative.nGetCanvas(nativePtr))

    fun makeImageSnapshot(): Image? {
        val ptr = SurfaceNative.nMakeImageSnapshot(nativePtr)
        return if (ptr == 0L) null else Image(ptr)
    }

    fun makeImageSnapshotArea(area: IRect): Image? {
        val ptr = SurfaceNative.nMakeImageSnapshotArea(nativePtr, area.left, area.top, area.right, area.bottom)
        return if (ptr == 0L) null else Image(ptr)
    }

    val width: Int get() = SurfaceNative.nWidth(nativePtr)

    val height: Int get() = SurfaceNative.nHeight(nativePtr)

    val imageInfo: ImageInfo get() = ImageInfo(SurfaceNative.nImageInfo(nativePtr))

    fun notifyContentWillChange(mode: ContentChangeMode = ContentChangeMode.RETAIN) =
        SurfaceNative.nNotifyContentWillChange(nativePtr, mode.ordinal)

    fun flush() = SurfaceNative.nFlush(nativePtr)

    fun draw(
        canvas: Canvas,
        x: Float,
        y: Float,
        paint: Paint? = null,
    ) = SurfaceNative.nDraw(nativePtr, canvas.ptr, x, y, paint?.nativePtr ?: 0L)

    enum class ContentChangeMode { DISCARD, RETAIN }

    companion object {
        fun makeRasterN32Premul(
            width: Int,
            height: Int,
        ): Surface? {
            val ptr = SurfaceNative.nMakeRasterN32Premul(width, height)
            return if (ptr == 0L) null else Surface(ptr)
        }

        fun makeRaster(info: ImageInfo): Surface? {
            val ptr = SurfaceNative.nMakeRaster(info.nativePtr)
            return if (ptr == 0L) null else Surface(ptr)
        }

        fun makeRenderTarget(
            context: DirectContext,
            budgeted: Boolean,
            info: ImageInfo,
            sampleCount: Int,
            surfaceOrigin: SurfaceOrigin,
            surfaceProps: SurfaceProps? = null,
            shouldCreateWithMips: Boolean = false,
            isProtected: Boolean = false,
        ): Surface? {
            val ptr =
                SurfaceNative.nMakeRenderTarget(
                    context.nativePtr,
                    budgeted,
                    info.nativePtr,
                    sampleCount,
                    surfaceOrigin.ordinal,
                    surfaceProps?.nativePtr ?: 0L,
                    shouldCreateWithMips,
                    isProtected,
                )
            return if (ptr == 0L) null else Surface(ptr)
        }

        fun wrapBackendTexture(
            context: DirectContext,
            backendTexture: BackendTexture,
            origin: SurfaceOrigin,
            sampleCnt: Int,
            colorType: ColorType,
            colorSpace: ColorSpace? = null,
            surfaceProps: SurfaceProps? = null,
        ): Surface? {
            val ptr =
                SurfaceNative.nWrapBackendTexture(
                    context.nativePtr,
                    backendTexture.nativePtr,
                    origin.ordinal,
                    sampleCnt,
                    colorType.ordinal,
                    colorSpace?.nativePtr ?: 0L,
                    surfaceProps?.nativePtr ?: 0L,
                )
            return if (ptr == 0L) null else Surface(ptr)
        }

        fun wrapBackendRenderTarget(
            context: DirectContext,
            backendRenderTarget: BackendRenderTarget,
            origin: SurfaceOrigin,
            colorType: ColorType,
            colorSpace: ColorSpace? = null,
            surfaceProps: SurfaceProps? = null,
        ): Surface? {
            val ptr =
                SurfaceNative.nWrapBackendRenderTarget(
                    context.nativePtr,
                    backendRenderTarget.nativePtr,
                    origin.ordinal,
                    colorType.ordinal,
                    colorSpace?.nativePtr ?: 0L,
                    surfaceProps?.nativePtr ?: 0L,
                )
            return if (ptr == 0L) null else Surface(ptr)
        }

        fun makeGraphiteRenderTarget(
            recorder: GraphiteRecorder,
            info: ImageInfo,
            mipmapped: Boolean = false,
            surfaceProps: SurfaceProps? = null,
        ): Surface? {
            val ptr = SurfaceNative.nMakeGraphiteRenderTarget(recorder.nativePtr, info.nativePtr, mipmapped, surfaceProps?.nativePtr ?: 0L)
            return if (ptr == 0L) null else Surface(ptr)
        }

        fun wrapGraphiteBackendTexture(
            recorder: GraphiteRecorder,
            backendTexture: GraphiteBackendTexture,
            colorType: ColorType,
            colorSpace: ColorSpace? = null,
            surfaceProps: SurfaceProps? = null,
        ): Surface? {
            val ptr =
                SurfaceNative.nWrapGraphiteBackendTexture(
                    recorder.nativePtr,
                    backendTexture.nativePtr,
                    colorType.ordinal,
                    colorSpace?.nativePtr ?: 0L,
                    surfaceProps?.nativePtr ?: 0L,
                )
            return if (ptr == 0L) null else Surface(ptr)
        }
    }
}

private object SurfaceNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeRasterN32Premul(
        width: Int,
        height: Int,
    ): Long

    external fun nMakeRaster(infoPtr: Long): Long

    external fun nMakeRenderTarget(
        contextPtr: Long,
        budgeted: Boolean,
        infoPtr: Long,
        sampleCount: Int,
        surfaceOrigin: Int,
        surfacePropsPtr: Long,
        shouldCreateWithMips: Boolean,
        isProtected: Boolean,
    ): Long

    external fun nWrapBackendTexture(
        contextPtr: Long,
        backendTexturePtr: Long,
        origin: Int,
        sampleCnt: Int,
        colorType: Int,
        colorSpacePtr: Long,
        surfacePropsPtr: Long,
    ): Long

    external fun nWrapBackendRenderTarget(
        contextPtr: Long,
        backendRenderTargetPtr: Long,
        origin: Int,
        colorType: Int,
        colorSpacePtr: Long,
        surfacePropsPtr: Long,
    ): Long

    external fun nMakeGraphiteRenderTarget(
        recorderPtr: Long,
        infoPtr: Long,
        mipmapped: Boolean,
        surfacePropsPtr: Long,
    ): Long

    external fun nWrapGraphiteBackendTexture(
        recorderPtr: Long,
        backendTexturePtr: Long,
        colorType: Int,
        colorSpacePtr: Long,
        surfacePropsPtr: Long,
    ): Long

    external fun nRelease(ptr: Long)

    external fun nGetCanvas(ptr: Long): Long

    external fun nMakeImageSnapshot(ptr: Long): Long

    external fun nMakeImageSnapshotArea(
        ptr: Long,
        left: Int,
        top: Int,
        right: Int,
        bottom: Int,
    ): Long

    external fun nWidth(ptr: Long): Int

    external fun nHeight(ptr: Long): Int

    external fun nImageInfo(ptr: Long): Long

    external fun nNotifyContentWillChange(
        ptr: Long,
        mode: Int,
    )

    external fun nFlush(ptr: Long)

    external fun nDraw(
        ptr: Long,
        canvasPtr: Long,
        x: Float,
        y: Float,
        paintPtr: Long,
    )
}
