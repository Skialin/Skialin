package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** Paragraph-wide layout settings: direction, alignment, line limits. Mirrors skparagraph's `ParagraphStyle`. */
class ParagraphStyle internal constructor(ptr: Long) : Managed(ptr, ParagraphStyleNative::nRelease) {
    enum class TextDirection { RTL, LTR }
    enum class TextAlign { LEFT, RIGHT, CENTER, JUSTIFY, START, END }

    /** A bitmask controlling ascent/descent adjustments at paragraph edges. */
    object TextHeightBehavior {
        const val ALL = 0x0
        const val DISABLE_FIRST_ASCENT = 0x1
        const val DISABLE_LAST_DESCENT = 0x2
        const val DISABLE_ALL = 0x3
    }

    constructor() : this(ParagraphStyleNative.nNew())

    /** The key knob for RTL/bidi layout: skparagraph resolves character-level bidi via ICU
     * internally once this is set to [TextDirection.RTL]. */
    var textDirection: TextDirection
        get() = TextDirection.entries[ParagraphStyleNative.nTextDirection(nativePtr)]
        set(value) = ParagraphStyleNative.nSetTextDirection(nativePtr, value.ordinal)

    var textAlign: TextAlign
        get() = TextAlign.entries[ParagraphStyleNative.nTextAlign(nativePtr)]
        set(value) = ParagraphStyleNative.nSetTextAlign(nativePtr, value.ordinal)

    var maxLines: Long
        get() = ParagraphStyleNative.nMaxLines(nativePtr)
        set(value) = ParagraphStyleNative.nSetMaxLines(nativePtr, value)

    var ellipsis: String
        get() = ParagraphStyleNative.nEllipsis(nativePtr)
        set(value) = ParagraphStyleNative.nSetEllipsis(nativePtr, value)

    var height: Float
        get() = ParagraphStyleNative.nHeight(nativePtr)
        set(value) = ParagraphStyleNative.nSetHeight(nativePtr, value)

    var textHeightBehavior: Int
        get() = ParagraphStyleNative.nTextHeightBehavior(nativePtr)
        set(value) = ParagraphStyleNative.nSetTextHeightBehavior(nativePtr, value)

    /** The default style new text runs start from. */
    var textStyle: TextStyle
        get() = TextStyle(ParagraphStyleNative.nTextStyle(nativePtr))
        set(value) = ParagraphStyleNative.nSetTextStyle(nativePtr, value.nativePtr)
}

private object ParagraphStyleNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nNew(): Long
    external fun nRelease(ptr: Long)
    external fun nTextDirection(ptr: Long): Int
    external fun nSetTextDirection(ptr: Long, direction: Int)
    external fun nTextAlign(ptr: Long): Int
    external fun nSetTextAlign(ptr: Long, align: Int)
    external fun nMaxLines(ptr: Long): Long
    external fun nSetMaxLines(ptr: Long, maxLines: Long)
    external fun nEllipsis(ptr: Long): String
    external fun nSetEllipsis(ptr: Long, ellipsis: String)
    external fun nHeight(ptr: Long): Float
    external fun nSetHeight(ptr: Long, height: Float)
    external fun nTextHeightBehavior(ptr: Long): Int
    external fun nSetTextHeightBehavior(ptr: Long, behavior: Int)
    external fun nTextStyle(ptr: Long): Long
    external fun nSetTextStyle(ptr: Long, stylePtr: Long)
}
