package org.skialin

import org.skialin.impl.Managed
import org.skialin.impl.NativeLoader
import java.nio.ByteBuffer

class Data internal constructor(
    ptr: Long,
) : Managed(ptr, DataNative::nRelease) {
    val size: Long get() = DataNative.nSize(nativePtr)
    val isEmpty: Boolean get() = DataNative.nIsEmpty(nativePtr)

    fun bytes(): ByteArray = DataNative.nBytes(nativePtr)

    fun byteBuffer(): ByteBuffer = DataNative.nByteBuffer(nativePtr)

    fun copyRange(
        offset: Long,
        length: Long,
    ): ByteArray = DataNative.nCopyRange(nativePtr, offset, length)

    fun copySubset(
        offset: Long,
        length: Long,
    ): Data? = DataNative.nCopySubset(nativePtr, offset, length).takeIf { it != 0L }?.let { Data(it) }

    fun shareSubset(
        offset: Long,
        length: Long,
    ): Data? = DataNative.nShareSubset(nativePtr, offset, length).takeIf { it != 0L }?.let { Data(it) }

    fun contentEquals(other: Data): Boolean = DataNative.nEquals(nativePtr, other.nativePtr)

    companion object {
        fun makeEmpty(): Data = Data(DataNative.nMakeEmpty())

        fun makeFromBytes(bytes: ByteArray): Data = Data(DataNative.nMakeWithCopy(bytes))

        fun makeUninitialized(length: Long): Data = Data(DataNative.nMakeUninitialized(length))

        fun makeZeroInitialized(length: Long): Data = Data(DataNative.nMakeZeroInitialized(length))

        fun makeFromFileName(path: String): Data? = DataNative.nMakeFromFileName(path).takeIf { it != 0L }?.let { Data(it) }
    }
}

private object DataNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nMakeEmpty(): Long

    external fun nMakeWithCopy(bytes: ByteArray): Long

    external fun nMakeUninitialized(length: Long): Long

    external fun nMakeZeroInitialized(length: Long): Long

    external fun nMakeFromFileName(path: String): Long

    external fun nRelease(ptr: Long)

    external fun nSize(ptr: Long): Long

    external fun nIsEmpty(ptr: Long): Boolean

    external fun nBytes(ptr: Long): ByteArray

    external fun nByteBuffer(ptr: Long): ByteBuffer

    external fun nCopyRange(
        ptr: Long,
        offset: Long,
        length: Long,
    ): ByteArray

    external fun nCopySubset(
        ptr: Long,
        offset: Long,
        length: Long,
    ): Long

    external fun nShareSubset(
        ptr: Long,
        offset: Long,
        length: Long,
    ): Long

    external fun nEquals(
        ptr: Long,
        otherPtr: Long,
    ): Boolean
}
