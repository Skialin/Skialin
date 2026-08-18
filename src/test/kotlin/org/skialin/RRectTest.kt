package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class RRectTest {
    @Test
    fun makeRectXYHasSimpleType() {
        RRect.makeRectXY(Rect(0f, 0f, 20f, 20f), 4f, 4f).use { rrect ->
            assertEquals(RRect.Type.SIMPLE, rrect.type)
            assertEquals(Rect(0f, 0f, 20f, 20f), rrect.rect)
        }
    }

    @Test
    fun makeOvalHasOvalType() {
        RRect.makeOval(Rect(0f, 0f, 20f, 10f)).use { rrect -> assertEquals(RRect.Type.OVAL, rrect.type) }
    }

    @Test
    fun makeRectHasRectType() {
        RRect.makeRect(Rect(0f, 0f, 20f, 20f)).use { rrect ->
            assertEquals(RRect.Type.RECT, rrect.type)
            assertFalse(rrect.isEmpty)
        }
    }

    @Test
    fun makeRectRadiiRoundtripsRadii() {
        val radii = arrayOf(Point(1f, 2f), Point(3f, 4f), Point(5f, 6f), Point(7f, 8f))
        RRect.makeRectRadii(Rect(0f, 0f, 40f, 40f), radii).use { rrect ->
            assertEquals(RRect.Type.COMPLEX, rrect.type)
            assertEquals(radii.toList(), rrect.radii.toList())
        }
    }

    @Test
    fun containsPointAndRect() {
        RRect.makeRect(Rect(0f, 0f, 20f, 20f)).use { rrect ->
            assertTrue(rrect.contains(Point(10f, 10f)))
            assertFalse(rrect.contains(Point(100f, 100f)))
            assertTrue(rrect.contains(Rect(2f, 2f, 18f, 18f)))
            assertTrue(rrect.isValid)
        }
    }

    @Test
    fun insetAndOutset() {
        RRect.makeRect(Rect(0f, 0f, 20f, 20f)).use { rrect ->
            rrect.inset(2f, 2f).use { assertEquals(Rect(2f, 2f, 18f, 18f), it.rect) }
            rrect.outset(2f, 2f).use { assertEquals(Rect(-2f, -2f, 22f, 22f), it.rect) }
        }
    }

    @Test
    fun drawsAndClipsWithoutCrashing() {
        RRect.makeRectXY(Rect(0f, 0f, 32f, 32f), 4f, 4f).use { outer ->
            RRect.makeRectXY(Rect(4f, 4f, 28f, 28f), 2f, 2f).use { inner ->
                Surface.makeRasterN32Premul(32, 32)!!.use { surface ->
                    val canvas = surface.canvas()
                    Paint().use { paint ->
                        paint.color = Colors.RED
                        canvas.drawRRect(outer, paint)
                        canvas.drawDRRect(outer, inner, paint)
                        canvas.clipRRect(outer)
                    }
                }
            }
        }
    }
}
