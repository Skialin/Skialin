package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

class PaintExpandedTest {
    @Test
    fun resetRestoresDefaults() {
        Paint().use { paint ->
            paint.color = Colors.RED
            paint.reset()
            assertEquals(Colors.BLACK, paint.color)
        }
    }

    @Test
    fun ditherRoundtrips() {
        Paint().use { paint ->
            paint.isDither = true
            assertTrue(paint.isDither)
        }
    }

    @Test
    fun alphaRoundtrips() {
        Paint().use { paint ->
            paint.alpha = 128
            assertEquals(128, paint.alpha)
            paint.alphaf = 0.5f
            assertTrue(kotlin.math.abs(paint.alphaf - 0.5f) < 0.01f)
        }
    }

    @Test
    fun setArgbSetsColor() {
        Paint().use { paint ->
            paint.setARGB(255, 10, 20, 30)
            assertEquals(Colors.argb(255, 10, 20, 30), paint.color)
        }
    }

    @Test
    fun strokeMiterRoundtrips() {
        Paint().use { paint ->
            paint.strokeMiter = 2.5f
            assertEquals(2.5f, paint.strokeMiter)
        }
    }

    @Test
    fun nothingToDrawAndIsSrcOver() {
        Paint().use { paint ->
            assertFalse(paint.nothingToDraw)
            assertTrue(paint.isSrcOver)
            paint.alpha = 0
            assertTrue(paint.nothingToDraw)
        }
    }

    @Test
    fun getterRoundtrips() {
        Paint().use { paint ->
            assertNull(paint.getShader())
            ColorFilter.makeBlend(Colors.RED)!!.use { filter ->
                paint.setColorFilter(filter)
                paint.getColorFilter().use { assertNotNull(it) }
            }
        }
    }

    @Test
    fun pathEffectAndBlendModeGettersRoundtrip() {
        Paint().use { paint ->
            assertNull(paint.getPathEffect())
            assertEquals(BlendMode.SRC_OVER, paint.blendMode)

            PathEffect.makeCorner(3f)!!.use { effect ->
                paint.setPathEffect(effect)
                paint.getPathEffect().use { assertNotNull(it) }
            }

            paint.setBlendMode(BlendMode.SCREEN)
            assertEquals(BlendMode.SCREEN, paint.blendMode)
        }
    }
}
