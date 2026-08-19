package org.skialin

import kotlin.test.Test
import kotlin.test.assertTrue

class TextLineAndBlobBuilderTest {
    private fun makeFont(): Font {
        val mgr = FontMgr.system()
        val name = mgr.familyName(0)
        val typeface = mgr.matchFamilyStyle(name)!!
        return Font(typeface, 18f)
    }

    @Test
    fun textLineMeasuresAndDraws() {
        makeFont().use { font ->
            val line = TextLine.make("hi", font)
            assertTrue(line.width > 0f)
            assertTrue(line.glyphs.isNotEmpty())

            Surface.makeRasterN32Premul(32, 32)!!.use { surface ->
                Paint().use { paint -> surface.canvas.drawTextLine(line, 1f, 16f, paint) }
            }
        }
    }

    @Test
    fun textBlobBuilderBuildsDrawableBlob() {
        makeFont().use { font ->
            val glyphs = font.textToGlyphs("hi")
            TextBlobBuilder().use { builder ->
                builder.appendRun(font, glyphs, 0f, 0f)
                builder.build()!!.use { blob ->
                    Surface.makeRasterN32Premul(32, 32)!!.use { surface ->
                        Paint().use { paint -> surface.canvas.drawTextBlob(blob, 1f, 16f, paint) }
                    }
                }
            }
        }
    }

    @Test
    fun textBlobBuilderAppendRunPosHAndPos() {
        makeFont().use { font ->
            val glyphs = font.textToGlyphs("hi")
            val widths = font.widths(glyphs)
            TextBlobBuilder().use { builder ->
                builder.appendRunPosH(font, glyphs, floatArrayOf(0f, widths[0]), 0f)
                assertTrue(builder.build() != null)
            }
            TextBlobBuilder().use { builder ->
                builder.appendRunPos(font, glyphs, arrayOf(Point(0f, 0f), Point(widths[0], 0f)))
                assertTrue(builder.build() != null)
            }
        }
    }
}
