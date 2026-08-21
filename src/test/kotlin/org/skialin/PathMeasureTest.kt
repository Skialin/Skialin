package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

class PathMeasureTest {
    private fun makeSquare(): Path {
        val builder = PathBuilder()
        builder.addRect(Rect(0f, 0f, 10f, 10f))
        return builder.snapshot()
    }

    @Test
    fun lengthMatchesSquarePerimeter() {
        makeSquare().use { path ->
            PathMeasure(path).use { measure -> assertEquals(40f, measure.length()) }
        }
    }

    @Test
    fun posTanAtZeroIsACorner() {
        makeSquare().use { path ->
            PathMeasure(path).use { measure ->
                val posTan = measure.posTan(0f)
                assertNotNull(posTan)
                assertEquals(0f, posTan.position.x)
                assertEquals(0f, posTan.position.y)
            }
        }
    }

    @Test
    fun matrixAtDistanceIsNonNull() {
        makeSquare().use { path ->
            PathMeasure(path).use { measure -> assertNotNull(measure.matrix(5f)) }
        }
    }

    @Test
    fun segmentAppendsToBuilder() {
        makeSquare().use { path ->
            PathMeasure(path).use { measure ->
                PathBuilder().use { dst ->
                    assertTrue(measure.segment(0f, 10f, dst, true))
                    dst.snapshot().use { assertFalse(it.isEmpty) }
                }
            }
        }
    }

    @Test
    fun isClosedAndNextContour() {
        makeSquare().use { path ->
            PathMeasure(path).use { measure ->
                assertTrue(measure.isContourClosed)
                assertFalse(measure.nextContour())
            }
        }
    }

    @Test
    fun setPathUpdatesMeasure() {
        makeSquare().use { path ->
            PathMeasure(path).use { measure ->
                measure.setPath(null)
                assertEquals(0f, measure.length())
                assertNull(measure.posTan(0f))
                measure.setPath(path)
                assertEquals(40f, measure.length())
            }
        }
    }

    @Test
    fun noArgConstructorStartsWithNoPath() {
        PathMeasure().use { measure ->
            assertEquals(0f, measure.length())
            assertNull(measure.posTan(0f))
            makeSquare().use { path ->
                measure.setPath(path)
                assertEquals(40f, measure.length())
            }
        }
    }
}
