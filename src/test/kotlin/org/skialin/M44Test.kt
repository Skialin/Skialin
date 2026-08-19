package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class M44Test {
    @Test
    fun identityMapsAPointToItself() {
        M44.identity().use { m ->
            val out = m.map(floatArrayOf(1f, 2f, 3f, 1f))
            assertTrue(out.contentEquals(floatArrayOf(1f, 2f, 3f, 1f)))
        }
    }

    @Test
    fun translateShiftsAPoint() {
        M44.makeTranslate(5f, 6f).use { m ->
            val out = m.map(floatArrayOf(1f, 2f, 0f, 1f))
            assertTrue(out.contentEquals(floatArrayOf(6f, 8f, 0f, 1f)))
        }
    }

    @Test
    fun scaleScalesAPoint() {
        M44.makeScale(2f, 3f).use { m ->
            val out = m.map(floatArrayOf(1f, 1f, 1f, 1f))
            assertTrue(out.contentEquals(floatArrayOf(2f, 3f, 1f, 1f)))
        }
    }

    @Test
    fun concatComposesTransforms() {
        M44.makeTranslate(10f, 0f).use { translate ->
            M44.makeScale(2f, 2f).use { scale ->
                M44.concat(translate, scale).use { combined ->
                    val out = combined.map(floatArrayOf(1f, 1f, 0f, 1f))
                    assertTrue(out.contentEquals(floatArrayOf(12f, 2f, 0f, 1f)))
                }
            }
        }
    }

    @Test
    fun invertUndoesTranslate() {
        M44.makeTranslate(3f, 4f).use { m ->
            m.invert()!!.use { inv ->
                val out = inv.map(floatArrayOf(3f, 4f, 0f, 1f))
                assertTrue(kotlin.math.abs(out[0]) < 1e-5f)
                assertTrue(kotlin.math.abs(out[1]) < 1e-5f)
            }
        }
    }

    @Test
    fun rowMajorRoundtrips() {
        val values = FloatArray(16) { it.toFloat() }
        M44.makeFromRowMajor(values).use { m -> assertTrue(m.rowMajor.contentEquals(values)) }
    }

    @Test
    fun equalityAndClone() {
        M44.makeTranslate(1f, 2f, 3f).use { a ->
            a.cloneM44().use { b -> assertEquals(a, b) }
            M44.identity().use { c -> assertNotEquals(a, c) }
        }
    }

    @Test
    fun verticesDrawsATriangleWithoutCrashing() {
        val positions = floatArrayOf(0f, 0f, 10f, 0f, 5f, 10f)
        val colors = intArrayOf(Colors.RED, Colors.GREEN, Colors.BLUE)
        Vertices.makeCopy(VertexMode.TRIANGLES, positions, colors = colors)!!.use { vertices ->
            Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
                Paint().use { paint -> surface.canvas().drawVertices(vertices, BlendMode.SRC_OVER, paint) }
            }
        }
    }

    @Test
    fun verticesWithIndicesAndTexs() {
        val positions = floatArrayOf(0f, 0f, 10f, 0f, 10f, 10f, 0f, 10f)
        val indices = shortArrayOf(0, 1, 2, 0, 2, 3)
        val vertices = Vertices.makeCopy(VertexMode.TRIANGLES, positions, texs = positions, indices = indices)
        assertNotNull(vertices)
        vertices.close()
    }

    @Test
    fun transposeSwapsRowsAndColumns() {
        val values = FloatArray(16) { it.toFloat() }
        M44.makeFromRowMajor(values).use { m ->
            m.transpose().use { t ->
                for (row in 0 until 4) {
                    for (col in 0 until 4) {
                        assertEquals(m.rc(row, col), t.rc(col, row))
                    }
                }
            }
        }
    }

    @Test
    fun rcMatchesRowMajor() {
        val values = FloatArray(16) { it.toFloat() }
        M44.makeFromRowMajor(values).use { m ->
            for (row in 0 until 4) {
                for (col in 0 until 4) {
                    assertEquals(values[row * 4 + col], m.rc(row, col))
                }
            }
        }
    }

    @Test
    fun concat44DrawsWithoutCrashing() {
        Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
            val canvas = surface.canvas()
            M44.makeTranslate(2f, 2f).use { canvas.concat44(it) }
            Paint().use { paint ->
                paint.color = Colors.RED
                canvas.drawRect(Rect(0f, 0f, 4f, 4f), paint)
            }
        }
    }
}
