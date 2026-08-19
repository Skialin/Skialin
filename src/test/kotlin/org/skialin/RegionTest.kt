package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class RegionTest {
    @Test
    fun makeRectReportsBoundsAndType() {
        Region.makeRect(IRect(0, 0, 10, 20)).use { region ->
            assertTrue(region.isRect)
            assertFalse(region.isEmpty)
            assertEquals(IRect(0, 0, 10, 20), region.bounds)
        }
    }

    @Test
    fun unionProducesComplexRegion() {
        Region.makeRect(IRect(0, 0, 10, 10)).use { region ->
            Region.makeRect(IRect(20, 20, 30, 30)).use { other ->
                assertTrue(region.op(other, RegionOp.UNION))
                assertTrue(region.isComplex)
                assertEquals(IRect(0, 0, 30, 30), region.bounds)
            }
        }
    }

    @Test
    fun containsAndIntersects() {
        Region.makeRect(IRect(0, 0, 10, 10)).use { region ->
            assertTrue(region.contains(5, 5))
            assertFalse(region.contains(15, 15))
            assertTrue(region.intersects(IRect(5, 5, 15, 15)))
            assertFalse(region.intersects(IRect(20, 20, 30, 30)))
        }
    }

    @Test
    fun cloneIsIndependent() {
        Region.makeRect(IRect(0, 0, 10, 10)).use { region ->
            region.cloneRegion().use { clone ->
                region.setRect(IRect(0, 0, 5, 5))
                assertEquals(IRect(0, 0, 10, 10), clone.bounds)
                assertEquals(IRect(0, 0, 5, 5), region.bounds)
            }
        }
    }

    @Test
    fun drawsAndClipsWithoutCrashing() {
        Region.makeRect(IRect(0, 0, 16, 16)).use { region ->
            Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
                Paint().use { paint ->
                    paint.color = Colors.RED
                    val canvas = surface.canvas()
                    canvas.drawRegion(region, paint)
                    canvas.clipRegion(region)
                }
            }
        }
    }
}
