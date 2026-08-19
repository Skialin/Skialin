package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

/** An immutable run of positioned glyphs, ready to draw. Mirrors Skia's `SkTextBlob`. */
class TextBlob internal constructor(
    ptr: Long,
) : Managed(ptr, TextBlobNative::nRelease) {
    enum class Encoding { UTF8, UTF16, UTF32, GLYPH_ID }

    val uniqueId: Int get() = TextBlobNative.nUniqueId(nativePtr)

    val bounds: Rect
        get() {
            val out = FloatArray(4)
            TextBlobNative.nBounds(nativePtr, out)
            return Rect(out[0], out[1], out[2], out[3])
        }

    companion object {
        /** A single run of [text], positioned using [font]'s default advances. `null` if [text] is empty. */
        fun makeFromText(
            text: String,
            font: Font,
            encoding: Encoding = Encoding.UTF8,
        ): TextBlob? {
            val ptr = TextBlobNative.nFromText(text, font.nativePtr, encoding.ordinal)
            return if (ptr == 0L) null else TextBlob(ptr)
        }

        /**
         * A single run of [text], positioned at `xpos[i]` on the shared baseline [constY].
         * `xpos.size` must equal the glyph/character count implied by [text] and [encoding].
         * `null` if [text] is empty.
         */
        fun makeFromPosTextH(
            text: String,
            xpos: FloatArray,
            constY: Float,
            font: Font,
            encoding: Encoding = Encoding.UTF8,
        ): TextBlob? {
            val ptr = TextBlobNative.nFromPosTextH(text, xpos, constY, font.nativePtr, encoding.ordinal)
            return if (ptr == 0L) null else TextBlob(ptr)
        }

        /**
         * A single run of [text], positioned at `pos[i]`. `pos.size` must equal the
         * glyph/character count implied by [text] and [encoding]. `null` if [text] is empty.
         */
        fun makeFromPosText(
            text: String,
            pos: Array<Point>,
            font: Font,
            encoding: Encoding = Encoding.UTF8,
        ): TextBlob? {
            val flat = FloatArray(pos.size * 2)
            pos.forEachIndexed { i, p ->
                flat[i * 2] = p.x
                flat[i * 2 + 1] = p.y
            }
            val ptr = TextBlobNative.nFromPosText(text, flat, font.nativePtr, encoding.ordinal)
            return if (ptr == 0L) null else TextBlob(ptr)
        }
    }
}

private object TextBlobNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nFromText(
        text: String,
        fontPtr: Long,
        encoding: Int,
    ): Long

    external fun nFromPosTextH(
        text: String,
        xpos: FloatArray,
        constY: Float,
        fontPtr: Long,
        encoding: Int,
    ): Long

    external fun nFromPosText(
        text: String,
        pos: FloatArray,
        fontPtr: Long,
        encoding: Int,
    ): Long

    external fun nRelease(ptr: Long)

    external fun nUniqueId(ptr: Long): Int

    external fun nBounds(
        ptr: Long,
        out: FloatArray,
    )
}
