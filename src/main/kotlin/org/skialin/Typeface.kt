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

    /** Clones this typeface with the given variable-font axis settings and/or font-collection
     * (ttc/dfont) index applied. `axes` is a list of (four-byte axis tag, value) pairs, e.g.
     * `FourByteTag('w','g','h','t') to 700f`. Returns `null` if the clone fails (e.g. a bad
     * [collectionIndex]); an axis tag the font doesn't support is simply ignored by Skia, not an
     * error. Mirrors `SkTypeface::makeClone(const SkFontArguments&)`. */
    fun makeClone(
        axes: List<Pair<Int, Float>> = emptyList(),
        collectionIndex: Int = 0,
    ): Typeface? {
        val ptr =
            TypefaceNative.nMakeClone(
                nativePtr,
                axes.map { it.first }.toIntArray(),
                axes.map { it.second }.toFloatArray(),
                collectionIndex,
            )
        return if (ptr == 0L) null else Typeface(ptr)
    }

    companion object {
        /** Packs a 4-character axis/table tag (e.g. `"wght"`) into Skia's four-byte-tag
         * encoding, matching `SkSetFourByteTag`/`SK_FOURCC`. */
        fun fourByteTag(tag: String): Int {
            require(tag.length == 4) { "tag must be exactly 4 characters: $tag" }
            return (tag[0].code shl 24) or (tag[1].code shl 16) or (tag[2].code shl 8) or tag[3].code
        }

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

    external fun nMakeClone(
        ptr: Long,
        axisTags: IntArray,
        axisValues: FloatArray,
        collectionIndex: Int,
    ): Long
}
