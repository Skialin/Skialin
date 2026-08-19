package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class Surface private constructor(
    ptr: Long,
) : Managed(ptr, SurfaceNative::nRelease) {
    fun canvas(): Canvas = Canvas(SurfaceNative.nGetCanvas(nativePtr))

    fun imageSnapshot(): Image? {
        val ptr = SurfaceNative.nMakeImageSnapshot(nativePtr)
        return if (ptr == 0L) null else Image(ptr)
    }

    fun imageSnapshot(area: IRect): Image? {
        val ptr = SurfaceNative.nMakeImageSnapshotArea(nativePtr, area.left, area.top, area.right, area.bottom)
        return if (ptr == 0L) null else Image(ptr)
    }

    val width: Int get() = SurfaceNative.nWidth(nativePtr)

    val height: Int get() = SurfaceNative.nHeight(nativePtr)

    val imageInfo: ImageInfo get() = ImageInfo(SurfaceNative.nImageInfo(nativePtr))

    fun notifyContentWillChange(mode: ContentChangeMode = ContentChangeMode.RETAIN) = SurfaceNative.nNotifyContentWillChange(nativePtr, mode.ordinal)

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

        /** Must be called on the thread [context]'s GL context is current on. */
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

        /**
         * Wraps an existing GPU texture as a render target instead of
         * allocating a new one. Must be called on the thread [context]'s
         * GL context is current on. [backendTexture] must outlive the
         * returned Surface.
         */
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

        fun makeGraphiteRenderTarget(
            recorder: GraphiteRecorder,
            info: ImageInfo,
            mipmapped: Boolean = false,
            surfaceProps: SurfaceProps? = null,
        ): Surface? {
            val ptr = SurfaceNative.nMakeGraphiteRenderTarget(recorder.nativePtr, info.nativePtr, mipmapped, surfaceProps?.nativePtr ?: 0L)
            return if (ptr == 0L) null else Surface(ptr)
        }

        /** [backendTexture] must outlive the returned Surface. */
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
