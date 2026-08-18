package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader
import java.nio.ByteBuffer

class Pixmap private constructor(ptr: Long, private val buffer: ByteBuffer) : Managed(ptr, PixmapNative::nRelease) {
    val addr: Long get() = PixmapNative.nAddr(nativePtr)
    val rowBytes: Long get() = PixmapNative.nRowBytes(nativePtr)
    val width: Int get() = PixmapNative.nWidth(nativePtr)
    val height: Int get() = PixmapNative.nHeight(nativePtr)
    val isEmpty: Boolean get() = PixmapNative.nIsEmpty(nativePtr)
    val colorType: ColorType get() = ColorType.entries[PixmapNative.nColorType(nativePtr)]
    val alphaType: AlphaType get() = AlphaType.entries[PixmapNative.nAlphaType(nativePtr)]
    val isOpaque: Boolean get() = PixmapNative.nIsOpaque(nativePtr)
    val rowBytesAsPixels: Int get() = PixmapNative.nRowBytesAsPixels(nativePtr)
    val shiftPerPixel: Int get() = PixmapNative.nShiftPerPixel(nativePtr)

    fun computeByteSize(): Long = PixmapNative.nComputeByteSize(nativePtr)

    /** Unpremultiplied color at `(x, y)`. Ignores color space; not bounds-checked. */
    fun getColor(x: Int, y: Int): Color = PixmapNative.nGetColor(nativePtr, x, y)

    fun getAlphaf(x: Int, y: Int): Float = PixmapNative.nGetAlphaf(nativePtr, x, y)

    /** The intersection with `area`, sharing this pixmap's backing storage, or null if empty. */
    fun extractSubset(area: IRect): Pixmap? {
        val ptr = PixmapNative.nExtractSubset(nativePtr, area.left, area.top, area.right, area.bottom)
        return if (ptr == 0L) null else Pixmap(ptr, buffer)
    }

    companion object {
        /** `buffer` must be direct (see [ByteBuffer.allocateDirect]) so its address is stable. */
        fun make(info: ImageInfo, buffer: ByteBuffer, rowBytes: Long): Pixmap {
            require(buffer.isDirect) { "Pixmap requires a direct ByteBuffer" }
            val addr = PixmapNative.nBufferAddress(buffer)
            return Pixmap(PixmapNative.nMake(info.nativePtr, addr, rowBytes), buffer)
        }
    }
}

private object PixmapNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nBufferAddress(buffer: ByteBuffer): Long
    external fun nMake(infoPtr: Long, addr: Long, rowBytes: Long): Long
    external fun nRelease(ptr: Long)
    external fun nAddr(ptr: Long): Long
    external fun nRowBytes(ptr: Long): Long
    external fun nWidth(ptr: Long): Int
    external fun nHeight(ptr: Long): Int
    external fun nIsEmpty(ptr: Long): Boolean
    external fun nColorType(ptr: Long): Int
    external fun nAlphaType(ptr: Long): Int
    external fun nIsOpaque(ptr: Long): Boolean
    external fun nRowBytesAsPixels(ptr: Long): Int
    external fun nShiftPerPixel(ptr: Long): Int
    external fun nComputeByteSize(ptr: Long): Long
    external fun nGetColor(ptr: Long, x: Int, y: Int): Int
    external fun nGetAlphaf(ptr: Long, x: Int, y: Int): Float
    external fun nExtractSubset(ptr: Long, left: Int, top: Int, right: Int, bottom: Int): Long
}
