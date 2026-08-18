package org.skialin

import kotlin.test.Test
import kotlin.test.assertNotNull

class FiltersTest {
    @Test
    fun colorFilterBlendIsUsable() {
        ColorFilter.makeBlend(Colors.RED)!!.use { filter ->
            Paint().use { paint ->
                paint.setColorFilter(filter)
                paint.setColorFilter(null)
            }
        }
    }

    @Test
    fun colorFilterMatrixIsUsable() {
        val identity = floatArrayOf(
            1f, 0f, 0f, 0f, 0f,
            0f, 1f, 0f, 0f, 0f,
            0f, 0f, 1f, 0f, 0f,
            0f, 0f, 0f, 1f, 0f,
        )
        ColorFilter.makeMatrix(identity)!!.use { filter ->
            Paint().use { paint -> paint.setColorFilter(filter) }
        }
    }

    @Test
    fun colorFilterComposeAndLerp() {
        ColorFilter.makeBlend(Colors.RED)!!.use { a ->
            ColorFilter.makeBlend(Colors.BLUE)!!.use { b ->
                ColorFilter.makeCompose(a, b).use { assertNotNull(it) }
                ColorFilter.makeLerp(0.5f, a, b).use { assertNotNull(it) }
            }
        }
    }

    @Test
    fun imageFilterBlurDrawsWithoutCrashing() {
        ImageFilter.makeBlur(4f, 4f)!!.use { filter ->
            Paint().use { paint ->
                paint.setImageFilter(filter)
                Surface.makeRasterN32Premul(32, 32)!!.use { surface ->
                    surface.canvas().drawRect(Rect(4f, 4f, 20f, 20f), paint)
                }
            }
        }
    }

    @Test
    fun imageFilterComposeChain() {
        ImageFilter.makeBlur(2f, 2f)!!.use { blur ->
            ImageFilter.makeOffset(3f, 3f, blur)!!.use { offset ->
                ImageFilter.makeCompose(offset, blur).use { assertNotNull(it) }
            }
        }
    }

    @Test
    fun imageFilterDropShadowAndMorphology() {
        ImageFilter.makeDropShadow(2f, 2f, 3f, 3f, Colors.BLACK).use { assertNotNull(it) }
        ImageFilter.makeDropShadowOnly(2f, 2f, 3f, 3f, Colors.BLACK).use { assertNotNull(it) }
        ImageFilter.makeDilate(2f, 2f).use { assertNotNull(it) }
        ImageFilter.makeErode(2f, 2f).use { assertNotNull(it) }
    }

    @Test
    fun imageFilterMatrixTransform() {
        val matrix = Matrix33.makeTranslate(5f, 5f)
        ImageFilter.makeMatrixTransform(matrix).use { assertNotNull(it) }
    }

    @Test
    fun maskFilterBlurDrawsWithoutCrashing() {
        MaskFilter.makeBlur(MaskFilter.BlurStyle.NORMAL, 3f)!!.use { filter ->
            Paint().use { paint ->
                paint.setMaskFilter(filter)
                Surface.makeRasterN32Premul(32, 32)!!.use { surface ->
                    surface.canvas().drawCircle(Point(16f, 16f), 8f, paint)
                }
            }
        }
    }
}
