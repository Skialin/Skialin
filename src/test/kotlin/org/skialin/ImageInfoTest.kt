package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class ImageInfoTest {
    @Test
    fun makeN32PremulHasExpectedProperties() {
        ImageInfo.makeN32Premul(64, 32).use { info ->
            assertEquals(64, info.width)
            assertEquals(32, info.height)
            assertEquals(ColorType.N32, info.colorType)
            assertEquals(AlphaType.PREMUL, info.alphaType)
            assertFalse(info.isEmpty)
            assertEquals(4, info.bytesPerPixel)
            assertEquals(64L * 4, info.minRowBytes)
        }
    }

    @Test
    fun withColorTypeChangesOnlyColorType() {
        ImageInfo.makeN32Premul(10, 10).use { info ->
            info.withColorType(ColorType.ALPHA_8).use { alpha8 ->
                assertEquals(ColorType.ALPHA_8, alpha8.colorType)
                assertEquals(10, alpha8.width)
                assertEquals(1, alpha8.bytesPerPixel)
            }
        }
    }

    @Test
    fun equalsComparesAllComponents() {
        ImageInfo.makeN32Premul(5, 5).use { a ->
            ImageInfo.makeN32Premul(5, 5).use { b ->
                ImageInfo.makeN32Premul(5, 6).use { c ->
                    assertTrue(a.contentEquals(b))
                    assertFalse(a.contentEquals(c))
                }
            }
        }
    }

    @Test
    fun bitmapAllocPixelsWithImageInfo() {
        Bitmap().use { bitmap ->
            ImageInfo.make(16, 16, ColorType.RGBA_8888, AlphaType.PREMUL).use { info ->
                bitmap.allocPixels(info)
            }
            assertEquals(16, bitmap.width)
            assertEquals(16, bitmap.height)
            bitmap.eraseColor(Colors.RED)
            assertEquals(16L * 4, bitmap.rowBytes)
        }
    }

    @Test
    fun surfaceMakeRasterWithImageInfo() {
        ImageInfo.makeN32Premul(20, 20).use { info ->
            Surface.makeRaster(info)!!.use { surface ->
                surface.canvas().clear(Colors.WHITE)
                surface.imageSnapshot()!!.use { image ->
                    assertEquals(20, image.width)
                    assertEquals(20, image.height)
                }
            }
        }
    }
}
