package org.skialin

import java.nio.ByteBuffer
import java.nio.ByteOrder
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue

class PixmapTest {
    @Test
    fun wrapsBufferAndReadsBack() {
        ImageInfo.make(4, 4, ColorType.RGBA_8888, AlphaType.PREMUL).use { info ->
            val rowBytes = info.minRowBytes
            val buffer = ByteBuffer.allocateDirect((rowBytes * 4).toInt()).order(ByteOrder.nativeOrder())
            // pixel (1,1) = opaque red, RGBA byte order.
            val offset = (rowBytes * 1 + 4 * 1).toInt()
            buffer.put(offset, 255.toByte())
            buffer.put(offset + 3, 255.toByte())

            Pixmap.make(info, buffer, rowBytes).use { pixmap ->
                assertEquals(4, pixmap.width)
                assertEquals(4, pixmap.height)
                assertFalse(pixmap.isEmpty)
                assertEquals(ColorType.RGBA_8888, pixmap.colorType)
                assertEquals(rowBytes, pixmap.rowBytes)

                assertEquals(Colors.argb(0xff, 0xff, 0, 0), pixmap.getColor(1, 1))

                pixmap.extractSubset(IRect(1, 1, 3, 3))!!.use { subset ->
                    assertEquals(2, subset.width)
                    assertEquals(2, subset.height)
                    assertEquals(Colors.argb(0xff, 0xff, 0, 0), subset.getColor(0, 0))
                }

                assertNull(pixmap.extractSubset(IRect(10, 10, 20, 20)))
            }
        }
    }

    @Test
    fun readPixelsCopiesIntoDestination() {
        ImageInfo.make(4, 4, ColorType.RGBA_8888, AlphaType.PREMUL).use { info ->
            val rowBytes = info.minRowBytes
            val buffer = ByteBuffer.allocateDirect((rowBytes * 4).toInt()).order(ByteOrder.nativeOrder())
            val offset = (rowBytes * 1 + 4 * 1).toInt()
            buffer.put(offset, 255.toByte())
            buffer.put(offset + 3, 255.toByte())

            Pixmap.make(info, buffer, rowBytes).use { src ->
                ImageInfo.make(2, 2, ColorType.RGBA_8888, AlphaType.PREMUL).use { dstInfo ->
                    val dstRowBytes = dstInfo.minRowBytes
                    val dstBuffer = ByteBuffer.allocateDirect((dstRowBytes * 2).toInt()).order(ByteOrder.nativeOrder())
                    Pixmap.make(dstInfo, dstBuffer, dstRowBytes).use { dst ->
                        assertTrue(src.readPixels(dst, 1, 1))
                        assertEquals(Colors.argb(0xff, 0xff, 0, 0), dst.getColor(0, 0))
                    }
                }
            }
        }
    }

    @Test
    fun scalePixelsFillsDestination() {
        ImageInfo.make(4, 4, ColorType.RGBA_8888, AlphaType.PREMUL).use { info ->
            val rowBytes = info.minRowBytes
            val buffer = ByteBuffer.allocateDirect((rowBytes * 4).toInt()).order(ByteOrder.nativeOrder())
            for (i in 0 until buffer.capacity() / 4) {
                buffer.put(i * 4, 255.toByte())
                buffer.put(i * 4 + 3, 255.toByte())
            }

            Pixmap.make(info, buffer, rowBytes).use { src ->
                ImageInfo.make(2, 2, ColorType.RGBA_8888, AlphaType.PREMUL).use { dstInfo ->
                    val dstRowBytes = dstInfo.minRowBytes
                    val dstBuffer = ByteBuffer.allocateDirect((dstRowBytes * 2).toInt()).order(ByteOrder.nativeOrder())
                    Pixmap.make(dstInfo, dstBuffer, dstRowBytes).use { dst ->
                        assertTrue(src.scalePixels(dst))
                        assertEquals(Colors.argb(0xff, 0xff, 0, 0), dst.getColor(0, 0))
                    }
                }
            }
        }
    }
}
