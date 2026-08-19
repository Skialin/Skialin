package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class FontTextBlobTypefaceExpandedTest {
    private fun makeFont(): Font {
        val mgr = FontMgr.system()
        val name = mgr.familyName(0)
        val typeface = mgr.matchFamilyStyle(name)!!
        return Font(typeface, 18f)
    }

    @Test
    fun fontBoundsPositionsAndPathsAreConsistent() {
        makeFont().use { font ->
            val glyphs = font.textToGlyphs("Hi")

            val bounds = font.getBounds(glyphs)
            assertEquals(2, bounds.size)

            val positions = font.getPositions(glyphs)
            assertEquals(2, positions.size)
            assertEquals(0f, positions[0].x)

            val xpos = font.getXPositions(glyphs)
            assertEquals(2, xpos.size)

            font.makeWithSize(36f).use { bigger -> assertEquals(36f, bigger.size) }

            font.getPath(glyphs[0]).use { assertNotNull(it) }
            val paths = font.getPaths(glyphs)
            assertEquals(2, paths.size)
            paths.forEach { it?.close() }
        }
    }

    @Test
    fun textBlobRSXformInterceptsAndSerialize() {
        makeFont().use { font ->
            val xforms = floatArrayOf(1f, 0f, 0f, 0f, 1f, 0f, 20f, 0f)
            TextBlob.makeFromRSXform("Hi", xforms, font)!!.use { blob ->
                assertTrue(blob.bounds.right > blob.bounds.left)
            }

            TextBlob.makeFromText("Hi", font)!!.use { blob ->
                val intercepts = blob.getIntercepts(blob.bounds.top, blob.bounds.bottom)
                assertTrue(intercepts.size % 2 == 0)

                blob.serializeToData().use { data ->
                    assertTrue(data.size > 0)
                    TextBlob.makeFromData(data)!!.use { restored ->
                        assertEquals(blob.bounds.right, restored.bounds.right)
                    }
                }
            }
        }
    }

    @Test
    fun typefaceTableIntrospection() {
        val mgr = FontMgr.system()
        val name = mgr.familyName(0)
        mgr.matchFamilyStyle(name)!!.use { typeface ->
            for (tag in typeface.tableTags) {
                val size = typeface.getTableSize(tag)
                val data = typeface.getTableData(tag)
                assertEquals(size, data.size.toLong())
            }
        }
    }
}
