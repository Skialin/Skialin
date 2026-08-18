package org.skialin

import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue

class ColorSpaceTest {
    @Test
    fun srgbIsSrgb() {
        ColorSpace.makeSRGB().use { cs ->
            assertTrue(cs.isSrgb)
            assertTrue(cs.gammaCloseToSrgb)
            assertFalse(cs.gammaIsLinear)
        }
    }

    @Test
    fun srgbLinearHasLinearGamma() {
        ColorSpace.makeSRGBLinear().use { cs ->
            assertTrue(cs.gammaIsLinear)
            assertFalse(cs.isSrgb)
        }
    }

    @Test
    fun cicpSrgbMatchesMakeSrgb() {
        ColorSpace.makeCICP(CicpPrimaries.REC709, CicpTransferFn.SRGB)!!.use { a ->
            ColorSpace.makeSRGB().use { b ->
                assertTrue(a.contentEquals(b))
            }
        }
    }

    @Test
    fun makeLinearGammaProducesLinear() {
        ColorSpace.makeSRGB().use { srgb ->
            srgb.makeLinearGamma().use { linear ->
                assertTrue(linear.gammaIsLinear)
            }
        }
    }

    @Test
    fun serializeThenDeserializeRoundtrips() {
        ColorSpace.makeSRGB().use { cs ->
            cs.serialize().use { data ->
                ColorSpace.deserialize(data.bytes())!!.use { restored ->
                    assertTrue(cs.contentEquals(restored))
                }
            }
        }
    }

    @Test
    fun fromIccProfileRejectsGarbage() {
        assertNull(ColorSpace.makeFromIccProfile("not an icc profile".toByteArray()))
    }

    @Test
    fun rgbRoundtripsGamut() {
        ColorSpace.makeRGB(NamedTransferFn.SRGB, NamedGamut.SRGB).use { cs ->
            val xyz = cs.toXyzD50()!!
            for (i in xyz.indices) {
                assertTrue(kotlin.math.abs(xyz[i] - NamedGamut.SRGB[i]) < 1e-4f)
            }
        }
    }
}
