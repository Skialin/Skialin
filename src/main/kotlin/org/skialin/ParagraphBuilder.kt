package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Accumulates styled text, then builds a [Paragraph]. Mirrors skparagraph's `ParagraphBuilder`. */
class ParagraphBuilder(style: ParagraphStyle, fontCollection: FontCollection) :
    Managed(ParagraphBuilderNative.nNew(style.nativePtr, fontCollection.nativePtr), ParagraphBuilderNative::nRelease) {

    /**
     * [style] is copied, not consumed: it stays independently valid and closeable
     * afterward. Text added after this call, until the matching [pop], uses [style].
     */
    fun pushStyle(style: TextStyle): ParagraphBuilder {
        ParagraphBuilderNative.nPushStyle(nativePtr, style.nativePtr)
        return this
    }

    fun pop(): ParagraphBuilder {
        ParagraphBuilderNative.nPop(nativePtr)
        return this
    }

    /** Adds text, styled with whatever is on top of the style stack. */
    fun addText(text: String): ParagraphBuilder {
        ParagraphBuilderNative.nAddText(nativePtr, text)
        return this
    }

    /** Builds a [Paragraph] from the accumulated text and styles. This builder remains usable afterward. */
    fun build(): Paragraph = Paragraph(ParagraphBuilderNative.nBuild(nativePtr))
}

private object ParagraphBuilderNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nNew(stylePtr: Long, fontCollectionPtr: Long): Long
    external fun nRelease(ptr: Long)
    external fun nPushStyle(ptr: Long, stylePtr: Long)
    external fun nPop(ptr: Long)
    external fun nAddText(ptr: Long, text: String)
    external fun nBuild(ptr: Long): Long
}
