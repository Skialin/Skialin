package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Character-level styling for a run of paragraph text. Mirrors skparagraph's `TextStyle`. */
class TextStyle internal constructor(ptr: Long) : Managed(ptr, TextStyleNative::nRelease) {
    /** A bitmask of decoration lines. */
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
        get() = FontStyle(
            TextStyleNative.nWeight(nativePtr),
            TextStyleNative.nWidth(nativePtr),
            FontStyle.Slant.entries[TextStyleNative.nSlant(nativePtr)],
        )
        set(value) = TextStyleNative.nSetFontStyle(nativePtr, value.weight, value.width, value.slant.ordinal)

    var decoration: Decoration
        get() = Decoration(
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

    /** 0 unless [heightOverride] is set. */
    var height: Float
        get() = TextStyleNative.nHeight(nativePtr)
        set(value) = TextStyleNative.nSetHeight(nativePtr, value)

    var heightOverride: Boolean
        get() = TextStyleNative.nHeightOverride(nativePtr)
        set(value) = TextStyleNative.nSetHeightOverride(nativePtr, value)

    var typeface: Typeface?
        get() = TextStyleNative.nTypeface(nativePtr).takeIf { it != 0L }?.let { Typeface(it) }
        set(value) = TextStyleNative.nSetTypeface(nativePtr, value?.nativePtr ?: 0L)

    var locale: String
        get() = TextStyleNative.nLocale(nativePtr)
        set(value) = TextStyleNative.nSetLocale(nativePtr, value)
}

private object TextStyleNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nNew(): Long
    external fun nClone(ptr: Long): Long
    external fun nRelease(ptr: Long)
    external fun nColor(ptr: Long): Int
    external fun nSetColor(ptr: Long, color: Int)
    external fun nFontFamilies(ptr: Long): Array<String>
    external fun nSetFontFamilies(ptr: Long, families: Array<String>)
    external fun nFontSize(ptr: Long): Float
    external fun nSetFontSize(ptr: Long, size: Float)
    external fun nWeight(ptr: Long): Int
    external fun nWidth(ptr: Long): Int
    external fun nSlant(ptr: Long): Int
    external fun nSetFontStyle(ptr: Long, weight: Int, width: Int, slant: Int)
    external fun nDecorationType(ptr: Long): Int
    external fun nDecorationMode(ptr: Long): Int
    external fun nDecorationColor(ptr: Long): Int
    external fun nDecorationStyle(ptr: Long): Int
    external fun nDecorationThicknessMultiplier(ptr: Long): Float
    external fun nSetDecoration(ptr: Long, decoration: Int)
    external fun nSetDecorationMode(ptr: Long, mode: Int)
    external fun nSetDecorationColor(ptr: Long, color: Int)
    external fun nSetDecorationStyle(ptr: Long, style: Int)
    external fun nSetDecorationThicknessMultiplier(ptr: Long, multiplier: Float)
    external fun nLetterSpacing(ptr: Long): Float
    external fun nSetLetterSpacing(ptr: Long, letterSpacing: Float)
    external fun nWordSpacing(ptr: Long): Float
    external fun nSetWordSpacing(ptr: Long, wordSpacing: Float)
    external fun nHeight(ptr: Long): Float
    external fun nSetHeight(ptr: Long, height: Float)
    external fun nHeightOverride(ptr: Long): Boolean
    external fun nSetHeightOverride(ptr: Long, heightOverride: Boolean)
    external fun nTypeface(ptr: Long): Long
    external fun nSetTypeface(ptr: Long, typefacePtr: Long)
    external fun nLocale(ptr: Long): String
    external fun nSetLocale(ptr: Long, locale: String)
}
