package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class Matrix33ExpandedTest {
    @Test
    fun isIdentityDistinguishesIdentityFromOthers() {
        assertTrue(Matrix33.IDENTITY.isIdentity)
        assertFalse(Matrix33.makeTranslate(1f, 0f).isIdentity)
    }

    @Test
    fun preConcatAndPostConcatComposeInExpectedOrder() {
        val translate = Matrix33.makeTranslate(10f, 0f)
        val scale = Matrix33.makeScale(2f, 2f)

        val pre = translate.preConcat(scale)
        val post = translate.postConcat(scale)

        assertEquals(pre.mapPoint(Point(1f, 1f)), translate.concat(scale).mapPoint(Point(1f, 1f)))
        assertEquals(post.mapPoint(Point(1f, 1f)), scale.concat(translate).mapPoint(Point(1f, 1f)))
    }

    @Test
    fun makeSkewSkewsAPoint() {
        val m = Matrix33.makeSkew(0.5f, 0f)
        val mapped = m.mapPoint(Point(0f, 10f))
        assertEquals(5f, mapped.x)
        assertEquals(10f, mapped.y)
    }

    @Test
    fun valuesExposesRawRowMajorComponents() {
        val values = Matrix33.makeTranslate(3f, 4f).values()
        assertEquals(9, values.size)
        // Row-major: [scaleX, skewX, translateX, skewY, scaleY, translateY, persp0, persp1, persp2]
        assertEquals(1f, values[0])
        assertEquals(3f, values[2])
        assertEquals(1f, values[4])
        assertEquals(4f, values[5])
        assertEquals(1f, values[8])
    }

    @Test
    fun makeFromValuesRoundTripsThroughValues() {
        val original = Matrix33.makeScale(2f, 3f).postConcat(Matrix33.makeTranslate(5f, 6f))
        val rebuilt = Matrix33.makeFromValues(original.values())
        assertEquals(original, rebuilt)
        assertEquals(original.mapPoint(Point(1f, 1f)), rebuilt.mapPoint(Point(1f, 1f)))
    }
}
