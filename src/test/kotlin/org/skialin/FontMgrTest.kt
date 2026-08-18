package org.skialin

import kotlin.test.Test
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

class FontMgrTest {
    @Test
    fun systemFontMgrHasFamilies() {
        FontMgr.system().use { mgr ->
            assertTrue(mgr.countFamilies > 0)
            assertTrue(mgr.familyName(0).isNotEmpty())
        }
    }

    @Test
    fun matchFamilyStyleFindsAKnownFamily() {
        FontMgr.system().use { mgr ->
            val name = mgr.familyName(0)
            mgr.matchFamilyStyle(name).use { typeface ->
                assertNotNull(typeface)
            }
        }
    }

    @Test
    fun matchFamilyStyleReturnsNullForEmptyMgr() {
        FontMgr.empty().use { mgr ->
            assertNull(mgr.matchFamilyStyle("Nonexistent Family XYZ"))
        }
    }

    @Test
    fun makeFromFileReturnsNullForMissingFile() {
        FontMgr.system().use { mgr ->
            assertNull(mgr.makeFromFile("C:/definitely/not/a/real/font.ttf"))
        }
    }
}
