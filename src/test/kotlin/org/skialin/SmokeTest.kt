package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class SmokeTest {
    @Test
    fun drawsRedRectangleOntoRasterSurface() {
        Surface.makeRasterN32Premul(64, 64)!!.use { surface ->
            val canvas = surface.canvas
            canvas.clear(Colors.WHITE)

            Paint().use { paint ->
                paint.color = Colors.RED
                paint.isAntiAlias = true
                canvas.drawRect(Rect(8f, 8f, 56f, 56f), paint)
            }

            surface.makeImageSnapshot()!!.use { image ->
                assertEquals(64, image.width)
                assertEquals(64, image.height)
                assertTrue(image.encodeToPng()!!.isNotEmpty())
            }
        }
    }

    @Test
    fun pathBuilderProducesNonEmptySnapshot() {
        PathBuilder().use { builder ->
            builder
                .moveTo(0f, 0f)
                .lineTo(10f, 0f)
                .lineTo(10f, 10f)
                .closePath()
            assertFalse(builder.isEmpty)

            builder.snapshot().use { path ->
                assertFalse(path.isEmpty)
                assertTrue(path.contains(Point(7f, 3f)))
            }
        }
    }

    @Test
    fun matrixMapsPointsAsExpected() {
        val m = Matrix33.makeTranslate(10f, 20f)
        val mapped = m.mapPoint(Point(1f, 1f))
        assertEquals(11f, mapped.x)
        assertEquals(21f, mapped.y)
    }
}
