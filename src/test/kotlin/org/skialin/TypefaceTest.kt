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
}
