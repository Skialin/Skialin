package org.skialin

import java.nio.ByteBuffer
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

class ImageTest {
    private fun redSquareImage(): Image {
        val surface = Surface.makeRasterN32Premul(16, 16)!!
        surface.canvas().clear(Colors.RED)
        return surface.imageSnapshot()!!
    }

    /** isOpaque reflects the alpha-type tag, not actual pixel content. */
    private fun opaqueRedSquareImage(): Image {
        val info = ImageInfo.make(16, 16, ColorType.N32, AlphaType.OPAQUE)
        val surface = Surface.makeRaster(info)!!
        surface.canvas().clear(Colors.RED)
        return surface.imageSnapshot()!!
    }

    @Test
    fun basicProperties() {
        redSquareImage().use { image ->
            assertEquals(16, image.width)
            assertEquals(16, image.height)
            assertEquals(ISize(16, 16), image.dimensions)
            assertEquals(IRect(0, 0, 16, 16), image.bounds)
            assertTrue(image.uniqueId != 0)
            assertFalse(image.isAlphaOnly)
            assertFalse(image.isTextureBacked)
            assertFalse(image.isProtected)
        }
    }

    @Test
    fun opaqueAlphaTypeReportsOpaque() {
        opaqueRedSquareImage().use { image ->
            assertTrue(image.isOpaque)
            assertEquals(AlphaType.OPAQUE, image.alphaType)
        }
    }

    @Test
    fun imageInfoMatchesSource() {
        redSquareImage().use { image ->
            image.imageInfo.use { info ->
                assertEquals(16, info.width)
                assertEquals(16, info.height)
            }
        }
    }

    @Test
    fun encodeAndDecodeRoundTrip() {
        redSquareImage().use { image ->
            val png = image.encodeToPng()!!
            assertTrue(png.isNotEmpty())

            Image.decode(png)!!.use { decoded ->
                assertEquals(16, decoded.width)
                assertEquals(16, decoded.height)
                assertNotNull(decoded.refEncodedData())
            }
            assertNull(image.refEncodedData())
        }
    }

    @Test
    fun peekPixelsAndReadPixels() {
        redSquareImage().use { image ->
            image.peekPixels()!!.use { pixmap ->
                assertEquals(16, pixmap.width)
                assertEquals(Colors.RED, pixmap.getColor(0, 0))
            }

            ImageInfo.makeN32Premul(16, 16).use { info ->
                val rowBytes = info.minRowBytes
                val buffer = ByteBuffer.allocateDirect((rowBytes * 16).toInt())
                val ok = image.readPixels(info, buffer, rowBytes)
                assertTrue(ok)
            }
        }
    }

    @Test
    fun makeSubsetAndScaled() {
        redSquareImage().use { image ->
            image.makeSubset(IRect(0, 0, 8, 8))!!.use { subset ->
                assertEquals(8, subset.width)
                assertEquals(8, subset.height)
            }

            ImageInfo.makeN32Premul(32, 32).use { info ->
                image.makeScaled(info, SamplingOptions.LINEAR)!!.use { scaled ->
                    assertEquals(32, scaled.width)
                    assertEquals(32, scaled.height)
                }
            }
        }
    }

    @Test
    fun asLegacyBitmapRoundTrip() {
        redSquareImage().use { image ->
            image.asLegacyBitmap()!!.use { bitmap ->
                assertEquals(16, bitmap.width)
                assertEquals(16, bitmap.height)
            }
        }
    }

    @Test
    fun makeShaderFromImage() {
        opaqueRedSquareImage().use { image ->
            image.makeShader()!!.use { shader ->
                assertTrue(shader.isOpaque)
            }
        }
    }

    @Test
    fun fromPixmapCopyAndFromData() {
        redSquareImage().use { image ->
            image.peekPixels()!!.use { pixmap ->
                Image.makeFromPixmapCopy(pixmap)!!.use { copy ->
                    assertEquals(16, copy.width)
                }
            }

            ImageInfo.make(2, 2, ColorType.RGBA_8888, AlphaType.PREMUL).use { info ->
                Data.makeFromBytes(
                    byteArrayOf(
                        255.toByte(), 0, 0, 255.toByte(),
                        255.toByte(), 0, 0, 255.toByte(),
                        255.toByte(), 0, 0, 255.toByte(),
                        255.toByte(), 0, 0, 255.toByte(),
                    ),
                ).use { pixels ->
                    Image.makeFromData(info, pixels, 8)!!.use { fromData ->
                        assertEquals(2, fromData.width)
                        assertEquals(2, fromData.height)
                    }
                    // pixels must stay independently usable: makeFromData refs, doesn't consume.
                    assertEquals(16L, pixels.size)
                }
            }
        }
    }

    @Test
    fun colorSpaceRoundTrip() {
        redSquareImage().use { image ->
            ColorSpace.makeSRGB().use { srgb ->
                image.makeColorSpace(srgb)?.use { }
                image.reinterpretColorSpace(srgb)!!.use { }
            }
        }
    }
}
