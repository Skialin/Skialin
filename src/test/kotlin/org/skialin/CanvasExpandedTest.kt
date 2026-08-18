package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class CanvasExpandedTest {
    private fun makeImage(): Image {
        val info = ImageInfo.makeN32Premul(4, 4)
        val pixels = ByteArray(4 * 4 * 4) { 0xFF.toByte() }
        val data = Data.makeFromBytes(pixels)
        return Image.makeFromData(info, data, 16L)!!
    }

    @Test
    fun skewAndMatrixRoundtrip() {
        Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
            val canvas = surface.canvas()
            canvas.skew(0.1f, 0f)
            val matrix = canvas.getTotalMatrix()
            canvas.setMatrix(matrix)
            canvas.resetMatrix()
            val identity = canvas.getTotalMatrix()
            assertEquals(1f, identity.values[0])
        }
    }

    @Test
    fun quickRejectRectOutsideClip() {
        Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
            val canvas = surface.canvas()
            canvas.clipRect(Rect(0f, 0f, 8f, 8f))
            assertTrue(canvas.quickReject(Rect(100f, 100f, 200f, 200f)))
            assertFalse(canvas.quickReject(Rect(0f, 0f, 4f, 4f)))
        }
    }

    @Test
    fun drawRoundRectAndArc() {
        Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
            val canvas = surface.canvas()
            Paint().use { paint ->
                paint.color = Colors.RED
                canvas.drawRoundRect(Rect(0f, 0f, 16f, 16f), 2f, 2f, paint)
                canvas.drawArc(Rect(0f, 0f, 16f, 16f), 0f, 180f, true, paint)
            }
        }
    }

    @Test
    fun drawPointsVariants() {
        Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
            val canvas = surface.canvas()
            Paint().use { paint ->
                paint.color = Colors.BLUE
                val pts = arrayOf(Point(1f, 1f), Point(5f, 5f), Point(10f, 2f))
                canvas.drawPoints(PointMode.POINTS, pts, paint)
                canvas.drawPoints(PointMode.LINES, pts, paint)
                canvas.drawPoints(PointMode.POLYGON, pts, paint)
            }
        }
    }

    @Test
    fun drawImageAndImageRect() {
        makeImage().use { image ->
            Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
                val canvas = surface.canvas()
                canvas.drawImage(image, 0f, 0f)
                Paint().use { paint ->
                    paint.isAntiAlias = true
                    canvas.drawImageRect(image, Rect(0f, 0f, 16f, 16f), paint = paint, constraint = SrcRectConstraint.FAST)
                    canvas.drawImageRect(image, Rect(0f, 0f, 8f, 8f), src = Rect(0f, 0f, 2f, 2f), paint = paint)
                }
            }
        }
    }

    @Test
    fun saveLayerReturnsIncrementingCount() {
        Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
            val canvas = surface.canvas()
            val countBefore = canvas.save()
            val layerCount = canvas.saveLayer(Rect(0f, 0f, 16f, 16f))
            assertTrue(layerCount > countBefore)
            canvas.restore()
        }
    }
}
