package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class ParagraphStyleTest {
    @Test
    fun replaceTabCharactersRoundtrips() {
        ParagraphStyle().use { style ->
            assertTrue(!style.replaceTabCharacters)
            style.replaceTabCharacters = true
            assertTrue(style.replaceTabCharacters)
        }
    }

    @Test
    fun defaultsToLtr() {
        ParagraphStyle().use { style -> assertEquals(ParagraphStyle.TextDirection.LTR, style.textDirection) }
    }

    @Test
    fun textDirectionRoundtripsToRtl() {
        ParagraphStyle().use { style ->
            style.textDirection = ParagraphStyle.TextDirection.RTL
            assertEquals(ParagraphStyle.TextDirection.RTL, style.textDirection)
        }
    }

    @Test
    fun textAlignRoundtrips() {
        ParagraphStyle().use { style ->
            style.textAlign = ParagraphStyle.TextAlign.JUSTIFY
            assertEquals(ParagraphStyle.TextAlign.JUSTIFY, style.textAlign)
        }
    }

    @Test
    fun maxLinesAndEllipsisRoundtrip() {
        ParagraphStyle().use { style ->
            style.maxLines = 3
            style.ellipsis = "..."
            assertEquals(3L, style.maxLines)
            assertEquals("...", style.ellipsis)
        }
    }

    @Test
    fun heightAndBehaviorRoundtrip() {
        ParagraphStyle().use { style ->
            style.height = 1.5f
            style.textHeightBehavior = ParagraphStyle.TextHeightBehavior.DISABLE_FIRST_ASCENT
            assertEquals(1.5f, style.height)
            assertEquals(ParagraphStyle.TextHeightBehavior.DISABLE_FIRST_ASCENT, style.textHeightBehavior)
        }
    }

    @Test
    fun textStyleRoundtrips() {
        ParagraphStyle().use { style ->
            val textStyle = style.textStyle
            textStyle.fontSize = 22f
            style.textStyle = textStyle
            textStyle.close()
            style.textStyle.use { assertEquals(22f, it.fontSize) }
        }
    }
}
