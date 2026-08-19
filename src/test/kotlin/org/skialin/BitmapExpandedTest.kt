package org.skialin

import kotlin.test.Test
import kotlin.test.assertTrue

class BitmapExpandedTest {
    @Test
    fun extractSubsetCopiesRegion() {
        Bitmap().use { src ->
            src.allocPixels(8, 8)
            src.eraseColor(Colors.RED)
            Bitmap().use { dst -> assertTrue(src.extractSubset(dst, IRect(0, 0, 4, 4))) }
        }
    }

    @Test
    fun extractAlphaProducesAlphaBitmap() {
        Bitmap().use { src ->
            src.allocPixels(4, 4)
            src.eraseColor(Colors.RED)
            Bitmap().use { dst -> assertTrue(src.extractAlpha(dst)) }
        }
    }

    @Test
    fun notifyPixelsChangedDoesNotCrash() {
        Bitmap().use { bitmap ->
            bitmap.allocPixels(4, 4)
            bitmap.notifyPixelsChanged()
        }
    }
}
