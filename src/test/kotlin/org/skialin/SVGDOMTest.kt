package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import kotlin.test.assertTrue

class SVGDOMTest {
    private val simpleSvg =
        """
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
        <rect x="0" y="0" width="16" height="16" fill="#ff0000"/>
        </svg>
        """.trimIndent().toByteArray()

    @Test
    fun parsesAndRendersWithoutCrashing() {
        SVGDOM.makeFromBytes(simpleSvg)!!.use { dom ->
            assertEquals(16f to 16f, dom.containerSize)

            Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
                dom.render(surface.canvas)
                surface.makeImageSnapshot()!!.use { image ->
                    val info = ImageInfo.make(16, 16, ColorType.N32, AlphaType.PREMUL)
                    val buffer = java.nio.ByteBuffer.allocateDirect(16 * 16 * 4)
                    assertTrue(image.readPixels(info, buffer, 16L * 4))
                    assertEquals(0, buffer.get(0).toInt() and 0xFF)
                    assertEquals(255, buffer.get(2).toInt() and 0xFF)
                }
            }
        }
    }

    @Test
    fun invalidBytesReturnsNull() {
        assertNull(SVGDOM.makeFromBytes("not svg at all".toByteArray()))
    }

    @Test
    fun svgCanvasRecordsDrawsAsXml() {
        SVGCanvas(Rect(0f, 0f, 16f, 16f)).use { svgCanvas ->
            Paint().use { paint ->
                paint.color = Colors.RED
                svgCanvas.canvas.drawRect(Rect(0f, 0f, 16f, 16f), paint)
            }
            val xml = String(svgCanvas.finish())
            assertTrue(xml.contains("<svg"))
        }
    }
}
