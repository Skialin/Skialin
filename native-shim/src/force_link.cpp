#include "include/core/SkBitmap.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkData.h"
#include "include/core/SkImage.h"
#include "include/core/SkMatrix.h"
#include "include/core/SkPaint.h"
#include "include/core/SkPathBuilder.h"

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
    (void)matrix.mapRect(SkRect::MakeEmpty());
}

SKIALIN_NOINLINE void touch(const SkPaint& paint, SkPaint& mutablePaint) {
    (void)paint.getColor();
    (void)paint.getStrokeCap();
    (void)paint.getStrokeJoin();
    (void)paint.getStrokeWidth();
    (void)paint.getStyle();
    (void)paint.isAntiAlias();
    mutablePaint.setAntiAlias(false);
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

}  // namespace skialin_force_link
