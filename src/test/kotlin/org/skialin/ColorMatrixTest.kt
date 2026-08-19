package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull

class ColorMatrixTest {
    @Test
    fun identityIsTheDefault20Floats() {
        val expected =
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
        assertEquals(expected.toList(), ColorMatrix.identity().toList())
    }

    @Test
    fun saturationZeroIsUsableInAFilter() {
        val m = ColorMatrix.makeSaturation(0f)
        ColorFilter.makeMatrix(m)!!.use { filter ->
            Paint().use { paint -> paint.setColorFilter(filter) }
        }
    }

    @Test
    fun scaleSetsDiagonal() {
        val m = ColorMatrix.makeScale(0.5f, 0.6f, 0.7f, 1f)
        assertEquals(0.5f, m[0])
        assertEquals(0.6f, m[6])
        assertEquals(0.7f, m[12])
        assertEquals(1f, m[18])
    }

    @Test
    fun postTranslateSetsLastColumn() {
        val m = ColorMatrix.postTranslate(ColorMatrix.identity(), 0.1f, 0.2f, 0.3f, 0f)
        assertEquals(0.1f, m[4])
        assertEquals(0.2f, m[9])
        assertEquals(0.3f, m[14])
    }

    @Test
    fun concatIsUsable() {
        val a = ColorMatrix.makeScale(0.5f, 0.5f, 0.5f, 1f)
        val b = ColorMatrix.makeSaturation(0f)
        val combined = ColorMatrix.concat(a, b)
        ColorFilter.makeMatrix(combined).use { assertNotNull(it) }
    }
}
