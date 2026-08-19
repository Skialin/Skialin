package org.skialin

import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

class ParagraphTest {
    private fun fontCollection(): FontCollection {
        val collection = FontCollection()
        collection.setDefaultFontManager(FontMgr.system())
        return collection
    }

    @Test
    fun layoutProducesNonzeroHeight() {
        fontCollection().use { collection ->
            ParagraphStyle().use { style ->
                ParagraphBuilder(style, collection).use { builder ->
                    TextStyle().use { textStyle ->
                        textStyle.fontSize = 18f
                        textStyle.color = Colors.BLACK
                        builder.pushStyle(textStyle)
                        builder.addText("Hello, world!")
                        builder.pop()

                        builder.build().use { paragraph ->
                            paragraph.layout(200f)
                            assertTrue(paragraph.height > 0f)
                            assertTrue(paragraph.maxWidth <= 200f)
                            assertTrue(paragraph.lineNumber == 1L)
                            assertFalse(paragraph.didExceedMaxLines)
                        }
                    }
                }
            }
        }
    }

    @Test
    fun narrowWidthWrapsToMultipleLines() {
        fontCollection().use { collection ->
            ParagraphStyle().use { style ->
                ParagraphBuilder(style, collection).use { builder ->
                    TextStyle().use { textStyle ->
                        textStyle.fontSize = 18f
                        builder.pushStyle(textStyle)
                        builder.addText("Hello, world! This is a longer sentence that should wrap.")
                        builder.pop()

                        builder.build().use { paragraph ->
                            paragraph.layout(80f)
                            assertTrue(paragraph.lineNumber > 1)
                        }
                    }
                }
            }
        }
    }

    @Test
    fun lineMetricsCoverTheText() {
        fontCollection().use { collection ->
            ParagraphStyle().use { style ->
                ParagraphBuilder(style, collection).use { builder ->
                    TextStyle().use { textStyle ->
                        textStyle.fontSize = 18f
                        builder.pushStyle(textStyle)
                        val text = "Hello"
                        builder.addText(text)
                        builder.pop()

                        builder.build().use { paragraph ->
                            paragraph.layout(200f)
                            val metrics = paragraph.lineMetrics()
                            assertTrue(metrics.size == 1)
                            assertTrue(metrics[0].startIndex == 0L)
                            assertTrue(metrics[0].endIndex == text.length.toLong())
                            assertTrue(metrics[0].width > 0.0)
                        }
                    }
                }
            }
        }
    }

    @Test
    fun rtlDirectionLaysOutWithoutCrashing() {
        fontCollection().use { collection ->
            ParagraphStyle().use { style ->
                style.textDirection = ParagraphStyle.TextDirection.RTL
                ParagraphBuilder(style, collection).use { builder ->
                    TextStyle().use { textStyle ->
                        textStyle.fontSize = 18f
                        builder.pushStyle(textStyle)
                        builder.addText("مرحبا بالعالم")
                        builder.pop()

                        builder.build().use { paragraph ->
                            paragraph.layout(200f)
                            assertTrue(paragraph.height > 0f)
                        }
                    }
                }
            }
        }
    }

    @Test
    fun paintsWithoutCrashing() {
        fontCollection().use { collection ->
            ParagraphStyle().use { style ->
                ParagraphBuilder(style, collection).use { builder ->
                    TextStyle().use { textStyle ->
                        textStyle.fontSize = 18f
                        textStyle.color = Colors.BLACK
                        builder.pushStyle(textStyle)
                        builder.addText("Hello, world!")
                        builder.pop()

                        builder.build().use { paragraph ->
                            paragraph.layout(200f)
                            Surface.makeRasterN32Premul(200, 100)!!.use { surface ->
                                paragraph.paint(surface.canvas(), 0f, 0f)
                            }
                        }
                    }
                }
            }
        }
    }

    @Test
    fun getRectsForRangeReturnsBoxesForText() {
        fontCollection().use { collection ->
            ParagraphStyle().use { style ->
                ParagraphBuilder(style, collection).use { builder ->
                    TextStyle().use { textStyle ->
                        textStyle.fontSize = 18f
                        textStyle.color = Colors.BLACK
                        builder.pushStyle(textStyle)
                        builder.addText("Hello, world!")
                        builder.pop()

                        builder.build().use { paragraph ->
                            paragraph.layout(200f)
                            val boxes = paragraph.getRectsForRange(0, 5)
                            assertTrue(boxes.isNotEmpty())
                            boxes.forEach { assertTrue(it.rect.right > it.rect.left) }
                        }
                    }
                }
            }
        }
    }

    @Test
    fun getRectsForPlaceholdersReturnsOneBoxPerPlaceholder() {
        fontCollection().use { collection ->
            ParagraphStyle().use { style ->
                ParagraphBuilder(style, collection).use { builder ->
                    TextStyle().use { textStyle ->
                        textStyle.fontSize = 18f
                        builder.pushStyle(textStyle)
                        builder.addText("Before ")
                        builder.addPlaceholder(PlaceholderStyle(20f, 20f, PlaceholderStyle.Alignment.MIDDLE))
                        builder.addText(" after")
                        builder.pop()

                        builder.build().use { paragraph ->
                            paragraph.layout(200f)
                            val boxes = paragraph.getRectsForPlaceholders()
                            assertTrue(boxes.size == 1)
                        }
                    }
                }
            }
        }
    }

    @Test
    fun glyphInfoQueriesFindTheGlyph() {
        fontCollection().use { collection ->
            ParagraphStyle().use { style ->
                ParagraphBuilder(style, collection).use { builder ->
                    TextStyle().use { textStyle ->
                        textStyle.fontSize = 18f
                        builder.pushStyle(textStyle)
                        builder.addText("Hello")
                        builder.pop()

                        builder.build().use { paragraph ->
                            paragraph.layout(200f)
                            val info = paragraph.glyphInfoAt(0L)
                            assertNotNull(info)
                            assertTrue(info.bounds.right > info.bounds.left)

                            val closest = paragraph.closestGlyphInfoAt(0f, 0f)
                            assertNotNull(closest)

                            assertNull(paragraph.glyphInfoAt(1000L))
                        }
                    }
                }
            }
        }
    }

    @Test
    fun updateFontSizeAndPaintsDoNotCrash() {
        fontCollection().use { collection ->
            ParagraphStyle().use { style ->
                ParagraphBuilder(style, collection).use { builder ->
                    TextStyle().use { textStyle ->
                        textStyle.fontSize = 18f
                        textStyle.color = Colors.BLACK
                        builder.pushStyle(textStyle)
                        builder.addText("Hello, world!")
                        builder.pop()

                        builder.build().use { paragraph ->
                            paragraph.layout(200f)
                            paragraph.updateFontSize(0L, 5L, 24f)
                            Paint().use { fg ->
                                fg.color = Colors.RED
                                paragraph.updateForegroundPaint(0L, 5L, fg)
                            }
                            Paint().use { bg ->
                                bg.color = Colors.BLUE
                                paragraph.updateBackgroundPaint(0L, 5L, bg)
                            }
                            Surface.makeRasterN32Premul(200, 100)!!.use { surface ->
                                paragraph.paint(surface.canvas(), 0f, 0f)
                            }
                        }
                    }
                }
            }
        }
    }
}
