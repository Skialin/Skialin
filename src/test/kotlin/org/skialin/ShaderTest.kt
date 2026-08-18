package org.skialin

import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class ShaderTest {
    @Test
    fun colorShaderOpacityMatchesAlpha() {
        Shader.makeColor(Colors.RED).use { assertTrue(it.isOpaque) }
        Shader.makeColor(Colors.argb(0x80, 0xff, 0, 0)).use { assertFalse(it.isOpaque) }
    }

    @Test
    fun withLocalMatrixProducesUsableShader() {
        Shader.makeColor(Colors.BLUE).use { shader ->
            shader.withLocalMatrix(Matrix33.makeTranslate(5f, 5f)).use { moved ->
                assertTrue(moved.isOpaque)
            }
        }
    }

    @Test
    fun attachesToPaintAndDraws() {
        Surface.makeRasterN32Premul(8, 8)!!.use { surface ->
            val canvas = surface.canvas()
            Paint().use { paint ->
                Shader.makeColor(Colors.GREEN).use { shader ->
                    paint.setShader(shader)
                    canvas.drawRect(Rect(0f, 0f, 8f, 8f), paint)
                }
                paint.setShader(null)
            }
        }
    }
}
