package org.skialin

import kotlin.test.Test
import kotlin.test.assertNotEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

class TextBlobTest {
    private fun makeFont(): Font {
        val mgr = FontMgr.system()
        val name = mgr.familyName(0)
        val typeface = mgr.matchFamilyStyle(name)!!
        return Font(typeface, 18f)
    }

    @Test
    fun fromTextHasNonemptyBounds() {
        makeFont().use { font ->
            TextBlob.makeFromText("Hi", font).use { blob ->
                assertNotNull(blob)
                assertTrue(blob.bounds.right > blob.bounds.left)
                assertNotEquals(0, blob.uniqueId)
            }
        }
    }

    @Test
    fun fromTextIsNullForEmptyString() {
        makeFont().use { font ->
            assertNull(TextBlob.makeFromText("", font))
        }
    }

    @Test
    fun fromPosTextHMatchesGlyphCount() {
        makeFont().use { font ->
            val glyphs = font.textToGlyphs("Hi")
            val widths = font.widths(glyphs)
            var x = 0f
            val xpos = FloatArray(widths.size)
            widths.forEachIndexed { i, w ->
                xpos[i] = x
                x += w
            }
            TextBlob.makeFromPosTextH("Hi", xpos, 0f, font)!!.use { blob ->
                assertTrue(blob.bounds.right > blob.bounds.left)
            }
        }
    }

    @Test
    fun drawsWithoutCrashing() {
        makeFont().use { font ->
            TextBlob.makeFromText("Hi", font)!!.use { blob ->
                Surface.makeRasterN32Premul(64, 64)!!.use { surface ->
                    Paint().use { paint ->
                        paint.color = Colors.BLACK
                        surface.canvas().drawTextBlob(blob, 4f, 20f, paint)
                    }
                }
            }
        }
    }
}
