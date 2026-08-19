package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull

class SkottieAnimationTest {
    private val minimalLottie =
        """
        {
          "v": "5.5.2", "fr": 30, "ip": 0, "op": 30, "w": 16, "h": 16, "nm": "test",
          "layers": [
            { "ty": 4, "nm": "rect", "ip": 0, "op": 30, "st": 0,
              "ks": { "o": { "a": 0, "k": 100 }, "p": { "a": 0, "k": [8, 8] }, "s": { "a": 0, "k": [100, 100] }, "r": { "a": 0, "k": 0 }, "a": { "a": 0, "k": [0, 0] } },
              "shapes": [
                { "ty": "rc", "p": { "a": 0, "k": [0, 0] }, "s": { "a": 0, "k": [16, 16] }, "r": { "a": 0, "k": 0 } },
                { "ty": "fl", "c": { "a": 0, "k": [1, 0, 0, 1] }, "o": { "a": 0, "k": 100 } }
              ]
            }
          ]
        }
        """.trimIndent().toByteArray()

    @Test
    fun parsesAndReportsMetadata() {
        SkottieAnimation.makeFromBytes(minimalLottie)!!.use { animation ->
            assertEquals(16f to 16f, animation.size)
            assertEquals(1.0, animation.duration)
            assertEquals(30.0, animation.fps)
        }
    }

    @Test
    fun rendersWithoutCrashing() {
        SkottieAnimation.makeFromBytes(minimalLottie)!!.use { animation ->
            animation.seek(0.5f)
            animation.seekFrame(15.0)
            Surface.makeRasterN32Premul(16, 16)!!.use { surface ->
                animation.render(surface.canvas, Rect(0f, 0f, 16f, 16f))
            }
        }
    }

    @Test
    fun invalidBytesReturnsNull() {
        assertNull(SkottieAnimation.makeFromBytes("not json".toByteArray()))
    }
}
