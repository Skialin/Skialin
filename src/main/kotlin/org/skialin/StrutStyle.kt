package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** An optional synthetic line-height override independent of any actual glyph in the line. Mirrors skparagraph's `StrutStyle`. */
class StrutStyle internal constructor(
    ptr: Long,
) : Managed(ptr, StrutStyleNative::nRelease) {
    constructor() : this(StrutStyleNative.nNew())

    var fontFamilies: List<String>
        get() = StrutStyleNative.nFontFamilies(nativePtr).toList()
        set(value) = StrutStyleNative.nSetFontFamilies(nativePtr, value.toTypedArray())

    var fontStyle: FontStyle
        get() =
            FontStyle(
                StrutStyleNative.nWeight(nativePtr),
                StrutStyleNative.nWidth(nativePtr),
                FontStyle.Slant.entries[StrutStyleNative.nSlant(nativePtr)],
            )
        set(value) = StrutStyleNative.nSetFontStyle(nativePtr, value.weight, value.width, value.slant.ordinal)

    var fontSize: Float
        get() = StrutStyleNative.nFontSize(nativePtr)
        set(value) = StrutStyleNative.nSetFontSize(nativePtr, value)

    var height: Float
        get() = StrutStyleNative.nHeight(nativePtr)
        set(value) = StrutStyleNative.nSetHeight(nativePtr, value)

    var leading: Float
        get() = StrutStyleNative.nLeading(nativePtr)
        set(value) = StrutStyleNative.nSetLeading(nativePtr, value)

    var strutEnabled: Boolean
        get() = StrutStyleNative.nStrutEnabled(nativePtr)
        set(value) = StrutStyleNative.nSetStrutEnabled(nativePtr, value)

    var forceStrutHeight: Boolean
        get() = StrutStyleNative.nForceStrutHeight(nativePtr)
        set(value) = StrutStyleNative.nSetForceStrutHeight(nativePtr, value)

    var heightOverride: Boolean
        get() = StrutStyleNative.nHeightOverride(nativePtr)
        set(value) = StrutStyleNative.nSetHeightOverride(nativePtr, value)

    var halfLeading: Boolean
        get() = StrutStyleNative.nHalfLeading(nativePtr)
        set(value) = StrutStyleNative.nSetHalfLeading(nativePtr, value)
}

private object StrutStyleNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nNew(): Long

    external fun nRelease(ptr: Long)

    external fun nFontFamilies(ptr: Long): Array<String>

    external fun nSetFontFamilies(
        ptr: Long,
        families: Array<String>,
    )

    external fun nWeight(ptr: Long): Int

    external fun nWidth(ptr: Long): Int

    external fun nSlant(ptr: Long): Int

    external fun nSetFontStyle(
        ptr: Long,
        weight: Int,
        width: Int,
        slant: Int,
    )

    external fun nFontSize(ptr: Long): Float

    external fun nSetFontSize(
        ptr: Long,
        size: Float,
    )

    external fun nHeight(ptr: Long): Float

    external fun nSetHeight(
        ptr: Long,
        height: Float,
    )

    external fun nLeading(ptr: Long): Float

    external fun nSetLeading(
        ptr: Long,
        leading: Float,
    )

    external fun nStrutEnabled(ptr: Long): Boolean

    external fun nSetStrutEnabled(
        ptr: Long,
        enabled: Boolean,
    )

    external fun nForceStrutHeight(ptr: Long): Boolean

    external fun nSetForceStrutHeight(
        ptr: Long,
        force: Boolean,
    )

    external fun nHeightOverride(ptr: Long): Boolean

    external fun nSetHeightOverride(
        ptr: Long,
        heightOverride: Boolean,
    )

    external fun nHalfLeading(ptr: Long): Boolean

    external fun nSetHalfLeading(
        ptr: Long,
        halfLeading: Boolean,
    )
}
