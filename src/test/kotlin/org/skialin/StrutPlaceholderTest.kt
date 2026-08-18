package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class StrutPlaceholderTest {
    private fun fontCollection(): FontCollection {
        val collection = FontCollection()
        collection.setDefaultFontManager(FontMgr.system())
        return collection
    }

    @Test
    fun strutStyleDefaultsAndRoundtrips() {
        StrutStyle().use { strut ->
            assertFalse(strut.strutEnabled)

            strut.strutEnabled = true
            strut.fontSize = 20f
            strut.height = 1.5f
            strut.heightOverride = true
            strut.leading = 0.2f
            strut.forceStrutHeight = true
            strut.halfLeading = true
            strut.fontFamilies = listOf("Arial")

            assertTrue(strut.strutEnabled)
            assertEquals(20f, strut.fontSize)
            assertEquals(1.5f, strut.height)
            assertTrue(strut.heightOverride)
            assertEquals(0.2f, strut.leading)
            assertTrue(strut.forceStrutHeight)
            assertTrue(strut.halfLeading)
            assertEquals(listOf("Arial"), strut.fontFamilies)
        }
    }

    @Test
    fun paragraphStyleStrutStyleRoundtrips() {
        ParagraphStyle().use { style ->
            val strut = style.strutStyle
            strut.strutEnabled = true
            strut.fontSize = 22f
            style.strutStyle = strut
            strut.close()

            style.strutStyle.use { s ->
                assertEquals(22f, s.fontSize)
                assertTrue(s.strutEnabled)
            }
        }
    }

    @Test
    fun addPlaceholderBuildsWithoutCrashing() {
        fontCollection().use { collection ->
            ParagraphStyle().use { style ->
                ParagraphBuilder(style, collection).use { builder ->
                    TextStyle().use { textStyle ->
                        textStyle.fontSize = 18f
                        builder.pushStyle(textStyle)
                        builder.addText("Before ")
                        builder.addPlaceholder(PlaceholderStyle(20f, 20f, PlaceholderStyle.Alignment.MIDDLE))
                        builder.addText(" after")
                        builder.pop()

                        builder.build().use { paragraph ->
                            paragraph.layout(200f)
                            assertTrue(paragraph.height > 0f)
                        }
                    }
                }
            }
        }
    }
}
