package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader

enum class RegionOp { DIFFERENCE, INTERSECT, UNION, XOR, REVERSE_DIFFERENCE, REPLACE }

class Region internal constructor(
    ptr: Long,
) : Managed(ptr, RegionNative::nRelease) {
    constructor() : this(RegionNative.nMake())

    fun cloneRegion(): Region = Region(RegionNative.nClone(nativePtr))

    fun setRect(rect: IRect): Boolean = RegionNative.nSetRect(nativePtr, rect.toIntArray())

    fun setPath(
        path: Path,
        clip: Region,
    ): Boolean = RegionNative.nSetPath(nativePtr, path.nativePtr, clip.nativePtr)

    fun op(
        other: Region,
        op: RegionOp,
    ): Boolean = RegionNative.nOp(nativePtr, other.nativePtr, op.ordinal)

    fun op(
        rect: IRect,
        op: RegionOp,
    ): Boolean = RegionNative.nOpRect(nativePtr, rect.toIntArray(), op.ordinal)

    val isEmpty: Boolean get() = RegionNative.nIsEmpty(nativePtr)
    val isRect: Boolean get() = RegionNative.nIsRect(nativePtr)
    val isComplex: Boolean get() = RegionNative.nIsComplex(nativePtr)

    val bounds: IRect
        get() = RegionNative.nGetBounds(nativePtr).let { IRect(it[0], it[1], it[2], it[3]) }

    fun contains(
        x: Int,
        y: Int,
    ): Boolean = RegionNative.nContainsPoint(nativePtr, x, y)

    fun contains(rect: IRect): Boolean = RegionNative.nContainsRect(nativePtr, rect.toIntArray())

    fun contains(other: Region): Boolean = RegionNative.nContainsRegion(nativePtr, other.nativePtr)

    fun intersects(rect: IRect): Boolean = RegionNative.nIntersectsRect(nativePtr, rect.toIntArray())

    fun intersects(other: Region): Boolean = RegionNative.nIntersectsRegion(nativePtr, other.nativePtr)

    fun contentEquals(other: Region): Boolean = RegionNative.nEquals(nativePtr, other.nativePtr)

    fun boundaryPath(): Path = Path(RegionNative.nGetBoundaryPath(nativePtr))

    companion object {
        fun makeRect(rect: IRect): Region = Region(RegionNative.nMakeRect(rect.toIntArray()))
    }
}

private fun IRect.toIntArray() = intArrayOf(left, top, right, bottom)

private object RegionNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMake(): Long

    external fun nMakeRect(rect: IntArray): Long

    external fun nRelease(ptr: Long)

    external fun nClone(ptr: Long): Long

    external fun nSetRect(
        ptr: Long,
        rect: IntArray,
    ): Boolean

    external fun nSetPath(
        ptr: Long,
        pathPtr: Long,
        clipPtr: Long,
    ): Boolean

    external fun nOp(
        ptr: Long,
        otherPtr: Long,
        op: Int,
    ): Boolean

    external fun nOpRect(
        ptr: Long,
        rect: IntArray,
        op: Int,
    ): Boolean

    external fun nIsEmpty(ptr: Long): Boolean

    external fun nIsRect(ptr: Long): Boolean

    external fun nIsComplex(ptr: Long): Boolean

    external fun nGetBounds(ptr: Long): IntArray

    external fun nContainsPoint(
        ptr: Long,
        x: Int,
        y: Int,
    ): Boolean

    external fun nContainsRect(
        ptr: Long,
        rect: IntArray,
    ): Boolean

    external fun nContainsRegion(
        ptr: Long,
        otherPtr: Long,
    ): Boolean

    external fun nIntersectsRect(
        ptr: Long,
        rect: IntArray,
    ): Boolean

    external fun nIntersectsRegion(
        ptr: Long,
        otherPtr: Long,
    ): Boolean

    external fun nEquals(
        ptr: Long,
        otherPtr: Long,
    ): Boolean

    external fun nGetBoundaryPath(ptr: Long): Long
}
