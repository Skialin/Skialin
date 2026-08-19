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
}
