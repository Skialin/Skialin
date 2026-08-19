package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** A [Typeface] plus size, scale, skew, and rendering settings. Mirrors Skia's `SkFont`. */
class Font internal constructor(
    ptr: Long,
) : Managed(ptr, FontNative::nRelease) {
    enum class Edging { ALIAS, ANTI_ALIAS, SUBPIXEL_ANTI_ALIAS }

    enum class Hinting { NONE, SLIGHT, NORMAL, FULL }

    constructor() : this(FontNative.nMakeDefault())
    constructor(typeface: Typeface, size: Float) : this(FontNative.nMakeWithTypeface(typeface.nativePtr, size))

    var typeface: Typeface?
        get() = FontNative.nTypeface(nativePtr).takeIf { it != 0L }?.let { Typeface(it) }
        set(value) = FontNative.nSetTypeface(nativePtr, value?.nativePtr ?: 0L)

    var size: Float
        get() = FontNative.nSize(nativePtr)
        set(value) = FontNative.nSetSize(nativePtr, value)

    var scaleX: Float
        get() = FontNative.nScaleX(nativePtr)
        set(value) = FontNative.nSetScaleX(nativePtr, value)

    var skewX: Float
        get() = FontNative.nSkewX(nativePtr)
        set(value) = FontNative.nSetSkewX(nativePtr, value)

    var edging: Edging
        get() = Edging.entries[FontNative.nEdging(nativePtr)]
        set(value) = FontNative.nSetEdging(nativePtr, value.ordinal)

    var hinting: Hinting
        get() = Hinting.entries[FontNative.nHinting(nativePtr)]
        set(value) = FontNative.nSetHinting(nativePtr, value.ordinal)

    var isSubpixel: Boolean
        get() = FontNative.nIsSubpixel(nativePtr)
        set(value) = FontNative.nSetSubpixel(nativePtr, value)

    var isEmbolden: Boolean
        get() = FontNative.nIsEmbolden(nativePtr)
        set(value) = FontNative.nSetEmbolden(nativePtr, value)

    var isLinearMetrics: Boolean
        get() = FontNative.nIsLinearMetrics(nativePtr)
        set(value) = FontNative.nSetLinearMetrics(nativePtr, value)

    var isForceAutoHinting: Boolean
        get() = FontNative.nIsForceAutoHinting(nativePtr)
        set(value) = FontNative.nSetForceAutoHinting(nativePtr, value)

    var isEmbeddedBitmaps: Boolean
        get() = FontNative.nIsEmbeddedBitmaps(nativePtr)
        set(value) = FontNative.nSetEmbeddedBitmaps(nativePtr, value)

    var isBaselineSnap: Boolean
        get() = FontNative.nIsBaselineSnap(nativePtr)
        set(value) = FontNative.nSetBaselineSnap(nativePtr, value)

    val spacing: Float get() = FontNative.nSpacing(nativePtr)

    fun unicharToGlyph(unichar: Int): Int = FontNative.nUnicharToGlyph(nativePtr, unichar)

    /** Converts `text` into glyph indices, using this font's typeface's default character-to-glyph mapping. */
    fun textToGlyphs(text: String): ShortArray = FontNative.nTextToGlyphs(nativePtr, text)

    fun measureText(text: String): Float = FontNative.nMeasureText(nativePtr, text)

    /** The advance width for each glyph in [glyphs]. */
    fun widths(glyphs: ShortArray): FloatArray = FontNative.nWidths(nativePtr, glyphs)

    fun metrics(): FontMetrics {
        val out = FloatArray(11)
        FontNative.nMetrics(nativePtr, out)
        return FontMetrics(out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7], out[8], out[9], out[10])
    }

    fun getBounds(
        glyphs: ShortArray,
        paint: Paint? = null,
    ): Array<Rect> {
        val flat = FontNative.nBounds(nativePtr, glyphs, paint?.nativePtr ?: 0L)
        return Array(flat.size / 4) { i -> Rect(flat[i * 4], flat[i * 4 + 1], flat[i * 4 + 2], flat[i * 4 + 3]) }
    }

    fun getPositions(
        glyphs: ShortArray,
        origin: Point = Point(0f, 0f),
    ): Array<Point> {
        val flat = FontNative.nPositions(nativePtr, glyphs, origin.x, origin.y)
        return Array(flat.size / 2) { i -> Point(flat[i * 2], flat[i * 2 + 1]) }
    }

    fun getXPositions(
        glyphs: ShortArray,
        origin: Float = 0f,
    ): FloatArray = FontNative.nXPositions(nativePtr, glyphs, origin)

    fun getPath(glyph: Short): Path? {
        val ptr = FontNative.nGetPath(nativePtr, glyph)
        return if (ptr == 0L) null else Path(ptr)
    }

    fun getPaths(glyphs: ShortArray): Array<Path?> = Array(glyphs.size) { i -> getPath(glyphs[i]) }

    fun makeWithSize(size: Float): Font = Font(FontNative.nMakeWithSize(nativePtr, size))
}

private object FontNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeDefault(): Long

    external fun nMakeWithTypeface(
        typefacePtr: Long,
        size: Float,
    ): Long

    external fun nRelease(ptr: Long)

    external fun nTypeface(ptr: Long): Long

    external fun nSetTypeface(
        ptr: Long,
        typefacePtr: Long,
    )

    external fun nSize(ptr: Long): Float

    external fun nSetSize(
        ptr: Long,
        size: Float,
    )

    external fun nScaleX(ptr: Long): Float

    external fun nSetScaleX(
        ptr: Long,
        scaleX: Float,
    )

    external fun nSkewX(ptr: Long): Float

    external fun nSetSkewX(
        ptr: Long,
        skewX: Float,
    )

    external fun nEdging(ptr: Long): Int

    external fun nSetEdging(
        ptr: Long,
        edging: Int,
    )

    external fun nHinting(ptr: Long): Int

    external fun nSetHinting(
        ptr: Long,
        hinting: Int,
    )

    external fun nIsSubpixel(ptr: Long): Boolean

    external fun nSetSubpixel(
        ptr: Long,
        subpixel: Boolean,
    )

    external fun nIsEmbolden(ptr: Long): Boolean

    external fun nSetEmbolden(
        ptr: Long,
        embolden: Boolean,
    )

    external fun nIsLinearMetrics(ptr: Long): Boolean

    external fun nSetLinearMetrics(
        ptr: Long,
        linearMetrics: Boolean,
    )

    external fun nIsForceAutoHinting(ptr: Long): Boolean

    external fun nSetForceAutoHinting(
        ptr: Long,
        forceAutoHinting: Boolean,
    )

    external fun nIsEmbeddedBitmaps(ptr: Long): Boolean

    external fun nSetEmbeddedBitmaps(
        ptr: Long,
        embeddedBitmaps: Boolean,
    )

    external fun nIsBaselineSnap(ptr: Long): Boolean

    external fun nSetBaselineSnap(
        ptr: Long,
        baselineSnap: Boolean,
    )

    external fun nUnicharToGlyph(
        ptr: Long,
        unichar: Int,
    ): Int

    external fun nTextToGlyphs(
        ptr: Long,
        text: String,
    ): ShortArray

    external fun nMeasureText(
        ptr: Long,
        text: String,
    ): Float

    external fun nWidths(
        ptr: Long,
        glyphs: ShortArray,
    ): FloatArray

    external fun nMetrics(
        ptr: Long,
        out: FloatArray,
    )

    external fun nSpacing(ptr: Long): Float

    external fun nBounds(
        ptr: Long,
        glyphs: ShortArray,
        paintPtr: Long,
    ): FloatArray

    external fun nPositions(
        ptr: Long,
        glyphs: ShortArray,
        originX: Float,
        originY: Float,
    ): FloatArray

    external fun nXPositions(
        ptr: Long,
        glyphs: ShortArray,
        origin: Float,
    ): FloatArray

    external fun nGetPath(
        ptr: Long,
        glyph: Short,
    ): Long

    external fun nMakeWithSize(
        ptr: Long,
        size: Float,
    ): Long
}
