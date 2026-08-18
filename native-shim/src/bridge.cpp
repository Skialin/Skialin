#include "skialin/bridge.h"

#include <cstring>

#include "include/core/SkSurface.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkImage.h"
#include "include/core/SkData.h"
#include "include/core/SkBitmap.h"
#include "include/core/SkPath.h"
#include "include/core/SkPathBuilder.h"
#include "include/core/SkColorSpace.h"
#include "include/encode/SkPngEncoder.h"
#include "modules/skcms/skcms.h"

namespace {

skcms_TransferFunction toTransferFn(const float* fn7) {
    return skcms_TransferFunction{fn7[0], fn7[1], fn7[2], fn7[3], fn7[4], fn7[5], fn7[6]};
}

void fromTransferFn(const skcms_TransferFunction& fn, float* out7) {
    out7[0] = fn.g;
    out7[1] = fn.a;
    out7[2] = fn.b;
    out7[3] = fn.c;
    out7[4] = fn.d;
    out7[5] = fn.e;
    out7[6] = fn.f;
}

skcms_Matrix3x3 toMatrix(const float* m9) {
    skcms_Matrix3x3 m;
    memcpy(m.vals, m9, sizeof(m.vals));
    return m;
}

void fromMatrix(const skcms_Matrix3x3& m, float* out9) {
    memcpy(out9, m.vals, sizeof(m.vals));
}

}  // namespace

extern "C" {

SkPoint skialin_bridge_Matrix_mapPoint(const SkMatrix* matrix, SkPoint point) {
    return matrix->mapPoint(point);
}

SkRect skialin_bridge_Matrix_mapRect(const SkMatrix* matrix, const SkRect* rect) {
    return matrix->mapRect(*rect);
}

SkRect skialin_bridge_PathBuilder_computeBounds(const SkPathBuilder* builder) {
    return builder->computeBounds();
}

SkPath* skialin_bridge_PathBuilder_snapshot(const SkPathBuilder* builder, const SkMatrix* matrix) {
    return new SkPath(builder->snapshot(matrix));
}

SkPath* skialin_bridge_PathBuilder_detach(SkPathBuilder* builder, const SkMatrix* matrix) {
    return new SkPath(builder->detach(matrix));
}

void skialin_bridge_Path_delete(SkPath* path) {
    delete path;
}

SkSurface* skialin_bridge_Surface_MakeRasterN32Premul(int32_t width, int32_t height) {
    return SkSurfaces::Raster(SkImageInfo::MakeN32Premul(width, height)).release();
}

SkSurface* skialin_bridge_Surface_MakeRaster(const SkImageInfo* info) {
    return SkSurfaces::Raster(*info).release();
}

void skialin_bridge_Surface_unref(SkSurface* surface) {
    SkSafeUnref(surface);
}

SkCanvas* skialin_bridge_Surface_getCanvas(SkSurface* surface) {
    return surface->getCanvas();
}

SkImage* skialin_bridge_Surface_makeImageSnapshot(SkSurface* surface) {
    return surface->makeImageSnapshot().release();
}

void skialin_bridge_Image_unref(SkImage* image) {
    SkSafeUnref(image);
}

SkImage* skialin_bridge_Image_MakeFromEncoded(const uint8_t* bytes, size_t length) {
    sk_sp<SkData> data = SkData::MakeWithCopy(bytes, length);
    return SkImages::DeferredFromEncodedData(data).release();
}

SkData* skialin_bridge_Image_encodeToData(const SkImage* image) {
    return SkPngEncoder::Encode(nullptr, image, {}).release();
}

SkImage* skialin_bridge_Bitmap_asImage(const SkBitmap* bitmap) {
    return SkImages::RasterFromBitmap(*bitmap).release();
}

void skialin_bridge_Data_unref(SkData* data) {
    SkSafeUnref(data);
}

SkData* skialin_bridge_Data_makeEmpty(void) {
    return SkData::MakeEmpty().release();
}

SkData* skialin_bridge_Data_makeWithCopy(const void* data, size_t length) {
    return SkData::MakeWithCopy(data, length).release();
}

SkData* skialin_bridge_Data_makeUninitialized(size_t length) {
    return SkData::MakeUninitialized(length).release();
}

SkData* skialin_bridge_Data_makeZeroInitialized(size_t length) {
    return SkData::MakeZeroInitialized(length).release();
}

SkData* skialin_bridge_Data_makeFromFileName(const char* path) {
    return SkData::MakeFromFileName(path).release();
}

SkData* skialin_bridge_Data_copySubset(const SkData* data, size_t offset, size_t length) {
    return data->copySubset(offset, length).release();
}

SkData* skialin_bridge_Data_shareSubset(const SkData* data, size_t offset, size_t length) {
    return const_cast<SkData*>(data)->shareSubset(offset, length).release();
}

void skialin_bridge_ColorSpace_unref(SkColorSpace* cs) {
    SkSafeUnref(cs);
}

SkColorSpace* skialin_bridge_ColorSpace_makeSRGB(void) {
    return SkColorSpace::MakeSRGB().release();
}

SkColorSpace* skialin_bridge_ColorSpace_makeSRGBLinear(void) {
    return SkColorSpace::MakeSRGBLinear().release();
}

SkColorSpace* skialin_bridge_ColorSpace_makeRGB(const float* transferFn7, const float* toXyz9) {
    return SkColorSpace::MakeRGB(toTransferFn(transferFn7), toMatrix(toXyz9)).release();
}

SkColorSpace* skialin_bridge_ColorSpace_makeCICP(uint8_t colorPrimaries, uint8_t transferCharacteristics) {
    auto primaries = static_cast<SkNamedPrimaries::CicpId>(colorPrimaries);
    auto transfer = static_cast<SkNamedTransferFn::CicpId>(transferCharacteristics);
    return SkColorSpace::MakeCICP(primaries, transfer).release();
}

SkColorSpace* skialin_bridge_ColorSpace_makeFromIccProfile(const uint8_t* bytes, size_t length) {
    skcms_ICCProfile profile;
    if (!skcms_Parse(bytes, length, &profile)) {
        return nullptr;
    }
    return SkColorSpace::Make(profile).release();
}

SkColorSpace* skialin_bridge_ColorSpace_deserialize(const uint8_t* bytes, size_t length) {
    return SkColorSpace::Deserialize(bytes, length).release();
}

SkColorSpace* skialin_bridge_ColorSpace_makeLinearGamma(const SkColorSpace* cs) {
    return cs->makeLinearGamma().release();
}

SkColorSpace* skialin_bridge_ColorSpace_makeSRGBGamma(const SkColorSpace* cs) {
    return cs->makeSRGBGamma().release();
}

SkColorSpace* skialin_bridge_ColorSpace_makeColorSpin(const SkColorSpace* cs) {
    return cs->makeColorSpin().release();
}

bool skialin_bridge_ColorSpace_toXYZD50(const SkColorSpace* cs, float* outXyz9) {
    skcms_Matrix3x3 m;
    if (!cs->toXYZD50(&m)) {
        return false;
    }
    fromMatrix(m, outXyz9);
    return true;
}

void skialin_bridge_ColorSpace_transferFn(const SkColorSpace* cs, float* outFn7) {
    skcms_TransferFunction fn;
    cs->transferFn(&fn);
    fromTransferFn(fn, outFn7);
}

void skialin_bridge_ColorSpace_invTransferFn(const SkColorSpace* cs, float* outFn7) {
    skcms_TransferFunction fn;
    cs->invTransferFn(&fn);
    fromTransferFn(fn, outFn7);
}

bool skialin_bridge_ColorSpace_isNumericalTransferFn(const SkColorSpace* cs, float* outFn7) {
    skcms_TransferFunction fn;
    if (!cs->isNumericalTransferFn(&fn)) {
        return false;
    }
    fromTransferFn(fn, outFn7);
    return true;
}

void skialin_bridge_ColorSpace_gamutTransformTo(const SkColorSpace* src, const SkColorSpace* dst, float* outXyz9) {
    skcms_Matrix3x3 m;
    src->gamutTransformTo(dst, &m);
    fromMatrix(m, outXyz9);
}

SkData* skialin_bridge_ColorSpace_serialize(const SkColorSpace* cs) {
    return cs->serialize().release();
}

bool skialin_bridge_ColorSpace_equals(const SkColorSpace* a, const SkColorSpace* b) {
    return SkColorSpace::Equals(a, b);
}

}  // extern "C"
