package org.skialin

import kotlin.test.Test
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

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
        val identity =
            floatArrayOf(
                1f,
                0f,
                0f,
                0f,
                0f,
                0f,
                1f,
                0f,
                0f,
                0f,
                0f,
                0f,
                1f,
                0f,
                0f,
                0f,
                0f,
                0f,
                1f,
                0f,
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
                    surface.canvas.drawRect(Rect(4f, 4f, 20f, 20f), paint)
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
    fun colorFilterExpandedFactoriesAreUsable() {
        ColorFilter.makeHSLAMatrix(FloatArray(20)).use { assertNotNull(it) }
        assertNotNull(ColorFilter.linearToSRGBGamma)
        assertNotNull(ColorFilter.sRGBToLinearGamma)
        ColorFilter.makeTable(ByteArray(256)).use { assertNotNull(it) }
        ColorFilter.makeTableARGB(a = ByteArray(256)).use { assertNotNull(it) }
        ColorFilter.makeLighting(Colors.RED, Colors.BLUE).use { assertNotNull(it) }
        ColorFilter.makeHighContrast(true, ColorFilter.InvertStyle.NO_INVERT, 0.2f)!!.use { assertNotNull(it) }
        assertNotNull(ColorFilter.luma)
    }

    @Test
    fun imageFilterExpandedFactoriesAreUsable() {
        ImageFilter.makeBlend(BlendMode.SRC_OVER).use { assertNotNull(it) }
        ImageFilter.makeMerge().use { assertNotNull(it) }
        Shader.makeColor(Colors.RED).use { shader -> ImageFilter.makeShader(shader).use { assertNotNull(it) } }
        ImageFilter.makeTile(Rect(0f, 0f, 8f, 8f), Rect(0f, 0f, 16f, 16f)).use { assertNotNull(it) }
    }

    @Test
    fun shaderExpandedFactoriesAreUsable() {
        Shader.makeColor(Colors.RED).use { dst ->
            Shader.makeColor(Colors.BLUE).use { src ->
                Shader.makeBlend(BlendMode.SRC_OVER, dst, src).use { assertNotNull(it) }
            }
        }
        Shader.makeFractalNoise(0.1f, 0.1f, 2, 0f).use { assertNotNull(it) }
        Shader.makeTurbulence(0.1f, 0.1f, 2, 0f).use { assertNotNull(it) }
    }

    @Test
    fun maskFilterSigmaRadiusConversionRoundtrips() {
        val sigma = MaskFilter.convertRadiusToSigma(4f)
        assertTrue(sigma > 0f)
        val radius = MaskFilter.convertSigmaToRadius(sigma)
        assertTrue(kotlin.math.abs(radius - 4f) < 0.01f)
    }

    @Test
    fun maskFilterBlurDrawsWithoutCrashing() {
        MaskFilter.makeBlur(MaskFilter.BlurStyle.NORMAL, 3f)!!.use { filter ->
            Paint().use { paint ->
                paint.setMaskFilter(filter)
                Surface.makeRasterN32Premul(32, 32)!!.use { surface ->
                    surface.canvas.drawCircle(Point(16f, 16f), 8f, paint)
                }
            }
        }
    }
}
