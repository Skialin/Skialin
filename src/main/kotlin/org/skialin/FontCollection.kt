package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class FontCollection : Managed(FontCollectionNative.nNew(), FontCollectionNative::nRelease) {
    fun setDefaultFontManager(fontManager: FontMgr) = FontCollectionNative.nSetDefaultFontManager(nativePtr, fontManager.nativePtr)

    fun setAssetFontManager(fontManager: FontMgr) = FontCollectionNative.nSetAssetFontManager(nativePtr, fontManager.nativePtr)

    fun setDynamicFontManager(fontManager: FontMgr) = FontCollectionNative.nSetDynamicFontManager(nativePtr, fontManager.nativePtr)

    fun setTestFontManager(fontManager: FontMgr) = FontCollectionNative.nSetTestFontManager(nativePtr, fontManager.nativePtr)

    fun disableFontFallback() = FontCollectionNative.nDisableFontFallback(nativePtr)

    fun enableFontFallback() = FontCollectionNative.nEnableFontFallback(nativePtr)

    val fontFallbackEnabled: Boolean get() = FontCollectionNative.nFontFallbackEnabled(nativePtr)
}

private object FontCollectionNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nNew(): Long

    external fun nRelease(ptr: Long)

    external fun nSetDefaultFontManager(
        ptr: Long,
        fontMgrPtr: Long,
    )

    external fun nSetAssetFontManager(
        ptr: Long,
        fontMgrPtr: Long,
    )

    external fun nSetDynamicFontManager(
        ptr: Long,
        fontMgrPtr: Long,
    )

    external fun nSetTestFontManager(
        ptr: Long,
        fontMgrPtr: Long,
    )

    external fun nDisableFontFallback(ptr: Long)

    external fun nEnableFontFallback(ptr: Long)

    external fun nFontFallbackEnabled(ptr: Long): Boolean
}
