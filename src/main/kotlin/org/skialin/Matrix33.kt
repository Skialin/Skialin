package org.skialin

import org.skialin.impl.NativeLoader

class Matrix33 internal constructor(
    internal val values: FloatArray,
) {
    init {
        require(values.size == 9) { "Matrix33 requires exactly 9 values" }
    }

    fun concat(other: Matrix33): Matrix33 = Matrix33(MatrixNative.nConcat(values, other.values))

    fun invert(): Matrix33? = MatrixNative.nInvert(values)?.let { Matrix33(it) }

    fun mapPoint(point: Point): Point {
        val out = MatrixNative.nMapPoint(values, point.x, point.y)
        return Point(out[0], out[1])
    }

    fun mapRect(rect: Rect): Rect {
        val out = MatrixNative.nMapRect(values, rect.left, rect.top, rect.right, rect.bottom)
        return Rect(out[0], out[1], out[2], out[3])
    }

    fun preConcat(other: Matrix33): Matrix33 = Matrix33(MatrixNative.nPreConcat(values, other.values))

    fun postConcat(other: Matrix33): Matrix33 = Matrix33(MatrixNative.nPostConcat(values, other.values))

    val isIdentity: Boolean get() = MatrixNative.nIsIdentity(values)

    /**
     * The raw row-major 3x3 matrix components (`[scaleX, skewX, translateX, skewY, scaleY,
     * translateY, persp0, persp1, persp2]`), matching Skia's `SkMatrix::get9`/`set9` layout.
     */
    fun values(): FloatArray = values.copyOf()

    override fun equals(other: Any?): Boolean = other is Matrix33 && values.contentEquals(other.values)

    override fun hashCode(): Int = values.contentHashCode()

    companion object {
        val IDENTITY: Matrix33 get() = Matrix33(MatrixNative.nIdentity())

        fun makeTranslate(
            dx: Float,
            dy: Float,
        ): Matrix33 = Matrix33(MatrixNative.nTranslate(dx, dy))

        fun makeScale(
            sx: Float,
            sy: Float,
        ): Matrix33 = Matrix33(MatrixNative.nScale(sx, sy))

        fun makeRotate(degrees: Float): Matrix33 = Matrix33(MatrixNative.nRotate(degrees))

        /**
         * Builds a matrix from its raw row-major 3x3 components, matching Skia's
         * `SkMatrix::get9`/`set9` layout (see [Matrix33.values]).
         */
        fun makeFromValues(values: FloatArray): Matrix33 = Matrix33(values.copyOf())

        fun makeSkew(
            sx: Float,
            sy: Float,
        ): Matrix33 = Matrix33(MatrixNative.nSkew(sx, sy))
    }
}

private object MatrixNative {
    init {
        NativeLoader.ensureLoaded()
    }

    external fun nIdentity(): FloatArray

    external fun nTranslate(
        dx: Float,
        dy: Float,
    ): FloatArray

    external fun nScale(
        sx: Float,
        sy: Float,
    ): FloatArray

    external fun nRotate(degrees: Float): FloatArray

    external fun nConcat(
        a: FloatArray,
        b: FloatArray,
    ): FloatArray

    external fun nInvert(m: FloatArray): FloatArray?

    external fun nMapPoint(
        m: FloatArray,
        x: Float,
        y: Float,
    ): FloatArray

    external fun nMapRect(
        m: FloatArray,
        left: Float,
        top: Float,
        right: Float,
        bottom: Float,
    ): FloatArray

    external fun nSkew(
        sx: Float,
        sy: Float,
    ): FloatArray

    external fun nPreConcat(
        a: FloatArray,
        b: FloatArray,
    ): FloatArray

    external fun nPostConcat(
        a: FloatArray,
        b: FloatArray,
    ): FloatArray

    external fun nIsIdentity(m: FloatArray): Boolean
}
