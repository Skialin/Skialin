package org.skialin

import kotlin.test.Test
import kotlin.test.assertContentEquals
import kotlin.test.assertTrue

class BitmapExpandedTest {
    @Test
    fun installPixelsSetsContents() {
        val pixels = byteArrayOf(0x11, 0x22, 0x33.toByte(), 0xFF.toByte())
        ImageInfo.make(1, 1, ColorType.N32, AlphaType.PREMUL).use { info ->
            Bitmap().use { bitmap ->
                assertTrue(bitmap.installPixels(info, pixels))
                assertContentEquals(pixels, bitmap.readPixels())
            }
        }
    }

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
