package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals

class SurfaceExpandedTest {
    @Test
    fun widthHeightAndImageInfoMatchConstructionSize() {
        Surface.makeRasterN32Premul(12, 8)!!.use { surface ->
            assertEquals(12, surface.width)
            assertEquals(8, surface.height)
            surface.imageInfo.use { info ->
                assertEquals(12, info.width)
                assertEquals(8, info.height)
            }
        }
    }

    @Test
    fun imageSnapshotAreaCropsToBounds() {
        Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
            surface.canvas.clear(Colors.RED)
            surface.makeImageSnapshotArea(IRect(0, 0, 4, 4))!!.use { image ->
                assertEquals(4, image.width)
                assertEquals(4, image.height)
            }
        }
    }

    @Test
    fun notifyContentWillChangeAndFlushDoNotCrash() {
        Surface.makeRasterN32Premul(4, 4)!!.use { surface ->
            surface.notifyContentWillChange()
            surface.flush()
        }
    }

    @Test
    fun drawOntoAnotherCanvasDoesNotCrash() {
        Surface.makeRasterN32Premul(4, 4)!!.use { src ->
            src.canvas.clear(Colors.BLUE)
            Surface.makeRasterN32Premul(8, 8)!!.use { dst -> src.draw(dst.canvas, 1f, 1f) }
        }
    }
}
