package org.skialin

import java.nio.ByteBuffer
import org.lwjgl.glfw.GLFW.GLFW_CLIENT_API
import org.lwjgl.glfw.GLFW.GLFW_OPENGL_API
import org.lwjgl.glfw.GLFW.GLFW_VISIBLE
import org.lwjgl.glfw.GLFW.glfwCreateWindow
import org.lwjgl.glfw.GLFW.glfwDestroyWindow
import org.lwjgl.glfw.GLFW.glfwInit
import org.lwjgl.glfw.GLFW.glfwMakeContextCurrent
import org.lwjgl.glfw.GLFW.glfwTerminate
import org.lwjgl.glfw.GLFW.glfwWindowHint
import org.lwjgl.opengl.GL
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlin.test.fail

/** Ganesh + OpenGL smoke test. GLFW provides the native GL context that
 * [DirectContext.makeGL] wraps; production callers do the same via
 * LWJGL/GLFW in their own windowing setup. */
class DirectContextTest {
    private fun withGlContext(block: () -> Unit) {
        check(glfwInit()) { "glfwInit failed" }
        try {
            glfwWindowHint(GLFW_VISIBLE, 0)
            glfwWindowHint(GLFW_CLIENT_API, GLFW_OPENGL_API)
            val window = glfwCreateWindow(1, 1, "skialin-test", 0, 0)
            check(window != 0L) { "glfwCreateWindow failed" }
            try {
                glfwMakeContextCurrent(window)
                GL.createCapabilities()
                block()
            } finally {
                glfwDestroyWindow(window)
            }
        } finally {
            glfwTerminate()
        }
    }

    @Test
    fun renderTargetRoundTrip() = withGlContext {
        val context = DirectContext.makeGL() ?: fail("DirectContext.makeGL failed -- no GL driver current?")
        context.use {
            val info = ImageInfo.make(16, 16, ColorType.N32, AlphaType.PREMUL)
            val surface = Surface.makeRenderTarget(
                context, budgeted = false, info = info, sampleCount = 0, surfaceOrigin = SurfaceOrigin.TOP_LEFT,
            ) ?: fail("makeRenderTarget failed")
            surface.use {
                surface.canvas().clear(Colors.RED)
                context.flush()
                context.submit(syncCpu = true)

                surface.imageSnapshot()!!.use { image ->
                    assertTrue(image.isTextureBacked)

                    val buffer = ByteBuffer.allocateDirect(16 * 16 * 4)
                    assertTrue(image.readPixels(info, buffer, 16L * 4))

                    // ColorType.N32 is BGRA_8888: opaque red -> B=0, G=0, R=255, A=255.
                    assertEquals(0, buffer.get(0).toInt() and 0xFF)
                    assertEquals(0, buffer.get(1).toInt() and 0xFF)
                    assertEquals(255, buffer.get(2).toInt() and 0xFF)
                    assertEquals(255, buffer.get(3).toInt() and 0xFF)
                }
            }
        }
    }
}
