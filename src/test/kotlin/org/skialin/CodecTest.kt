package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import kotlin.test.assertTrue

class CodecTest {
    private fun makeSolidImage(): Image {
        val info = ImageInfo.makeN32Premul(8, 8)
        val pixels = ByteArray(8 * 8 * 4) { i -> if (i % 4 == 2) 0xFF.toByte() else 0x00.toByte() }
        val data = Data.makeFromBytes(pixels)
        return Image.makeFromData(info, data, 8L * 4)!!
    }

    @Test
    fun encodesAndDecodesJpeg() {
        makeSolidImage().use { image ->
            val bytes = image.encodeToJpeg(90)!!
            assertTrue(bytes.isNotEmpty())

            Codec.makeFromBytes(bytes)!!.use { codec ->
                assertEquals(ISize(8, 8), codec.dimensions)
                assertEquals(3, codec.encodedFormat)
                assertEquals(1, codec.frameCount)
            }
        }
    }

    @Test
    fun encodesAndDecodesWebp() {
        makeSolidImage().use { image ->
            val bytes = image.encodeToWebp(90f, false)!!
            assertTrue(bytes.isNotEmpty())

            Codec.makeFromBytes(bytes)!!.use { codec ->
                assertEquals(ISize(8, 8), codec.dimensions)
                assertEquals(6, codec.encodedFormat)

                val info = ImageInfo.make(8, 8, ColorType.N32, AlphaType.PREMUL)
                val buffer = java.nio.ByteBuffer.allocateDirect(8 * 8 * 4)
                assertTrue(codec.getPixels(info, buffer, 8L * 4))
            }
        }
    }

    @Test
    fun invalidBytesReturnsNull() {
        assertNull(Codec.makeFromBytes("not an image".toByteArray()))
    }
}
