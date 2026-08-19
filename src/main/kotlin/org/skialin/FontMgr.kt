package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Enumerates and creates [Typeface]s. Mirrors Skia's `SkFontMgr`. */
class FontMgr internal constructor(
    ptr: Long,
) : Managed(ptr, FontMgrNative::nRelease) {
    val countFamilies: Int get() = FontMgrNative.nCountFamilies(nativePtr)

    fun familyName(index: Int): String = FontMgrNative.nFamilyName(nativePtr, index)

    /** `familyName` of `null` requests the default system family, which most
     * systems don't have, so it will often fall through to `null` here. */
    fun matchFamilyStyle(
        familyName: String?,
        style: FontStyle = FontStyle.NORMAL,
    ): Typeface? {
        val ptr = FontMgrNative.nMatchFamilyStyle(nativePtr, familyName, style.weight, style.width, style.slant.ordinal)
        return if (ptr == 0L) null else Typeface(ptr)
    }

    fun makeFromData(
        data: Data,
        ttcIndex: Int = 0,
    ): Typeface? {
        val ptr = FontMgrNative.nMakeFromData(nativePtr, data.nativePtr, ttcIndex)
        return if (ptr == 0L) null else Typeface(ptr)
    }

    fun makeFromFile(
        path: String,
        ttcIndex: Int = 0,
    ): Typeface? {
        val ptr = FontMgrNative.nMakeFromFile(nativePtr, path, ttcIndex)
        return if (ptr == 0L) null else Typeface(ptr)
    }

    companion object {
        /** The platform's default font manager (DirectWrite on Windows, CoreText on macOS, FontConfig on Linux). */
        fun system(): FontMgr = FontMgr(FontMgrNative.nSystem())

        fun empty(): FontMgr = FontMgr(FontMgrNative.nEmpty())
    }
}

private object FontMgrNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nSystem(): Long

    external fun nEmpty(): Long

    external fun nRelease(ptr: Long)

    external fun nCountFamilies(ptr: Long): Int

    external fun nFamilyName(
        ptr: Long,
        index: Int,
    ): String

    external fun nMatchFamilyStyle(
        ptr: Long,
        familyName: String?,
        weight: Int,
        width: Int,
        slant: Int,
    ): Long

    external fun nMakeFromData(
        ptr: Long,
        dataPtr: Long,
        ttcIndex: Int,
    ): Long

    external fun nMakeFromFile(
        ptr: Long,
        path: String,
        ttcIndex: Int,
    ): Long
}
