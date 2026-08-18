package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/**
 * Resolves font family names to [Typeface]s during paragraph layout.
 * Mirrors skparagraph's `FontCollection`.
 */
class FontCollection : Managed(FontCollectionNative.nNew(), FontCollectionNative::nRelease) {
    /** The minimum needed to get real glyphs out of a laid-out paragraph; usually [FontMgr.system]. */
    fun setDefaultFontManager(fontManager: FontMgr) = FontCollectionNative.nSetDefaultFontManager(nativePtr, fontManager.nativePtr)
}

private object FontCollectionNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nNew(): Long
    external fun nRelease(ptr: Long)
    external fun nSetDefaultFontManager(ptr: Long, fontMgrPtr: Long)
}
