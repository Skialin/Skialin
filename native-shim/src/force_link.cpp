#include "include/core/SkBitmap.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkColorSpace.h"
#include "include/core/SkData.h"
#include "include/core/SkFont.h"
#include "include/core/SkImageInfo.h"
#include "include/core/SkImage.h"
#include "include/core/SkMatrix.h"
#include "include/core/SkPaint.h"
#include "include/core/SkPath.h"
#include "include/core/SkPathBuilder.h"
#include "include/core/SkPixmap.h"
#include "include/core/SkTextBlob.h"

namespace skialin_force_link {

#if defined(_MSC_VER)
#define SKIALIN_NOINLINE __declspec(noinline)
#else
#define SKIALIN_NOINLINE __attribute__((noinline))
#endif

SKIALIN_NOINLINE void touch(const SkBitmap& bitmap, SkBitmap& mutableBitmap) {
    mutableBitmap.eraseARGB(0, 0, 0, 0);
    (void)bitmap.height();
    (void)bitmap.width();
    (void)bitmap.rowBytes();
    (void)mutableBitmap.getPixels();
}

SKIALIN_NOINLINE void touch(SkCanvas& canvas, const SkPath& path, const SkPaint& paint) {
    canvas.clear(0);
    canvas.clipPath(path, SkClipOp::kIntersect);
    canvas.clipRect(SkRect::MakeEmpty(), SkClipOp::kIntersect);
    canvas.drawCircle({0, 0}, 0, paint);
    canvas.drawColor(0, SkBlendMode::kSrcOver);
    canvas.drawLine({0, 0}, {0, 0}, paint);
}

SKIALIN_NOINLINE void touch(const SkData& data) {
    (void)data.bytes();
    (void)data.size();
}

SKIALIN_NOINLINE void touch(const SkImage& image) {
    (void)image.height();
    (void)image.width();
}

SKIALIN_NOINLINE void touch(SkMatrix& matrix) {
    matrix.setIdentity();
    float buffer[9];
    matrix.get9(buffer);
    matrix.invert(nullptr);
    (void)matrix.isIdentity();
    (void)matrix.mapRect(SkRect::MakeEmpty());
}

SKIALIN_NOINLINE void touch(const SkPaint& paint, SkPaint& mutablePaint) {
    (void)paint.getColor();
    (void)paint.getStrokeCap();
    (void)paint.getStrokeJoin();
    (void)paint.getStrokeWidth();
    (void)paint.getStyle();
    (void)paint.isAntiAlias();
    (void)paint.isDither();
    (void)paint.getAlphaf();
    (void)paint.getStrokeMiter();
    mutablePaint.setAntiAlias(false);
    mutablePaint.setDither(false);
    mutablePaint.setAlpha(0);
}

SKIALIN_NOINLINE void touch(const SkPath& path) {
    (void)path.getFillType();
}

SKIALIN_NOINLINE void touch(const SkPathBuilder& builder) {
    (void)builder.isEmpty();
    (void)builder.computeBounds();
}

SKIALIN_NOINLINE void touch(const SkData& data, SkData& mutableData) {
    (void)data.size();
    (void)data.empty();
    (void)data.isEmpty();
    (void)data.data();
    (void)data.bytes();
    (void)mutableData.writable_data();
    (void)data.equals(&data);
}

SKIALIN_NOINLINE void touch(const SkColorSpace& cs) {
    (void)cs.gammaCloseToSRGB();
    (void)cs.gammaIsLinear();
    (void)cs.isSRGB();
    (void)cs.toXYZD50Hash();
    (void)cs.transferFnHash();
    (void)cs.hash();
}

SKIALIN_NOINLINE void touch(const SkImageInfo& info) {
    (void)info.width();
    (void)info.height();
    (void)info.colorType();
    (void)info.alphaType();
    (void)info.isEmpty();
    (void)info.isOpaque();
    (void)info.gammaCloseToSRGB();
    (void)info.bytesPerPixel();
    (void)info.shiftPerPixel();
    (void)info.minRowBytes64();
    (void)info.minRowBytes();
    (void)info.computeMinByteSize();
    (void)info.validRowBytes(0);
}

SKIALIN_NOINLINE void touch(const SkPixmap& pixmap) {
    (void)pixmap.rowBytes();
    (void)pixmap.addr();
    (void)pixmap.width();
    (void)pixmap.height();
    (void)pixmap.isEmpty();
    (void)pixmap.colorType();
    (void)pixmap.alphaType();
    (void)pixmap.isOpaque();
    (void)pixmap.bounds();
    (void)pixmap.rowBytesAsPixels();
    (void)pixmap.shiftPerPixel();
    (void)pixmap.computeByteSize();
    (void)pixmap.getColor(0, 0);
    (void)pixmap.getAlphaf(0, 0);
    (void)pixmap.writable_addr();
}

SKIALIN_NOINLINE void touch(const SkFont& font) {
    (void)font.getSize();
    (void)font.getScaleX();
    (void)font.getSkewX();
    (void)font.getEdging();
    (void)font.getHinting();
    (void)font.isSubpixel();
    (void)font.isEmbolden();
    (void)font.isLinearMetrics();
    (void)font.isForceAutoHinting();
    (void)font.isEmbeddedBitmaps();
    (void)font.isBaselineSnap();
    (void)font.getSpacing();
    (void)font.countText(nullptr, 0, SkTextEncoding::kUTF8);
    (void)font.measureText(nullptr, 0, SkTextEncoding::kUTF8);
    font.getWidths({}, {});
}

SKIALIN_NOINLINE void touch(const SkTextBlob& blob) {
    (void)blob.bounds();
    (void)blob.uniqueID();
}

}  // namespace skialin_force_link
