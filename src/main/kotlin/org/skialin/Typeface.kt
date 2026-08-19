package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Specifies a typeface and its intrinsic style. Mirrors Skia's `SkTypeface`. */
class Typeface internal constructor(
    ptr: Long,
) : Managed(ptr, TypefaceNative::nRelease) {
    val uniqueId: Int get() = TypefaceNative.nUniqueId(nativePtr)
    val isBold: Boolean get() = TypefaceNative.nIsBold(nativePtr)
    val isItalic: Boolean get() = TypefaceNative.nIsItalic(nativePtr)
    val isFixedPitch: Boolean get() = TypefaceNative.nIsFixedPitch(nativePtr)
    val countGlyphs: Int get() = TypefaceNative.nCountGlyphs(nativePtr)
    val unitsPerEm: Int get() = TypefaceNative.nUnitsPerEm(nativePtr)
    val familyName: String get() = TypefaceNative.nFamilyName(nativePtr)

    val fontStyle: FontStyle
        get() =
            FontStyle(
                TypefaceNative.nWeight(nativePtr),
                TypefaceNative.nWidth(nativePtr),
                FontStyle.Slant.entries[TypefaceNative.nSlant(nativePtr)],
            )

    fun unicharToGlyph(unichar: Int): Int = TypefaceNative.nUnicharToGlyph(nativePtr, unichar)

    val tableTags: IntArray get() = TypefaceNative.nTableTags(nativePtr)

    fun getTableSize(tag: Int): Long = TypefaceNative.nTableSize(nativePtr, tag)

    fun getTableData(
        tag: Int,
        offset: Long = 0L,
        length: Long = getTableSize(tag),
    ): ByteArray = TypefaceNative.nTableData(nativePtr, tag, offset, length)

    companion object {
        fun makeEmpty(): Typeface = Typeface(TypefaceNative.nMakeEmpty())
    }
}

private object TypefaceNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeEmpty(): Long

    external fun nRelease(ptr: Long)

    external fun nUniqueId(ptr: Long): Int

    external fun nIsBold(ptr: Long): Boolean

    external fun nIsItalic(ptr: Long): Boolean

    external fun nIsFixedPitch(ptr: Long): Boolean

    external fun nCountGlyphs(ptr: Long): Int

    external fun nUnitsPerEm(ptr: Long): Int

    external fun nUnicharToGlyph(
        ptr: Long,
        unichar: Int,
    ): Int

    external fun nWeight(ptr: Long): Int

    external fun nWidth(ptr: Long): Int

    external fun nSlant(ptr: Long): Int

    external fun nFamilyName(ptr: Long): String

    external fun nTableTags(ptr: Long): IntArray

    external fun nTableSize(
        ptr: Long,
        tag: Int,
    ): Long

    external fun nTableData(
        ptr: Long,
        tag: Int,
        offset: Long,
        length: Long,
    ): ByteArray
}
