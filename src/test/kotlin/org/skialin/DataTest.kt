package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue

class DataTest {
    @Test
    fun makeFromBytesRoundTrips() {
        Data.makeFromBytes("hello world".toByteArray()).use { data ->
            assertEquals(11L, data.size)
            assertFalse(data.isEmpty)
            assertEquals("hello world", String(data.bytes()))
        }
    }

    @Test
    fun emptyIsEmpty() {
        Data.makeEmpty().use { data ->
            assertEquals(0L, data.size)
            assertTrue(data.isEmpty)
        }
    }

    @Test
    fun zeroInitializedIsAllZero() {
        Data.makeZeroInitialized(4).use { data ->
            assertEquals(listOf<Byte>(0, 0, 0, 0), data.bytes().toList())
        }
    }

    @Test
    fun byteBufferSharesBackingStorage() {
        Data.makeFromBytes("abcdef".toByteArray()).use { data ->
            val buffer = data.byteBuffer()
            val out = ByteArray(6)
            buffer.get(out)
            assertEquals("abcdef", String(out))
        }
    }

    @Test
    fun copyRangeExtractsMiddle() {
        Data.makeFromBytes("0123456789".toByteArray()).use { data ->
            assertEquals("234", String(data.copyRange(2, 3)))
        }
    }

    @Test
    fun copyAndShareSubset() {
        Data.makeFromBytes("0123456789".toByteArray()).use { data ->
            data.copySubset(3, 4)!!.use { subset -> assertEquals("3456", String(subset.bytes())) }
            data.shareSubset(3, 4)!!.use { subset -> assertEquals("3456", String(subset.bytes())) }
            assertNull(data.copySubset(8, 10))
        }
    }

    @Test
    fun contentEqualsComparesBytes() {
        Data.makeFromBytes("same".toByteArray()).use { a ->
            Data.makeFromBytes("same".toByteArray()).use { b ->
                Data.makeFromBytes("different".toByteArray()).use { c ->
                    assertTrue(a.contentEquals(b))
                    assertFalse(a.contentEquals(c))
                }
            }
        }
    }

    @Test
    fun missingFileReturnsNull() {
        assertNull(Data.makeFromFileName("Z:/definitely/does/not/exist.bin"))
    }
}
