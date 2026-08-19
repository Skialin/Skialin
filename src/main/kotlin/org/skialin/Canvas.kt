package org.skialin

import org.skialin.impl.NativeLoader

class Canvas internal constructor(
    internal val ptr: Long,
) {
    fun clear(color: Color) = CanvasNative.nClear(ptr, color)

    fun drawColor(
        color: Color,
        mode: BlendMode = BlendMode.SRC_OVER,
    ) = CanvasNative.nDrawColor(ptr, color, mode.ordinal)

    fun drawPaint(paint: Paint) = CanvasNative.nDrawPaint(ptr, paint.nativePtr)

    fun drawLine(
        p0: Point,
        p1: Point,
        paint: Paint,
    ) = CanvasNative.nDrawLine(ptr, p0.x, p0.y, p1.x, p1.y, paint.nativePtr)

    fun drawRect(
        rect: Rect,
        paint: Paint,
    ) = CanvasNative.nDrawRect(ptr, rect.left, rect.top, rect.right, rect.bottom, paint.nativePtr)

    fun drawOval(
        rect: Rect,
        paint: Paint,
    ) = CanvasNative.nDrawOval(ptr, rect.left, rect.top, rect.right, rect.bottom, paint.nativePtr)

    fun drawCircle(
        center: Point,
        radius: Float,
        paint: Paint,
    ) = CanvasNative.nDrawCircle(ptr, center.x, center.y, radius, paint.nativePtr)

    fun drawPath(
        path: Path,
        paint: Paint,
    ) = CanvasNative.nDrawPath(ptr, path.nativePtr, paint.nativePtr)

    fun drawTextBlob(
        blob: TextBlob,
        x: Float,
        y: Float,
        paint: Paint,
    ) = CanvasNative.nDrawTextBlob(ptr, blob.nativePtr, x, y, paint.nativePtr)

    fun save(): Int = CanvasNative.nSave(ptr)

    fun restore() = CanvasNative.nRestore(ptr)

    fun restoreToCount(saveCount: Int) = CanvasNative.nRestoreToCount(ptr, saveCount)

    fun translate(
        dx: Float,
        dy: Float,
    ) = CanvasNative.nTranslate(ptr, dx, dy)

    fun scale(
        sx: Float,
        sy: Float,
    ) = CanvasNative.nScale(ptr, sx, sy)

    fun rotate(degrees: Float) = CanvasNative.nRotate(ptr, degrees)

    fun clipRect(
        rect: Rect,
        op: ClipOp = ClipOp.INTERSECT,
    ) = CanvasNative.nClipRect(ptr, rect.left, rect.top, rect.right, rect.bottom, op.ordinal)

    fun clipPath(
        path: Path,
        op: ClipOp = ClipOp.INTERSECT,
    ) = CanvasNative.nClipPath(ptr, path.nativePtr, op.ordinal)

    fun skew(
        sx: Float,
        sy: Float,
    ) = CanvasNative.nSkew(ptr, sx, sy)

    fun resetMatrix() = CanvasNative.nResetMatrix(ptr)

    fun setMatrix(matrix: Matrix33) = CanvasNative.nSetMatrix(ptr, matrix.values)

    fun getTotalMatrix(): Matrix33 = Matrix33(CanvasNative.nTotalMatrix(ptr))

    fun quickReject(rect: Rect): Boolean = CanvasNative.nQuickRejectRect(ptr, rect.left, rect.top, rect.right, rect.bottom)

    fun quickReject(path: Path): Boolean = CanvasNative.nQuickRejectPath(ptr, path.nativePtr)

    fun drawRoundRect(
        rect: Rect,
        rx: Float,
        ry: Float,
        paint: Paint,
    ) = CanvasNative.nDrawRoundRect(ptr, rect.left, rect.top, rect.right, rect.bottom, rx, ry, paint.nativePtr)

    fun drawArc(
        oval: Rect,
        startAngle: Float,
        sweepAngle: Float,
        useCenter: Boolean,
        paint: Paint,
    ) = CanvasNative.nDrawArc(ptr, oval.left, oval.top, oval.right, oval.bottom, startAngle, sweepAngle, useCenter, paint.nativePtr)

    fun drawPoints(
        mode: PointMode,
        points: Array<Point>,
        paint: Paint,
    ) {
        val flat = FloatArray(points.size * 2)
        points.forEachIndexed { i, p ->
            flat[i * 2] = p.x
            flat[i * 2 + 1] = p.y
        }
        CanvasNative.nDrawPoints(ptr, mode.ordinal, flat, paint.nativePtr)
    }

    fun drawImage(
        image: Image,
        x: Float,
        y: Float,
        sampling: SamplingOptions = SamplingOptions.NEAREST,
        paint: Paint? = null,
    ) = CanvasNative.nDrawImage(
        ptr,
        image.nativePtr,
        x,
        y,
        sampling.maxAniso,
        sampling.useCubic,
        sampling.cubicB ?: 0f,
        sampling.cubicC ?: 0f,
        sampling.filter.ordinal,
        sampling.mipmap.ordinal,
        paint?.nativePtr ?: 0L,
    )

    /** [src] defaults to the whole image when `null`. */
    fun drawImageRect(
        image: Image,
        dst: Rect,
        src: Rect? = null,
        sampling: SamplingOptions = SamplingOptions.NEAREST,
        paint: Paint? = null,
        constraint: SrcRectConstraint = SrcRectConstraint.STRICT,
    ) = CanvasNative.nDrawImageRect(
        ptr,
        image.nativePtr,
        src?.let { floatArrayOf(it.left, it.top, it.right, it.bottom) },
        dst.left,
        dst.top,
        dst.right,
        dst.bottom,
        sampling.maxAniso,
        sampling.useCubic,
        sampling.cubicB ?: 0f,
        sampling.cubicC ?: 0f,
        sampling.filter.ordinal,
        sampling.mipmap.ordinal,
        paint?.nativePtr ?: 0L,
        constraint.ordinal,
    )

    /** Draws [image] as a 9-patch: [center] scales to fill the interior of [dst], while the
     * surrounding edges/corners scale only along one axis (or not at all). */
    fun drawImageNine(
        image: Image,
        center: IRect,
        dst: Rect,
        filter: FilterMode = FilterMode.NEAREST,
        paint: Paint? = null,
    ) = CanvasNative.nDrawImageNine(
        ptr,
        image.nativePtr,
        intArrayOf(center.left, center.top, center.right, center.bottom),
        dst.left,
        dst.top,
        dst.right,
        dst.bottom,
        filter.ordinal,
        paint?.nativePtr ?: 0L,
    )

    /** Draws a Coons patch: [cubics] is the 12-point boundary (4 cubic Bezier edges sharing
     * corner points), [colors] are the 4 corner colors, [texCoords] optionally maps a source
     * image's 4 corners onto the patch via a shader on [paint]. */
    fun drawPatch(
        cubics: Array<Point>,
        colors: Array<Color>,
        texCoords: Array<Point>? = null,
        mode: BlendMode = BlendMode.SRC_OVER,
        paint: Paint,
    ) {
        require(cubics.size == 12) { "cubics must have 12 points" }
        require(colors.size == 4) { "colors must have 4 entries" }
        require(texCoords == null || texCoords.size == 4) { "texCoords must have 4 points" }
        val flatCubics = FloatArray(24)
        cubics.forEachIndexed { i, p ->
            flatCubics[i * 2] = p.x
            flatCubics[i * 2 + 1] = p.y
        }
        val flatTex =
            texCoords?.let { coords ->
                FloatArray(8).also { flat ->
                    coords.forEachIndexed { i, p ->
                        flat[i * 2] = p.x
                        flat[i * 2 + 1] = p.y
                    }
                }
            }
        CanvasNative.nDrawPatch(ptr, flatCubics, colors.toIntArray(), flatTex, mode.ordinal, paint.nativePtr)
    }

    /** Attaches a key/value annotation to the document at [rect] (e.g. a hyperlink or named
     * destination when drawing into a PDF-backed canvas); a no-op for raster/GPU canvases. */
    fun drawAnnotation(
        rect: Rect,
        key: String,
        value: Data? = null,
    ) = CanvasNative.nDrawAnnotation(ptr, rect.left, rect.top, rect.right, rect.bottom, key, value?.nativePtr ?: 0L)

    fun drawRRect(
        rrect: RRect,
        paint: Paint,
    ) = CanvasNative.nDrawRRect(ptr, rrect.nativePtr, paint.nativePtr)

    fun drawPicture(picture: Picture) = CanvasNative.nDrawPicture(ptr, picture.nativePtr)

    /** Draws the ring between [outer] and [inner]; [inner] must be contained within [outer]. */
    fun drawDRRect(
        outer: RRect,
        inner: RRect,
        paint: Paint,
    ) = CanvasNative.nDrawDRRect(ptr, outer.nativePtr, inner.nativePtr, paint.nativePtr)

    fun clipRRect(
        rrect: RRect,
        op: ClipOp = ClipOp.INTERSECT,
    ) = CanvasNative.nClipRRect(ptr, rrect.nativePtr, op.ordinal)

    fun drawRegion(
        region: Region,
        paint: Paint,
    ) = CanvasNative.nDrawRegion(ptr, region.nativePtr, paint.nativePtr)

    fun clipRegion(
        region: Region,
        op: ClipOp = ClipOp.INTERSECT,
    ) = CanvasNative.nClipRegion(ptr, region.nativePtr, op.ordinal)

    fun drawVertices(
        vertices: Vertices,
        mode: BlendMode,
        paint: Paint,
    ) = CanvasNative.nDrawVertices(ptr, vertices.nativePtr, mode.ordinal, paint.nativePtr)

    /** Concatenates a 4x4 local-to-device transform onto the canvas's current matrix. */
    fun concat44(matrix: M44) = CanvasNative.nConcat44(ptr, matrix.nativePtr)

    /**
     * Saves the canvas state, then redirects drawing to a new layer. [bounds], if given, is
     * a hint for the layer's extent. Returns the new save count, for [restoreToCount].
     */
    fun saveLayer(
        bounds: Rect? = null,
        paint: Paint? = null,
    ): Int = CanvasNative.nSaveLayer(ptr, bounds?.let { floatArrayOf(it.left, it.top, it.right, it.bottom) }, paint?.nativePtr ?: 0L)

    /**
     * Like [saveLayer], but [backdrop], if given, filters the current layer's content before
     * it's drawn into the new one (instead of the new layer starting out transparent-black).
     */
    fun saveLayer(
        bounds: Rect? = null,
        paint: Paint? = null,
        backdrop: ImageFilter?,
        flags: Int = 0,
    ): Int =
        CanvasNative.nSaveLayerWithBackdrop(
            ptr,
            bounds?.let { floatArrayOf(it.left, it.top, it.right, it.bottom) },
            paint?.nativePtr ?: 0L,
            backdrop?.nativePtr ?: 0L,
            flags,
        )

    /** Runs [block] between [save] and [restore]. */
    inline fun withSave(block: Canvas.() -> Unit) {
        save()
        try {
            block()
        } finally {
            restore()
        }
    }
}

private object CanvasNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nClear(
        ptr: Long,
        color: Int,
    )

    external fun nDrawColor(
        ptr: Long,
        color: Int,
        mode: Int,
    )

    external fun nDrawPaint(
        ptr: Long,
        paintPtr: Long,
    )

    external fun nDrawLine(
        ptr: Long,
        x0: Float,
        y0: Float,
        x1: Float,
        y1: Float,
        paintPtr: Long,
    )

    external fun nDrawRect(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        paintPtr: Long,
    )

    external fun nDrawOval(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        paintPtr: Long,
    )

    external fun nDrawCircle(
        ptr: Long,
        cx: Float,
        cy: Float,
        radius: Float,
        paintPtr: Long,
    )

    external fun nDrawPath(
        ptr: Long,
        pathPtr: Long,
        paintPtr: Long,
    )

    external fun nDrawTextBlob(
        ptr: Long,
        blobPtr: Long,
        x: Float,
        y: Float,
        paintPtr: Long,
    )

    external fun nSave(ptr: Long): Int

    external fun nRestore(ptr: Long)

    external fun nRestoreToCount(
        ptr: Long,
        saveCount: Int,
    )

    external fun nTranslate(
        ptr: Long,
        dx: Float,
        dy: Float,
    )

    external fun nScale(
        ptr: Long,
        sx: Float,
        sy: Float,
    )

    external fun nRotate(
        ptr: Long,
        degrees: Float,
    )

    external fun nClipRect(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        op: Int,
    )

    external fun nClipPath(
        ptr: Long,
        pathPtr: Long,
        op: Int,
    )

    external fun nSkew(
        ptr: Long,
        sx: Float,
        sy: Float,
    )

    external fun nResetMatrix(ptr: Long)

    external fun nSetMatrix(
        ptr: Long,
        matrix: FloatArray,
    )

    external fun nTotalMatrix(ptr: Long): FloatArray

    external fun nQuickRejectRect(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
    ): Boolean

    external fun nQuickRejectPath(
        ptr: Long,
        pathPtr: Long,
    ): Boolean

    external fun nDrawRoundRect(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        rx: Float,
        ry: Float,
        paintPtr: Long,
    )

    external fun nDrawArc(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        startAngle: Float,
        sweepAngle: Float,
        useCenter: Boolean,
        paintPtr: Long,
    )

    external fun nDrawPoints(
        ptr: Long,
        mode: Int,
        points: FloatArray,
        paintPtr: Long,
    )

    external fun nDrawImage(
        ptr: Long,
        imagePtr: Long,
        x: Float,
        y: Float,
        maxAniso: Int,
        useCubic: Boolean,
        cubicB: Float,
        cubicC: Float,
        filter: Int,
        mipmap: Int,
        paintPtr: Long,
    )

    external fun nDrawImageRect(
        ptr: Long,
        imagePtr: Long,
        src: FloatArray?,
        dstLeft: Float,
        dstTop: Float,
        dstRight: Float,
        dstBottom: Float,
        maxAniso: Int,
        useCubic: Boolean,
        cubicB: Float,
        cubicC: Float,
        filter: Int,
        mipmap: Int,
        paintPtr: Long,
        constraint: Int,
    )

    external fun nSaveLayer(
        ptr: Long,
        bounds: FloatArray?,
        paintPtr: Long,
    ): Int

    external fun nSaveLayerWithBackdrop(
        ptr: Long,
        bounds: FloatArray?,
        paintPtr: Long,
        backdropPtr: Long,
        flags: Int,
    ): Int

    external fun nDrawImageNine(
        ptr: Long,
        imagePtr: Long,
        center: IntArray,
        dstLeft: Float,
        dstTop: Float,
        dstRight: Float,
        dstBottom: Float,
        filter: Int,
        paintPtr: Long,
    )

    external fun nDrawPatch(
        ptr: Long,
        cubics: FloatArray,
        colors: IntArray,
        texCoords: FloatArray?,
        mode: Int,
        paintPtr: Long,
    )

    external fun nDrawAnnotation(
        ptr: Long,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
        key: String,
        valuePtr: Long,
    )

    external fun nDrawRRect(
        ptr: Long,
        rrectPtr: Long,
        paintPtr: Long,
    )

    external fun nDrawPicture(
        ptr: Long,
        picturePtr: Long,
    )

    external fun nDrawDRRect(
        ptr: Long,
        outerPtr: Long,
        innerPtr: Long,
        paintPtr: Long,
    )

    external fun nClipRRect(
        ptr: Long,
        rrectPtr: Long,
        op: Int,
    )

    external fun nDrawRegion(
        ptr: Long,
        regionPtr: Long,
        paintPtr: Long,
    )

    external fun nClipRegion(
        ptr: Long,
        regionPtr: Long,
        op: Int,
    )

    external fun nDrawVertices(
        ptr: Long,
        verticesPtr: Long,
        mode: Int,
        paintPtr: Long,
    )

    external fun nConcat44(
        ptr: Long,
        matrixPtr: Long,
    )
}
