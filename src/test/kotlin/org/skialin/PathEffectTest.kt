package org.skialin

import kotlin.test.Test
import kotlin.test.assertNotNull
import kotlin.test.assertNull

class PathEffectTest {
    private fun makeSquarePath(): Path {
        val builder = PathBuilder()
        builder.addRect(Rect(0f, 0f, 20f, 20f))
        return builder.snapshot()
    }

    @Test
    fun dashDrawsWithoutCrashing() {
        PathEffect.makeDash(floatArrayOf(4f, 2f))!!.use { effect ->
            Paint().use { paint ->
                paint.style = PaintStyle.STROKE
                paint.setPathEffect(effect)
                Surface.makeRasterN32Premul(32, 32)!!.use { surface ->
                    surface.canvas().drawPath(makeSquarePath(), paint)
                }
            }
        }
    }

    @Test
    fun dashNullForOddIntervals() {
        assertNull(PathEffect.makeDash(floatArrayOf(4f, 2f, 1f)))
    }

    @Test
    fun cornerAndDiscreteAreUsable() {
        PathEffect.makeCorner(3f)!!.use { corner ->
            PathEffect.makeDiscrete(5f, 2f)!!.use { discrete ->
                Paint().use { paint ->
                    paint.setPathEffect(corner)
                    paint.setPathEffect(discrete)
                }
            }
        }
    }

    @Test
    fun trimModesAreUsable() {
        PathEffect.makeTrim(0.25f, 0.75f, PathEffect.TrimMode.NORMAL)!!.use { assertNotNull(it) }
        PathEffect.makeTrim(0.25f, 0.75f, PathEffect.TrimMode.INVERTED)!!.use { assertNotNull(it) }
    }

    @Test
    fun composeAndSum() {
        PathEffect.makeDash(floatArrayOf(4f, 2f))!!.use { dash ->
            PathEffect.makeCorner(3f)!!.use { corner ->
                PathEffect.makeCompose(dash, corner).use { assertNotNull(it) }
            }
        }
        PathEffect.makeDash(floatArrayOf(4f, 2f))!!.use { dash2 ->
            PathEffect.makeCorner(3f)!!.use { corner2 ->
                PathEffect.makeSum(dash2, corner2).use { assertNotNull(it) }
            }
        }
    }

    @Test
    fun path1DPath2DAndLine2DAreUsable() {
        val dot = PathBuilder().use { it.addCircle(Point(2f, 2f), 2f); it.snapshot() }
        dot.use {
            PathEffect.makePath1D(dot, 8f, 0f, Path1DStyle.ROTATE)!!.use { assertNotNull(it) }
            PathEffect.makePath2D(Matrix33.makeScale(8f, 8f), dot)!!.use { assertNotNull(it) }
        }
        PathEffect.makeLine2D(1f, Matrix33.makeScale(4f, 4f))!!.use { assertNotNull(it) }
    }

    @Test
    fun setPathEffectNoneClears() {
        PathEffect.makeCorner(3f)!!.use { effect ->
            Paint().use { paint ->
                paint.setPathEffect(effect)
                paint.setPathEffect(null)
                paint.color = Colors.RED
            }
        }
    }
}
