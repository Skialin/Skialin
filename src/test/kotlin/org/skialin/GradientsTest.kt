package org.skialin

import kotlin.test.Test
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

class GradientsTest {
    @Test
    fun linearGradientIsOpaqueForOpaqueColors() {
        Shader.makeLinearGradient(Point(0f, 0f), Point(10f, 0f), intArrayOf(Colors.RED, Colors.BLUE))!!.use {
            assertTrue(it.isOpaque)
        }
    }

    @Test
    fun linearGradientWithPositions() {
        val shader = Shader.makeLinearGradient(
            Point(0f, 0f), Point(10f, 0f),
            intArrayOf(Colors.RED, Colors.GREEN, Colors.BLUE),
            floatArrayOf(0f, 0.25f, 1f),
        )
        assertNotNull(shader)
        shader.close()
    }

    @Test
    fun radialGradientDrawsWithoutCrashing() {
        Shader.makeRadialGradient(Point(16f, 16f), 10f, intArrayOf(Colors.RED, Colors.BLUE))!!.use { shader ->
            Paint().use { paint ->
                paint.setShader(shader)
                Surface.makeRasterN32Premul(32, 32)!!.use { surface ->
                    surface.canvas().drawRect(Rect(0f, 0f, 32f, 32f), paint)
                }
            }
        }
    }

    @Test
    fun radialGradientNullForEmptyColors() {
        assertNull(Shader.makeRadialGradient(Point(0f, 0f), 10f, intArrayOf()))
    }

    @Test
    fun twoPointConicalGradientDrawsWithoutCrashing() {
        Shader.makeTwoPointConicalGradient(Point(8f, 8f), 4f, Point(16f, 16f), 12f, intArrayOf(Colors.RED, Colors.BLUE))!!.use { shader ->
            Paint().use { paint ->
                paint.setShader(shader)
                Surface.makeRasterN32Premul(32, 32)!!.use { surface ->
                    surface.canvas().drawRect(Rect(0f, 0f, 32f, 32f), paint)
                }
            }
        }
    }

    @Test
    fun sweepGradientDrawsWithoutCrashing() {
        Shader.makeSweepGradient(Point(16f, 16f), intArrayOf(Colors.RED, Colors.GREEN, Colors.BLUE))!!.use { shader ->
            Paint().use { paint ->
                paint.setShader(shader)
                Surface.makeRasterN32Premul(32, 32)!!.use { surface ->
                    surface.canvas().drawRect(Rect(0f, 0f, 32f, 32f), paint)
                }
            }
        }
    }

    @Test
    fun gradientWithLocalMatrix() {
        val matrix = Matrix33.makeTranslate(5f, 5f)
        val shader = Shader.makeLinearGradient(Point(0f, 0f), Point(10f, 0f), intArrayOf(Colors.RED, Colors.BLUE), localMatrix = matrix)
        assertNotNull(shader)
        shader.close()
    }
}
