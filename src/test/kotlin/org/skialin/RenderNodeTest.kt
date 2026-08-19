package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals

class RenderNodeTest {
    @Test
    fun recordAndDrawMatchesPixels() {
        RenderNodeContext().use { context ->
            RenderNode(context).use { node ->
                node.bounds = Rect(0f, 0f, 16f, 16f)
                val canvas = node.beginRecording()
                Paint().use { paint ->
                    paint.color = Colors.RED
                    canvas.drawRect(Rect(0f, 0f, 16f, 16f), paint)
                }
                node.endRecording()

                Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
                    node.drawInto(surface.canvas)
                    surface.makeImageSnapshot()!!.use { image ->
                        val info = ImageInfo.make(16, 16, ColorType.N32, AlphaType.PREMUL)
                        val buffer = java.nio.ByteBuffer.allocateDirect(16 * 16 * 4)
                        assertTrueOrFail(image.readPixels(info, buffer, 16L * 4))
                        // BGRA8888: opaque red -> B=0, G=0, R=255, A=255.
                        assertEquals(0, buffer.get(0).toInt() and 0xFF)
                        assertEquals(255, buffer.get(2).toInt() and 0xFF)
                    }
                }
            }
        }
    }

    @Test
    fun clipPathConfinesContent() {
        RenderNodeContext().use { context ->
            RenderNode(context).use { node ->
                node.bounds = Rect(0f, 0f, 16f, 16f)
                node.clip = true
                PathBuilder().use { builder ->
                    builder.addRect(Rect(0f, 0f, 8f, 16f))
                    builder.snapshot().use { path -> node.setClipPath(path) }
                }
                val canvas = node.beginRecording()
                Paint().use { paint ->
                    paint.color = Colors.RED
                    canvas.drawRect(Rect(0f, 0f, 16f, 16f), paint)
                }
                node.endRecording()

                Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
                    surface.canvas.clear(Colors.WHITE)
                    node.drawInto(surface.canvas)
                    surface.makeImageSnapshot()!!.use { image ->
                        val info = ImageInfo.make(16, 16, ColorType.N32, AlphaType.PREMUL)
                        val buffer = java.nio.ByteBuffer.allocateDirect(16 * 16 * 4)
                        assertTrueOrFail(image.readPixels(info, buffer, 16L * 4))
                        val pixelAt = { x: Int, y: Int -> (y * 16 + x) * 4 }
                        val inside = pixelAt(2, 8)
                        assertEquals(0, buffer.get(inside).toInt() and 0xFF)
                        assertEquals(255, buffer.get(inside + 2).toInt() and 0xFF)
                        val outside = pixelAt(12, 8)
                        assertEquals(255, buffer.get(outside).toInt() and 0xFF)
                        assertEquals(255, buffer.get(outside + 2).toInt() and 0xFF)
                    }
                }
            }
        }
    }
}

private fun assertTrueOrFail(value: Boolean) {
    if (!value) throw AssertionError("expected true")
}
