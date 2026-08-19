package org.skialin

import kotlin.test.Test
import kotlin.test.assertNotNull

class BlenderTest {
    @Test
    fun modeBlenderAttachesToPaintAndDraws() {
        Blender.mode(BlendMode.SRC_OVER).use { blender ->
            Paint().use { paint ->
                paint.setBlender(blender)
                assertNotNull(paint.getBlender())
                Surface.makeRasterN32Premul(8, 8)!!.use { surface ->
                    surface.canvas.drawRect(Rect(0f, 0f, 8f, 8f), paint)
                }
            }
        }
    }

    @Test
    fun shaderBlendWithBlender() {
        Blender.mode(BlendMode.SRC_OVER).use { blender ->
            Shader.makeColor(Colors.RED).use { dst ->
                Shader.makeColor(Colors.BLUE).use { src ->
                    val blended = Shader.makeBlend(blender, dst, src)
                    assertNotNull(blended)
                    blended.use {}
                }
            }
        }
    }

    @Test
    fun imageFilterBlendWithBlender() {
        Blender.mode(BlendMode.SRC_OVER).use { blender ->
            val filter = ImageFilter.makeBlend(blender)
            assertNotNull(filter)
            filter.use {}
        }
    }

    @Test
    fun runtimeEffectMakeBlender() {
        val sksl =
            """
            vec4 main(vec4 src, vec4 dst) {
                return src;
            }
            """.trimIndent()
        RuntimeEffect.makeForBlender(sksl).use { effect ->
            val blender = effect.makeBlender()
            assertNotNull(blender)
            blender.use { paint ->
                Paint().use { it.setBlender(paint) }
            }
        }
    }
}
