package org.skialin

import kotlin.test.Test
import kotlin.test.assertTrue

class TypefaceFontProviderTest {
    @Test
    fun registerTypefaceAndUseInParagraphShaping() {
        val mgr = FontMgr.system()
        val name = mgr.familyName(0)
        mgr.matchFamilyStyle(name)!!.use { typeface ->
            TypefaceFontProvider().use { provider ->
                provider.registerTypeface(typeface, "MyCustomAlias")

                FontCollection().use { collection ->
                    collection.setAssetTypefaceProvider(provider)
                    collection.setDefaultFontManager(FontMgr.system())

                    ParagraphStyle().use { style ->
                        ParagraphBuilder(style, collection).use { builder ->
                            TextStyle().use { textStyle ->
                                textStyle.fontSize = 18f
                                textStyle.fontFamilies = listOf("MyCustomAlias")
                                builder.pushStyle(textStyle)
                                builder.addText("Hello, world!")
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
        }
    }

    @Test
    fun registerTypefaceReturnsOneOnSuccess() {
        val mgr = FontMgr.system()
        val name = mgr.familyName(0)
        mgr.matchFamilyStyle(name)!!.use { typeface ->
            TypefaceFontProvider().use { provider ->
                assertTrue(provider.registerTypeface(typeface, "AliasOne") == 1L)
                assertTrue(provider.registerTypeface(typeface, "AliasTwo") == 1L)
            }
        }
    }
}
