package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

class TextBlobBuilder : Managed(TextBlobBuilderNative.nNew(), TextBlobBuilderNative::nRelease) {
    fun build(): TextBlob? {
        val ptr = TextBlobBuilderNative.nBuild(nativePtr)
        return if (ptr == 0L) null else TextBlob(ptr)
    }

    fun appendRun(
        font: Font,
        glyphs: ShortArray,
        x: Float,
        y: Float,
    ): TextBlobBuilder {
        TextBlobBuilderNative.nAppendRun(nativePtr, font.nativePtr, glyphs, x, y)
        return this
    }

    fun appendRunPosH(
        font: Font,
        glyphs: ShortArray,
        xpos: FloatArray,
        y: Float,
    ): TextBlobBuilder {
        TextBlobBuilderNative.nAppendRunPosH(nativePtr, font.nativePtr, glyphs, xpos, y)
        return this
    }

    fun appendRunPos(
        font: Font,
        glyphs: ShortArray,
        pos: Array<Point>,
    ): TextBlobBuilder {
        val flat = FloatArray(pos.size * 2)
        pos.forEachIndexed { i, p ->
            flat[i * 2] = p.x
            flat[i * 2 + 1] = p.y
        }
        TextBlobBuilderNative.nAppendRunPos(nativePtr, font.nativePtr, glyphs, flat)
        return this
    }

    fun appendRunRSXform(
        font: Font,
        glyphs: ShortArray,
        xforms: FloatArray,
    ): TextBlobBuilder {
        TextBlobBuilderNative.nAppendRunRSXform(nativePtr, font.nativePtr, glyphs, xforms)
        return this
    }
}

private object TextBlobBuilderNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nNew(): Long

    external fun nRelease(ptr: Long)

    external fun nBuild(ptr: Long): Long

    external fun nAppendRun(
        ptr: Long,
        fontPtr: Long,
        glyphs: ShortArray,
        x: Float,
        y: Float,
    )

    external fun nAppendRunPosH(
        ptr: Long,
        fontPtr: Long,
        glyphs: ShortArray,
        xpos: FloatArray,
        y: Float,
    )

    external fun nAppendRunPos(
        ptr: Long,
        fontPtr: Long,
        glyphs: ShortArray,
        pos: FloatArray,
    )

    external fun nAppendRunRSXform(
        ptr: Long,
        fontPtr: Long,
        glyphs: ShortArray,
        xforms: FloatArray,
    )
}
