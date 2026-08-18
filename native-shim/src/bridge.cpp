#include "skialin/bridge.h"

#include "include/core/SkSurface.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkImage.h"
#include "include/core/SkData.h"
#include "include/core/SkBitmap.h"
#include "include/core/SkPath.h"
#include "include/core/SkPathBuilder.h"
#include "include/encode/SkPngEncoder.h"

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

}  // extern "C"
