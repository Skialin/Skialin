package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class TextStyleTest {
    @Test
    fun defaultFontSizeIs14() {
        TextStyle().use { style -> assertEquals(14f, style.fontSize) }
    }

    @Test
    fun colorRoundtrips() {
        TextStyle().use { style ->
            style.color = Colors.RED
            assertEquals(Colors.RED, style.color)
        }
    }

    @Test
    fun fontFamiliesRoundtrip() {
        TextStyle().use { style ->
            style.fontFamilies = listOf("Arial", "Helvetica")
            assertEquals(listOf("Arial", "Helvetica"), style.fontFamilies)
        }
    }

    @Test
    fun decorationRoundtrips() {
        TextStyle().use { style ->
            style.decoration =
                TextStyle.Decoration(
                    TextStyle.TextDecoration.UNDERLINE or TextStyle.TextDecoration.LINE_THROUGH,
                    TextStyle.DecorationMode.GAPS,
                    Colors.BLUE,
                    TextStyle.DecorationStyle.DASHED,
                    2f,
                )
            val decoration = style.decoration
            assertEquals(TextStyle.TextDecoration.UNDERLINE or TextStyle.TextDecoration.LINE_THROUGH, decoration.decoration)
            assertEquals(TextStyle.DecorationMode.GAPS, decoration.mode)
            assertEquals(Colors.BLUE, decoration.color)
            assertEquals(TextStyle.DecorationStyle.DASHED, decoration.style)
            assertEquals(2f, decoration.thicknessMultiplier)
        }
    }

    @Test
    fun spacingAndHeightRoundtrip() {
        TextStyle().use { style ->
            style.letterSpacing = 1.5f
            style.wordSpacing = 3f
            style.heightOverride = true
            style.height = 2f
            assertEquals(1.5f, style.letterSpacing)
            assertEquals(3f, style.wordSpacing)
            assertTrue(style.heightOverride)
            assertEquals(2f, style.height)
        }
    }

    @Test
    fun localeRoundtrips() {
        TextStyle().use { style ->
            style.locale = "en-US"
            assertEquals("en-US", style.locale)
        }
    }

    @Test
    fun shadowsRoundtrip() {
        TextStyle().use { style ->
            assertEquals(emptyList(), style.shadows)
            style.addShadow(TextStyle.Shadow(Colors.RED, 1f, 2f, 3.0))
            style.addShadow(TextStyle.Shadow(Colors.BLUE, -1f, 0.5f, 0.0))
            val shadows = style.shadows
            assertEquals(2, shadows.size)
            assertEquals(TextStyle.Shadow(Colors.RED, 1f, 2f, 3.0), shadows[0])
            assertEquals(TextStyle.Shadow(Colors.BLUE, -1f, 0.5f, 0.0), shadows[1])
            style.clearShadows()
            assertEquals(emptyList(), style.shadows)
        }
    }

    @Test
    fun fontFeaturesRoundtrip() {
        TextStyle().use { style ->
            assertEquals(emptyList(), style.fontFeatures)
            style.addFontFeature("liga", 1)
            style.addFontFeature("smcp", 0)
            val features = style.fontFeatures
            assertEquals(2, features.size)
            assertEquals(TextStyle.FontFeature("liga", 1), features[0])
            assertEquals(TextStyle.FontFeature("smcp", 0), features[1])
            style.clearFontFeatures()
            assertEquals(emptyList(), style.fontFeatures)
        }
    }

    @Test
    fun cloneIsIndependent() {
        TextStyle().use { style ->
            style.fontSize = 20f
            style.cloneStyle().use { cloned ->
                style.fontSize = 30f
                assertEquals(20f, cloned.fontSize)
                assertEquals(30f, style.fontSize)
            }
        }
    }
}
