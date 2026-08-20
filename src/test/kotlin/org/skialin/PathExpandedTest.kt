package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class PathExpandedTest {
    @Test
    fun relativeAndConicBuildersDoNotCrash() {
        PathBuilder().use { builder ->
            builder
                .moveTo(0f, 0f)
                .rLineTo(10f, 0f)
                .rQuadTo(2f, 2f, 4f, 0f)
                .conicTo(6f, 2f, 8f, 0f, 0.7f)
                .rConicTo(2f, 2f, 4f, 0f, 0.7f)
                .rCubicTo(1f, 1f, 2f, 2f, 3f, 0f)
                .closePath()
            builder.snapshot().use { path -> assertFalse(path.isEmpty) }
        }
    }

    @Test
    fun addRRectAndAddPolyAndAddPath() {
        PathBuilder().use { rectBuilder ->
            rectBuilder.addRect(Rect(0f, 0f, 4f, 4f))
            rectBuilder.snapshot().use { rectPath ->
                PathBuilder().use { builder ->
                    builder.addRRect(RRect.makeRectXY(Rect(0f, 0f, 10f, 10f), 2f, 2f))
                    builder.addPoly(arrayOf(Point(0f, 0f), Point(5f, 0f), Point(5f, 5f)), true)
                    builder.addPath(rectPath, dx = 20f, dy = 20f)
                    builder.addPath(rectPath, Matrix33.makeTranslate(40f, 0f))
                    builder.snapshot().use { path -> assertFalse(path.isEmpty) }
                }
            }
        }
    }

    @Test
    fun transformSetLastPtAndReset() {
        PathBuilder().use { builder ->
            builder.moveTo(0f, 0f).lineTo(10f, 0f).setLastPt(20f, 0f)
            builder.transform(Matrix33.makeTranslate(5f, 5f))
            assertFalse(builder.isEmpty)
            builder.reset()
            assertTrue(builder.isEmpty)
        }
    }

    @Test
    fun pathIntrospectionMatchesKnownRect() {
        PathBuilder().use { builder ->
            builder.addRect(Rect(0f, 0f, 10f, 20f))
            builder.snapshot().use { path ->
                assertEquals(PathFillType.WINDING, path.fillType)
                assertTrue(path.isConvex)
                assertEquals(4, path.pointsCount)
                assertEquals(4, path.points.size)
                assertTrue(path.generationId != 0)

                val tight = path.computeTightBounds()
                assertEquals(0f, tight.left)
                assertEquals(0f, tight.top)
                assertEquals(10f, tight.right)
                assertEquals(20f, tight.bottom)
            }
        }
    }

    @Test
    fun isOvalAndIsRRectRecognizeShapes() {
        PathBuilder().use { ovalBuilder ->
            ovalBuilder.addOval(Rect(0f, 0f, 10f, 10f))
            ovalBuilder.snapshot().use { ovalPath -> assertNotNull(ovalPath.isOval) }
        }
        PathBuilder().use { rrectBuilder ->
            rrectBuilder.addRRect(RRect.makeRectXY(Rect(0f, 0f, 10f, 10f), 2f, 2f))
            rrectBuilder.snapshot().use { rrectPath ->
                rrectPath.isRRect.use { r -> assertNotNull(r) }
            }
        }
    }

    @Test
    fun builderCanBeSeededFromExistingPath() {
        PathBuilder().use { original ->
            original.moveTo(0f, 0f).lineTo(10f, 0f).lineTo(10f, 10f)
            original.snapshot().use { snapshot ->
                val originalPointsCount = snapshot.pointsCount
                PathBuilder(snapshot).use { seeded ->
                    assertFalse(seeded.isEmpty)
                    seeded.lineTo(0f, 10f)
                    seeded.snapshot().use { extended ->
                        assertTrue(extended.pointsCount > originalPointsCount)
                        assertEquals(Point(0f, 0f), extended.points[0])
                        assertEquals(Point(0f, 10f), extended.points[extended.pointsCount - 1])
                    }
                }
            }
        }
    }

    @Test
    fun builderFillTypeIsSettableAndReadable() {
        PathBuilder().use { builder ->
            assertEquals(PathFillType.WINDING, builder.fillType)
            builder.fillType = PathFillType.EVEN_ODD
            assertEquals(PathFillType.EVEN_ODD, builder.fillType)
            builder.addRect(Rect(0f, 0f, 10f, 10f))
            builder.snapshot().use { path -> assertEquals(PathFillType.EVEN_ODD, path.fillType) }
        }
    }

    @Test
    fun arcToAppendsArcSegment() {
        PathBuilder().use { builder ->
            builder.moveTo(0f, 0f)
            builder.arcTo(Rect(0f, 0f, 20f, 20f), 0f, 90f, false)
            builder.snapshot().use { path -> assertFalse(path.isEmpty) }
        }
    }

    @Test
    fun pathSegmentsWalksVerbsInOrder() {
        PathBuilder().use { builder ->
            builder.moveTo(0f, 0f).lineTo(10f, 0f).quadTo(15f, 5f, 10f, 10f).cubicTo(8f, 12f, 4f, 12f, 0f, 10f).closePath()
            builder.snapshot().use { path ->
                val segments = path.segments()
                val verbs = segments.map { it.verb }
                assertEquals(listOf(PathVerb.MOVE, PathVerb.LINE, PathVerb.QUAD, PathVerb.CUBIC, PathVerb.LINE, PathVerb.CLOSE), verbs)
                assertEquals(Point(0f, 0f), segments[0].points[0])
                assertEquals(Point(10f, 0f), segments[1].points[1])
            }
        }
    }

    @Test
    fun pathSegmentsCanConvertConicsToQuads() {
        PathBuilder().use { builder ->
            builder.moveTo(0f, 0f).conicTo(10f, 0f, 10f, 10f, 0.7f)
            builder.snapshot().use { path ->
                val rawSegments = path.segments(convertConicsToQuads = false)
                assertTrue(rawSegments.any { it.verb == PathVerb.CONIC })

                val convertedSegments = path.segments(convertConicsToQuads = true)
                assertTrue(convertedSegments.none { it.verb == PathVerb.CONIC })
                assertTrue(convertedSegments.any { it.verb == PathVerb.QUAD })
            }
        }
    }
}
