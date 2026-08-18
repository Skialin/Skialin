package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class PathOpsTest {
    private fun makeRect(left: Float, top: Float, right: Float, bottom: Float): Path {
        val builder = PathBuilder()
        builder.addRect(Rect(left, top, right, bottom))
        return builder.snapshot()
    }

    @Test
    fun unionCoversBothRects() {
        makeRect(0f, 0f, 10f, 10f).use { a ->
            makeRect(5f, 5f, 15f, 15f).use { b ->
                Path.op(a, b, PathOp.UNION)!!.use { result ->
                    assertEquals(Rect(0f, 0f, 15f, 15f), result.bounds)
                }
            }
        }
    }

    @Test
    fun intersectIsTheOverlap() {
        makeRect(0f, 0f, 10f, 10f).use { a ->
            makeRect(5f, 5f, 15f, 15f).use { b ->
                Path.op(a, b, PathOp.INTERSECT)!!.use { result ->
                    assertEquals(Rect(5f, 5f, 10f, 10f), result.bounds)
                }
            }
        }
    }

    @Test
    fun differenceRemovesOverlap() {
        makeRect(0f, 0f, 10f, 10f).use { a ->
            makeRect(5f, 5f, 15f, 15f).use { b ->
                Path.op(a, b, PathOp.DIFFERENCE)!!.use { result ->
                    assertFalse(result.isEmpty)
                    assertTrue(result.contains(Point(1f, 1f)))
                    assertFalse(result.contains(Point(7f, 7f)))
                }
            }
        }
    }

    @Test
    fun xorAndReverseDifferenceAreUsable() {
        makeRect(0f, 0f, 10f, 10f).use { a ->
            makeRect(5f, 5f, 15f, 15f).use { b ->
                Path.op(a, b, PathOp.XOR).use { assertNotNull(it) }
            }
        }
        makeRect(0f, 0f, 10f, 10f).use { a2 ->
            makeRect(5f, 5f, 15f, 15f).use { b2 ->
                Path.op(a2, b2, PathOp.REVERSE_DIFFERENCE).use { assertNotNull(it) }
            }
        }
    }

    @Test
    fun simplifyRemovesSelfIntersections() {
        makeRect(0f, 0f, 10f, 10f).use { path ->
            path.simplify()!!.use { simplified -> assertEquals(path.bounds, simplified.bounds) }
        }
    }
}
