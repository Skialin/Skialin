package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/**
 * A concrete [FontMgr] (`skia::textlayout::TypefaceFontProvider`) that resolves family names to
 * in-memory-registered typefaces. Register typefaces with [registerTypeface], then hand it to
 * [FontCollection.setAssetTypefaceProvider] (or set it as the dynamic/test font manager via
 * [FontCollection]) so paragraph shaping/fallback can resolve names to these typefaces - the
 * standard way to make custom/embedded fonts (e.g. loaded from bytes) participate in name-based
 * fallback resolution during layout, the same as system fonts.
 */
class TypefaceFontProvider : Managed(TypefaceFontProviderNative.nNew(), TypefaceFontProviderNative::nRelease) {
    /** Registers [typeface] under its own family name. Returns 1 on success, 0 if the typeface
     * has no family name. */
    fun registerTypeface(typeface: Typeface): Long = TypefaceFontProviderNative.nRegisterTypeface(nativePtr, typeface.nativePtr)

    /** Registers [typeface] under [alias] instead of its own family name - useful for giving a
     * loaded font's synthetic identity a resolvable family name. */
    fun registerTypeface(
        typeface: Typeface,
        alias: String,
    ): Long = TypefaceFontProviderNative.nRegisterTypefaceAlias(nativePtr, typeface.nativePtr, alias)
}

private object TypefaceFontProviderNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nNew(): Long

    external fun nRelease(ptr: Long)

    external fun nRegisterTypeface(
        ptr: Long,
        typefacePtr: Long,
    ): Long

    external fun nRegisterTypefaceAlias(
        ptr: Long,
        typefacePtr: Long,
        alias: String,
    ): Long
}
