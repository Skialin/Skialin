package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotEquals

class TypefaceTest {
    @Test
    fun emptyTypefaceHasNoGlyphs() {
        Typeface.makeEmpty().use { typeface ->
            assertEquals(0, typeface.countGlyphs)
            assertFalse(typeface.isBold)
            assertFalse(typeface.isItalic)
        }
    }

    @Test
    fun emptyTypefaceHasAUniqueId() {
        Typeface.makeEmpty().use { a ->
            Typeface.makeEmpty().use { b ->
                assertNotEquals(0, a.uniqueId)
                assertNotEquals(0, b.uniqueId)
            }
        }
    }

    @Test
    fun fontStyleDefaultsToNormal() {
        Typeface.makeEmpty().use { typeface ->
            assertEquals(FontStyle.NORMAL, typeface.fontStyle)
        }
    }

    @Test
    fun familyNameIsEmptyStringForEmptyTypeface() {
        Typeface.makeEmpty().use { typeface ->
            assertEquals("", typeface.familyName)
        }
    }

    @Test
    fun fourByteTagPacksBigEndian() {
        // 'w'=0x77 'g'=0x67 'h'=0x68 't'=0x74
        assertEquals(0x77676874, Typeface.fourByteTag("wght"))
    }

    @Test
    fun makeCloneWithNoAxesReturnsAWorkingTypeface() {
        val mgr = FontMgr.system()
        val name = mgr.familyName(0)
        mgr.matchFamilyStyle(name)!!.use { typeface ->
            typeface.makeClone()!!.use { clone ->
                assertEquals(typeface.familyName, clone.familyName)
            }
        }
    }
}
