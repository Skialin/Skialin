package org.skialin

import java.nio.ByteBuffer
import java.nio.ByteOrder
import kotlin.test.Test
import kotlin.test.assertFailsWith
import kotlin.test.assertNotNull

class RuntimeEffectTest {
    @Test
    fun shaderEffectCompilesAndDraws() {
        val sksl = """
            vec4 main(vec2 coord) {
                return vec4(1.0, 0.0, 0.0, 1.0);
            }
        """.trimIndent()
        RuntimeEffect.makeForShader(sksl).use { effect ->
            effect.makeShader()!!.use { shader ->
                Paint().use { paint ->
                    paint.setShader(shader)
                    Surface.makeRasterN32Premul(8, 8)!!.use { surface ->
                        surface.canvas().drawRect(Rect(0f, 0f, 8f, 8f), paint)
                    }
                }
            }
        }
    }

    @Test
    fun shaderEffectWithUniform() {
        val sksl = """
            uniform half4 color;
            vec4 main(vec2 coord) {
                return color;
            }
        """.trimIndent()
        val uniforms = ByteBuffer.allocate(16).order(ByteOrder.nativeOrder())
        uniforms.putFloat(0f).putFloat(1f).putFloat(0f).putFloat(1f)

        RuntimeEffect.makeForShader(sksl).use { effect ->
            effect.makeShader(uniforms.array())!!.use { shader ->
                Paint().use { paint ->
                    paint.setShader(shader)
                    Surface.makeRasterN32Premul(8, 8)!!.use { surface ->
                        surface.canvas().drawRect(Rect(0f, 0f, 8f, 8f), paint)
                    }
                }
            }
        }
    }

    @Test
    fun colorFilterEffectCompiles() {
        val sksl = """
            vec4 main(vec4 inColor) {
                return inColor.bgra;
            }
        """.trimIndent()
        RuntimeEffect.makeForColorFilter(sksl).use { effect ->
            effect.makeColorFilter()!!.use { filter ->
                Paint().use { paint -> paint.setColorFilter(filter) }
            }
        }
    }

    @Test
    fun invalidSkslThrows() {
        assertFailsWith<IllegalArgumentException> {
            RuntimeEffect.makeForShader("this is not valid sksl")
        }
    }

    @Test
    fun shaderEffectWithChildShader() {
        val sksl = """
            uniform shader child;
            vec4 main(vec2 coord) {
                return child.eval(coord);
            }
        """.trimIndent()
        RuntimeEffect.makeForShader(sksl).use { effect ->
            Shader.makeColor(Colors.RED).use { child ->
                effect.makeShader(children = arrayOf(child))!!.use { shader ->
                    assertNotNull(shader)
                    Paint().use { paint ->
                        paint.setShader(shader)
                        Surface.makeRasterN32Premul(8, 8)!!.use { surface ->
                            surface.canvas().drawRect(Rect(0f, 0f, 8f, 8f), paint)
                        }
                    }
                }
            }
        }
    }
}
