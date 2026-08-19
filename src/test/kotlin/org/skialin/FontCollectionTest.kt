package org.skialin

import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class FontCollectionTest {
    @Test
    fun setsAdditionalFontManagersWithoutCrashing() {
        FontCollection().use { collection ->
            collection.setDefaultFontManager(FontMgr.system())
            collection.setAssetFontManager(FontMgr.system())
            collection.setDynamicFontManager(FontMgr.system())
            collection.setTestFontManager(FontMgr.system())
        }
    }

    @Test
    fun fontFallbackTogglesAndDefaultsEnabled() {
        FontCollection().use { collection ->
            assertTrue(collection.fontFallbackEnabled)
            collection.disableFontFallback()
            assertFalse(collection.fontFallbackEnabled)
            collection.enableFontFallback()
            assertTrue(collection.fontFallbackEnabled)
        }
    }
}
