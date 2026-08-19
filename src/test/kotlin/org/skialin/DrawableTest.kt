package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

private class RedSquareDrawable : Drawable() {
    var drawCalls = 0
    var boundsCalls = 0

    override fun onDraw(canvas: Canvas) {
        drawCalls++
        Paint().use { paint ->
            paint.color = Colors.RED
            canvas.drawRect(Rect(0f, 0f, 16f, 16f), paint)
        }
    }

    override fun onGetBounds(): Rect {
        boundsCalls++
        return Rect(0f, 0f, 16f, 16f)
    }
}

class DrawableTest {
    @Test
    fun onDrawIsCalledAndPixelsMatch() {
        RedSquareDrawable().use { drawable ->
            Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
                surface.canvas().clear(Colors.WHITE)
                surface.canvas().drawDrawable(drawable)

                surface.imageSnapshot()!!.use { image ->
                    val info = ImageInfo.make(16, 16, ColorType.N32, AlphaType.PREMUL)
                    val buffer = java.nio.ByteBuffer.allocateDirect(16 * 16 * 4)
                    image.readPixels(info, buffer, 16L * 4)
                    // BGRA8888: opaque red -> B=0, G=0, R=255, A=255.
                    assertEquals(0, buffer.get(0).toInt() and 0xFF)
                    assertEquals(255, buffer.get(2).toInt() and 0xFF)
                }
            }
            assertEquals(1, drawable.drawCalls)
        }
    }

    @Test
    fun drawnThroughAPicture() {
        RedSquareDrawable().use { drawable ->
            PictureRecorder().use { recorder ->
                val canvas = recorder.beginRecording(Rect(0f, 0f, 16f, 16f))
                canvas.drawDrawable(drawable)
                recorder.finishRecordingAsPicture()!!.use { picture ->
                    Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
                        surface.canvas().clear(Colors.WHITE)
                        picture.playback(surface.canvas())

                        surface.imageSnapshot()!!.use { image ->
                            val info = ImageInfo.make(16, 16, ColorType.N32, AlphaType.PREMUL)
                            val buffer = java.nio.ByteBuffer.allocateDirect(16 * 16 * 4)
                            image.readPixels(info, buffer, 16L * 4)
                            assertEquals(255, buffer.get(2).toInt() and 0xFF)
                        }
                    }
                }
            }
        }
    }

    @Test
    fun boundsMakePictureSnapshotAndNotifyDrawingChanged() {
        RedSquareDrawable().use { drawable ->
            assertEquals(Rect(0f, 0f, 16f, 16f), drawable.bounds)

            val genBefore = drawable.generationId
            drawable.notifyDrawingChanged()
            assertTrue(drawable.generationId != genBefore)

            drawable.makePictureSnapshot()!!.use { picture -> assertNotNull(picture) }
        }
    }
}
