package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class FontTest {
    @Test
    fun defaultFontHasATypeface() {
        Font().use { font ->
            font.typeface.use { assertNotNull(it) }
        }
    }

    @Test
    fun sizeRoundtrips() {
        Font().use { font ->
            font.size = 24f
            assertEquals(24f, font.size)
        }
    }

    @Test
    fun edgingAndHintingRoundtrip() {
        Font().use { font ->
            font.edging = Font.Edging.ANTI_ALIAS
            assertEquals(Font.Edging.ANTI_ALIAS, font.edging)
            font.hinting = Font.Hinting.FULL
            assertEquals(Font.Hinting.FULL, font.hinting)
        }
    }

    @Test
    fun fromTypefaceUsesTheGivenTypeface() {
        FontMgr.system().use { mgr ->
            val name = mgr.familyName(0)
            mgr.matchFamilyStyle(name)!!.use { typeface ->
                Font(typeface, 18f).use { font ->
                    assertEquals(18f, font.size)
                    font.typeface.use { assertNotNull(it) }
                }
            }
        }
    }

    @Test
    fun textToGlyphsAndMeasureTextAreConsistent() {
        FontMgr.system().use { mgr ->
            val name = mgr.familyName(0)
            mgr.matchFamilyStyle(name)!!.use { typeface ->
                Font(typeface, 18f).use { font ->
                    val glyphs = font.textToGlyphs("Hi")
                    assertEquals(2, glyphs.size)

                    val width = font.measureText("Hi")
                    assertTrue(width > 0f)

                    val widths = font.widths(glyphs)
                    assertEquals(2, widths.size)
                    assertTrue(widths.sum() > 0f)
                }
            }
        }
    }

    @Test
    fun metricsAreNonzeroForAPositiveSize() {
        FontMgr.system().use { mgr ->
            val name = mgr.familyName(0)
            mgr.matchFamilyStyle(name)!!.use { typeface ->
                Font(typeface, 18f).use { font ->
                    val metrics = font.metrics()
                    assertTrue(metrics.descent - metrics.ascent > 0f)
                    assertTrue(font.spacing > 0f)
                }
            }
        }
    }
}
