package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class TextStyle internal constructor(
    ptr: Long,
) : Managed(ptr, TextStyleNative::nRelease) {
    object TextDecoration {
        const val NONE = 0x0
        const val UNDERLINE = 0x1
        const val OVERLINE = 0x2
        const val LINE_THROUGH = 0x4
    }

    enum class DecorationMode { GAPS, THROUGH }

    enum class DecorationStyle { SOLID, DOUBLE, DOTTED, DASHED, WAVY }

    data class Decoration(
        val decoration: Int,
        val mode: DecorationMode,
        val color: Color,
        val style: DecorationStyle,
        val thicknessMultiplier: Float,
    )

    data class Shadow(
        val color: Color,
        val offsetX: Float,
        val offsetY: Float,
        val blurSigma: Double,
    )

    data class FontFeature(
        val name: String,
        val value: Int,
    )

    constructor() : this(TextStyleNative.nNew())

    fun cloneStyle(): TextStyle = TextStyle(TextStyleNative.nClone(nativePtr))

    var color: Color
        get() = TextStyleNative.nColor(nativePtr)
        set(value) = TextStyleNative.nSetColor(nativePtr, value)

    var fontFamilies: List<String>
        get() = TextStyleNative.nFontFamilies(nativePtr).toList()
        set(value) = TextStyleNative.nSetFontFamilies(nativePtr, value.toTypedArray())

    var fontSize: Float
        get() = TextStyleNative.nFontSize(nativePtr)
        set(value) = TextStyleNative.nSetFontSize(nativePtr, value)

    var fontStyle: FontStyle
        get() =
            FontStyle(
                TextStyleNative.nWeight(nativePtr),
                TextStyleNative.nWidth(nativePtr),
                FontStyle.Slant.entries[TextStyleNative.nSlant(nativePtr)],
            )
        set(value) = TextStyleNative.nSetFontStyle(nativePtr, value.weight, value.width, value.slant.ordinal)

    var decoration: Decoration
        get() =
            Decoration(
                TextStyleNative.nDecorationType(nativePtr),
                DecorationMode.entries[TextStyleNative.nDecorationMode(nativePtr)],
                TextStyleNative.nDecorationColor(nativePtr),
                DecorationStyle.entries[TextStyleNative.nDecorationStyle(nativePtr)],
                TextStyleNative.nDecorationThicknessMultiplier(nativePtr),
            )
        set(value) {
            TextStyleNative.nSetDecoration(nativePtr, value.decoration)
            TextStyleNative.nSetDecorationMode(nativePtr, value.mode.ordinal)
            TextStyleNative.nSetDecorationColor(nativePtr, value.color)
            TextStyleNative.nSetDecorationStyle(nativePtr, value.style.ordinal)
            TextStyleNative.nSetDecorationThicknessMultiplier(nativePtr, value.thicknessMultiplier)
        }

    var letterSpacing: Float
        get() = TextStyleNative.nLetterSpacing(nativePtr)
        set(value) = TextStyleNative.nSetLetterSpacing(nativePtr, value)

    var wordSpacing: Float
        get() = TextStyleNative.nWordSpacing(nativePtr)
        set(value) = TextStyleNative.nSetWordSpacing(nativePtr, value)

    var height: Float
        get() = TextStyleNative.nHeight(nativePtr)
        set(value) = TextStyleNative.nSetHeight(nativePtr, value)

    var heightOverride: Boolean
        get() = TextStyleNative.nHeightOverride(nativePtr)
        set(value) = TextStyleNative.nSetHeightOverride(nativePtr, value)

    /** Baseline offset in px, applied on top of the run's normal baseline (e.g. for
     * superscript/subscript positioning). */
    var baselineShift: Float
        get() = TextStyleNative.nBaselineShift(nativePtr)
        set(value) = TextStyleNative.nSetBaselineShift(nativePtr, value)

    /** Whether extra line-height leading is split evenly above/below the run (true) or added
     * entirely below per legacy behavior (false). */
    var halfLeading: Boolean
        get() = TextStyleNative.nHalfLeading(nativePtr)
        set(value) = TextStyleNative.nSetHalfLeading(nativePtr, value)

    enum class FontEdging { ALIAS, ANTI_ALIAS, SUBPIXEL_ANTI_ALIAS }

    enum class FontHinting { NONE, SLIGHT, NORMAL, FULL }

    /** Per-run rasterization edging, forwarded to the `SkFont` skparagraph builds internally
     * for this run. */
    var fontEdging: FontEdging
        get() = FontEdging.entries[TextStyleNative.nFontEdging(nativePtr)]
        set(value) = TextStyleNative.nSetFontEdging(nativePtr, value.ordinal)

    /** Per-run rasterization hinting, forwarded to the `SkFont` skparagraph builds internally
     * for this run. */
    var fontHinting: FontHinting
        get() = FontHinting.entries[TextStyleNative.nFontHinting(nativePtr)]
        set(value) = TextStyleNative.nSetFontHinting(nativePtr, value.ordinal)

    var typeface: Typeface?
        get() = TextStyleNative.nTypeface(nativePtr).takeIf { it != 0L }?.let { Typeface(it) }
        set(value) = TextStyleNative.nSetTypeface(nativePtr, value?.nativePtr ?: 0L)

    var locale: String
        get() = TextStyleNative.nLocale(nativePtr)
        set(value) = TextStyleNative.nSetLocale(nativePtr, value)

    val shadows: List<Shadow>
        get() {
            val flat = TextStyleNative.nShadows(nativePtr)
            return (0 until flat.size / 4).map { i ->
                Shadow(flat[i * 4].toRawBits(), flat[i * 4 + 1], flat[i * 4 + 2], flat[i * 4 + 3].toDouble())
            }
        }

    fun addShadow(shadow: Shadow) = TextStyleNative.nAddShadow(nativePtr, shadow.color, shadow.offsetX, shadow.offsetY, shadow.blurSigma)

    fun addShadows(shadows: List<Shadow>) = shadows.forEach { addShadow(it) }

    fun clearShadows() = TextStyleNative.nResetShadows(nativePtr)

    val fontFeatures: List<FontFeature>
        get() {
            val names = TextStyleNative.nFontFeatureNames(nativePtr)
            val values = TextStyleNative.nFontFeatureValues(nativePtr)
            return names.indices.map { i -> FontFeature(names[i], values[i]) }
        }

    fun addFontFeature(
        name: String,
        value: Int,
    ) = TextStyleNative.nAddFontFeature(nativePtr, name, value)

    fun clearFontFeatures() = TextStyleNative.nResetFontFeatures(nativePtr)

    val hasForeground: Boolean get() = TextStyleNative.nHasForeground(nativePtr)

    val hasBackground: Boolean get() = TextStyleNative.nHasBackground(nativePtr)

    var foreground: Paint?
        get() = TextStyleNative.nGetForegroundPaint(nativePtr).takeIf { it != 0L }?.let { Paint(it) }
        set(value) {
            if (value == null) {
                TextStyleNative.nClearForeground(nativePtr)
            } else {
                TextStyleNative.nSetForegroundPaint(nativePtr, value.nativePtr)
            }
        }

    var background: Paint?
        get() = TextStyleNative.nGetBackgroundPaint(nativePtr).takeIf { it != 0L }?.let { Paint(it) }
        set(value) {
            if (value == null) {
                TextStyleNative.nClearBackground(nativePtr)
            } else {
                TextStyleNative.nSetBackgroundPaint(nativePtr, value.nativePtr)
            }
        }
}

private object TextStyleNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nNew(): Long

    external fun nClone(ptr: Long): Long

    external fun nRelease(ptr: Long)

    external fun nColor(ptr: Long): Int

    external fun nSetColor(
        ptr: Long,
        color: Int,
    )

    external fun nFontFamilies(ptr: Long): Array<String>

    external fun nSetFontFamilies(
        ptr: Long,
        families: Array<String>,
    )

    external fun nFontSize(ptr: Long): Float

    external fun nSetFontSize(
        ptr: Long,
        size: Float,
    )

    external fun nWeight(ptr: Long): Int

    external fun nWidth(ptr: Long): Int

    external fun nSlant(ptr: Long): Int

    external fun nSetFontStyle(
        ptr: Long,
        weight: Int,
        width: Int,
        slant: Int,
    )

    external fun nDecorationType(ptr: Long): Int

    external fun nDecorationMode(ptr: Long): Int

    external fun nDecorationColor(ptr: Long): Int

    external fun nDecorationStyle(ptr: Long): Int

    external fun nDecorationThicknessMultiplier(ptr: Long): Float

    external fun nSetDecoration(
        ptr: Long,
        decoration: Int,
    )

    external fun nSetDecorationMode(
        ptr: Long,
        mode: Int,
    )

    external fun nSetDecorationColor(
        ptr: Long,
        color: Int,
    )

    external fun nSetDecorationStyle(
        ptr: Long,
        style: Int,
    )

    external fun nSetDecorationThicknessMultiplier(
        ptr: Long,
        multiplier: Float,
    )

    external fun nLetterSpacing(ptr: Long): Float

    external fun nSetLetterSpacing(
        ptr: Long,
        letterSpacing: Float,
    )

    external fun nWordSpacing(ptr: Long): Float

    external fun nSetWordSpacing(
        ptr: Long,
        wordSpacing: Float,
    )

    external fun nHeight(ptr: Long): Float

    external fun nSetHeight(
        ptr: Long,
        height: Float,
    )

    external fun nHeightOverride(ptr: Long): Boolean

    external fun nSetHeightOverride(
        ptr: Long,
        heightOverride: Boolean,
    )

    external fun nBaselineShift(ptr: Long): Float

    external fun nSetBaselineShift(
        ptr: Long,
        baselineShift: Float,
    )

    external fun nHalfLeading(ptr: Long): Boolean

    external fun nSetHalfLeading(
        ptr: Long,
        halfLeading: Boolean,
    )

    external fun nFontEdging(ptr: Long): Int

    external fun nSetFontEdging(
        ptr: Long,
        edging: Int,
    )

    external fun nFontHinting(ptr: Long): Int

    external fun nSetFontHinting(
        ptr: Long,
        hinting: Int,
    )

    external fun nTypeface(ptr: Long): Long

    external fun nSetTypeface(
        ptr: Long,
        typefacePtr: Long,
    )

    external fun nLocale(ptr: Long): String

    external fun nSetLocale(
        ptr: Long,
        locale: String,
    )

    external fun nShadows(ptr: Long): FloatArray

    external fun nAddShadow(
        ptr: Long,
        color: Int,
        offsetX: Float,
        offsetY: Float,
        blurSigma: Double,
    )

    external fun nResetShadows(ptr: Long)

    external fun nFontFeatureNames(ptr: Long): Array<String>

    external fun nFontFeatureValues(ptr: Long): IntArray

    external fun nAddFontFeature(
        ptr: Long,
        name: String,
        value: Int,
    )

    external fun nResetFontFeatures(ptr: Long)

    external fun nHasForeground(ptr: Long): Boolean

    external fun nHasBackground(ptr: Long): Boolean

    external fun nGetForegroundPaint(ptr: Long): Long

    external fun nGetBackgroundPaint(ptr: Long): Long

    external fun nSetForegroundPaint(
        ptr: Long,
        paintPtr: Long,
    )

    external fun nSetBackgroundPaint(
        ptr: Long,
        paintPtr: Long,
    )

    external fun nClearForeground(ptr: Long)

    external fun nClearBackground(ptr: Long)
}
