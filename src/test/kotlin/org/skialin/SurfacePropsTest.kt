package org.skialin

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals

class SurfacePropsTest {
    @Test
    fun accessorsMatchConstructor() {
        val flags = SurfacePropsFlags.ALWAYS_DITHER or SurfacePropsFlags.USE_DEVICE_INDEPENDENT_FONTS
        SurfaceProps(flags, PixelGeometry.RGB_H, 0.5f, 1.5f).use { props ->
            assertEquals(flags, props.flags)
            assertEquals(PixelGeometry.RGB_H, props.pixelGeometry)
            assertEquals(0.5f, props.textContrast)
            assertEquals(1.5f, props.textGamma)
        }
    }

    @Test
    fun cloneWithPixelGeometryChangesOnlyGeometry() {
        SurfaceProps(SurfacePropsFlags.ALWAYS_DITHER, PixelGeometry.RGB_V, 0.2f, 1.8f).use { props ->
            props.cloneWithPixelGeometry(PixelGeometry.BGR_H).use { cloned ->
                assertEquals(PixelGeometry.BGR_H, cloned.pixelGeometry)
                assertEquals(props.flags, cloned.flags)
            }
        }
    }

    @Test
    fun equalsComparesByValue() {
        SurfaceProps(SurfacePropsFlags.DEFAULT, PixelGeometry.RGB_H, 0f, 2.2f).use { a ->
            SurfaceProps(SurfacePropsFlags.DEFAULT, PixelGeometry.RGB_H, 0f, 2.2f).use { b ->
                SurfaceProps(SurfacePropsFlags.DEFAULT, PixelGeometry.BGR_H, 0f, 2.2f).use { c ->
                    assertEquals(a, b)
                    assertNotEquals(a, c)
                }
            }
        }
    }
}
