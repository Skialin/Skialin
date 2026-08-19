#include "skialin/bridge.h"

#include <cstring>
#include <memory>

#include "include/core/SkSurface.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkPicture.h"
#include "include/core/SkPictureRecorder.h"
#include "include/core/SkPoint3.h"
#include "include/utils/SkShadowUtils.h"
#include "include/core/SkImage.h"
#include "include/core/SkData.h"
#include "include/core/SkBitmap.h"
#include "include/core/SkPath.h"
#include "include/core/SkDrawable.h"
#include "include/core/SkPathBuilder.h"
#include "include/core/SkColorSpace.h"
#include "include/core/SkPixmap.h"
#include "include/core/SkPaint.h"
#include "include/core/SkShader.h"
#include "include/core/SkBlender.h"
#include "include/core/SkSamplingOptions.h"
#include "include/core/SkTypeface.h"
#include "include/core/SkFontStyle.h"
#include "include/core/SkFontMgr.h"
#include "include/core/SkFont.h"
#include "include/core/SkTextBlob.h"
#include "include/core/SkSerialProcs.h"
#include "modules/skparagraph/include/TextStyle.h"
#include "modules/skparagraph/include/ParagraphStyle.h"
#include "modules/skparagraph/include/FontCollection.h"
#include "modules/skparagraph/include/ParagraphBuilder.h"
#include "modules/skparagraph/include/Paragraph.h"
#include "modules/skparagraph/include/Metrics.h"
#include "modules/skunicode/include/SkUnicode_icu.h"
#include "include/core/SkColorFilter.h"
#include "include/core/SkImageFilter.h"
#include "include/core/SkMaskFilter.h"
#include "include/core/SkBlurTypes.h"
#include "include/effects/SkImageFilters.h"
#include "include/effects/SkGradient.h"
#include "include/effects/SkRuntimeEffect.h"
#include "include/core/SkRRect.h"
#include "include/core/SkRegion.h"
#include "modules/svg/include/SkSVGDOM.h"
#include "modules/svg/include/SkSVGSVG.h"
#include "include/svg/SkSVGCanvas.h"
#include "modules/skottie/include/Skottie.h"
#include "include/core/SkPathEffect.h"
#include "include/effects/SkDashPathEffect.h"
#include "include/effects/SkCornerPathEffect.h"
#include "include/effects/SkDiscretePathEffect.h"
#include "include/effects/SkTrimPathEffect.h"
#include "include/effects/Sk1DPathEffect.h"
#include "include/effects/Sk2DPathEffect.h"
#include "include/core/SkPathMeasure.h"
#include "include/pathops/SkPathOps.h"
#include "include/core/SkM44.h"
#include "include/core/SkVertices.h"
#include "include/effects/SkColorMatrix.h"
#include "include/effects/SkHighContrastFilter.h"
#include "include/effects/SkLumaColorFilter.h"
#include "include/effects/SkPerlinNoiseShader.h"
#include "include/core/SkString.h"
#include "include/core/SkStream.h"
#include "include/encode/SkPngEncoder.h"
#include "include/encode/SkJpegEncoder.h"
#include "include/encode/SkWebpEncoder.h"
#include "include/codec/SkCodec.h"
#include "include/core/SkTypes.h"
#if defined(SK_BUILD_FOR_WIN)
#include "include/ports/SkTypeface_win.h"
#elif defined(SK_BUILD_FOR_MAC)
#include "include/ports/SkFontMgr_mac_ct.h"
#elif defined(SK_BUILD_FOR_UNIX)
#include "include/ports/SkFontMgr_fontconfig.h"
#include "include/ports/SkFontScanner_FreeType.h"
#endif
#include "modules/skcms/skcms.h"
#include "include/gpu/ganesh/GrBackendSurface.h"
#include "include/gpu/ganesh/GrDirectContext.h"
#include "include/gpu/ganesh/GrTypes.h"
#include "include/gpu/ganesh/SkSurfaceGanesh.h"
#include "include/gpu/ganesh/gl/GrGLDirectContext.h"
#include "include/gpu/ganesh/gl/GrGLBackendSurface.h"
#include "include/gpu/ganesh/vk/GrVkBackendSurface.h"
#include "include/gpu/ganesh/vk/GrVkDirectContext.h"
#include "include/gpu/graphite/BackendTexture.h"
#include "include/gpu/graphite/Context.h"
#include "include/gpu/graphite/ContextOptions.h"
#include "include/gpu/graphite/Recorder.h"
#include "include/gpu/graphite/Recording.h"
#include "include/gpu/graphite/Surface.h"
#include "include/gpu/graphite/vk/VulkanGraphiteContext.h"
#include "include/gpu/graphite/vk/VulkanGraphiteTypes.h"
#include "src/gpu/GpuTypesPriv.h"
#include "src/gpu/vk/vulkanmemoryallocator/VulkanMemoryAllocatorPriv.h"

namespace {

SkSamplingOptions toSamplingOptions(int32_t maxAniso, bool useCubic, float cubicB, float cubicC, SkFilterMode filter, SkMipmapMode mipmap) {
    if (maxAniso != 0) {
        return SkSamplingOptions::Aniso(maxAniso);
    }
    if (useCubic) {
        return SkSamplingOptions(SkCubicResampler{cubicB, cubicC});
    }
    return SkSamplingOptions(filter, mipmap);
}

std::vector<SkColor4f> toColor4fVec(const uint32_t* colors, size_t count) {
    std::vector<SkColor4f> result;
    result.reserve(count);
    for (size_t i = 0; i < count; ++i) {
        result.push_back(SkColor4f::FromColor(colors[i]));
    }
    return result;
}

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

void skialin_bridge_Canvas_getTotalMatrix(const SkCanvas* canvas, SkMatrix* outMatrix) {
    *outMatrix = canvas->getTotalMatrix();
}

int32_t skialin_bridge_Canvas_saveLayer(SkCanvas* canvas, const SkRect* bounds, const SkPaint* paint, SkImageFilter* backdrop, uint32_t flags) {
    SkCanvas::SaveLayerRec rec(bounds, paint, backdrop, static_cast<SkCanvas::SaveLayerFlags>(flags));
    return canvas->saveLayer(rec);
}

SkM44* skialin_bridge_Canvas_getLocalToDevice(const SkCanvas* canvas) {
    return new SkM44(canvas->getLocalToDevice());
}

SkCanvas* skialin_bridge_Canvas_newFromBitmap(const SkBitmap* bitmap) {
    return new SkCanvas(*bitmap);
}

void skialin_bridge_Canvas_deleteOwned(SkCanvas* canvas) {
    delete canvas;
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

SkPath* skialin_bridge_Path_clone(const SkPath* path) {
    return new SkPath(*path);
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

SkImage* skialin_bridge_Surface_makeImageSnapshotArea(SkSurface* surface, const SkIRect* bounds) {
    return surface->makeImageSnapshot(*bounds).release();
}

SkImageInfo* skialin_bridge_Surface_imageInfo(SkSurface* surface) {
    return new SkImageInfo(surface->imageInfo());
}

void skialin_bridge_Surface_notifyContentWillChange(SkSurface* surface, int32_t mode) {
    surface->notifyContentWillChange(static_cast<SkSurface::ContentChangeMode>(mode));
}

void skialin_bridge_Surface_flush(SkSurface* surface) {
    skgpu::ganesh::FlushAndSubmit(surface);
}

void skialin_bridge_Surface_draw(SkSurface* surface, SkCanvas* canvas, float x, float y, const SkPaint* paint) {
    surface->draw(canvas, x, y, SkSamplingOptions(), paint);
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

SkData* skialin_bridge_Image_encodeToDataJpeg(const SkImage* image, int32_t quality) {
    SkJpegEncoder::Options options;
    options.fQuality = quality;
    return SkJpegEncoder::Encode(nullptr, image, options).release();
}

SkData* skialin_bridge_Image_encodeToDataWebp(const SkImage* image, float quality, bool lossless) {
    SkWebpEncoder::Options options;
    options.fCompression = lossless ? SkWebpEncoder::Compression::kLossless : SkWebpEncoder::Compression::kLossy;
    options.fQuality = quality;
    return SkWebpEncoder::Encode(nullptr, image, options).release();
}

SkImage* skialin_bridge_Bitmap_asImage(const SkBitmap* bitmap) {
    return SkImages::RasterFromBitmap(*bitmap).release();
}

int32_t skialin_bridge_Image_width(const SkImage* image) {
    return image->width();
}

int32_t skialin_bridge_Image_height(const SkImage* image) {
    return image->height();
}

uint32_t skialin_bridge_Image_uniqueID(const SkImage* image) {
    return image->uniqueID();
}

SkAlphaType skialin_bridge_Image_alphaType(const SkImage* image) {
    return image->alphaType();
}

SkColorType skialin_bridge_Image_colorType(const SkImage* image) {
    return image->colorType();
}

SkColorSpace* skialin_bridge_Image_colorSpace(const SkImage* image) {
    return image->colorSpace();
}

SkImageInfo* skialin_bridge_Image_imageInfo(const SkImage* image) {
    return new SkImageInfo(image->imageInfo());
}

bool skialin_bridge_Image_isAlphaOnly(const SkImage* image) {
    return image->isAlphaOnly();
}

bool skialin_bridge_Image_isOpaque(const SkImage* image) {
    return image->isOpaque();
}

bool skialin_bridge_Image_isTextureBacked(const SkImage* image) {
    return image->isTextureBacked();
}

bool skialin_bridge_Image_isLazyGenerated(const SkImage* image) {
    return image->isLazyGenerated();
}

bool skialin_bridge_Image_hasMipmaps(const SkImage* image) {
    return image->hasMipmaps();
}

bool skialin_bridge_Image_isProtected(const SkImage* image) {
    return image->isProtected();
}

SkColorSpace* skialin_bridge_Image_refColorSpace(const SkImage* image) {
    return image->refColorSpace().release();
}

SkShader* skialin_bridge_Image_makeShader(
    const SkImage* image, SkTileMode tmx, SkTileMode tmy,
    int32_t maxAniso, bool useCubic, float cubicB, float cubicC, SkFilterMode filter, SkMipmapMode mipmap,
    const SkMatrix* localMatrix) {
    auto sampling = toSamplingOptions(maxAniso, useCubic, cubicB, cubicC, filter, mipmap);
    return image->makeShader(tmx, tmy, sampling, localMatrix).release();
}

SkShader* skialin_bridge_Image_makeRawShader(
    const SkImage* image, SkTileMode tmx, SkTileMode tmy,
    int32_t maxAniso, bool useCubic, float cubicB, float cubicC, SkFilterMode filter, SkMipmapMode mipmap,
    const SkMatrix* localMatrix) {
    auto sampling = toSamplingOptions(maxAniso, useCubic, cubicB, cubicC, filter, mipmap);
    return image->makeRawShader(tmx, tmy, sampling, localMatrix).release();
}

bool skialin_bridge_Image_peekPixels(const SkImage* image, SkPixmap* pixmap) {
    return image->peekPixels(pixmap);
}

bool skialin_bridge_Image_readPixels(const SkImage* image, const SkImageInfo* dstInfo, void* dstPixels, size_t dstRowBytes, int32_t srcX, int32_t srcY) {
    return image->readPixels(*dstInfo, dstPixels, dstRowBytes, srcX, srcY);
}

bool skialin_bridge_Image_scalePixels(
    const SkImage* image, SkPixmap* dst,
    int32_t maxAniso, bool useCubic, float cubicB, float cubicC, SkFilterMode filter, SkMipmapMode mipmap) {
    auto sampling = toSamplingOptions(maxAniso, useCubic, cubicB, cubicC, filter, mipmap);
    return image->scalePixels(*dst, sampling);
}

SkImage* skialin_bridge_Image_makeScaled(
    const SkImage* image, const SkImageInfo* info,
    int32_t maxAniso, bool useCubic, float cubicB, float cubicC, SkFilterMode filter, SkMipmapMode mipmap) {
    auto sampling = toSamplingOptions(maxAniso, useCubic, cubicB, cubicC, filter, mipmap);
    return image->makeScaled(*info, sampling).release();
}

SkData* skialin_bridge_Image_refEncodedData(const SkImage* image) {
    return const_cast<SkData*>(image->refEncodedData().release());
}

SkImage* skialin_bridge_Image_makeSubset(const SkImage* image, int32_t left, int32_t top, int32_t right, int32_t bottom, bool mipmapped) {
    SkImage::RequiredProperties props{mipmapped};
    return image->makeSubset(nullptr, SkIRect::MakeLTRB(left, top, right, bottom), props).release();
}

SkImage* skialin_bridge_Image_withDefaultMipmaps(const SkImage* image) {
    return image->withDefaultMipmaps().release();
}

SkImage* skialin_bridge_Image_makeNonTextureImage(const SkImage* image) {
    return image->makeNonTextureImage().release();
}

SkImage* skialin_bridge_Image_makeRasterImage(const SkImage* image, bool allowCaching) {
    auto hint = allowCaching ? SkImage::kAllow_CachingHint : SkImage::kDisallow_CachingHint;
    return image->makeRasterImage(nullptr, hint).release();
}

bool skialin_bridge_Image_asLegacyBitmap(const SkImage* image, SkBitmap* bitmap) {
    return image->asLegacyBitmap(bitmap);
}

SkImage* skialin_bridge_Image_makeColorSpace(const SkImage* image, SkColorSpace* targetColorSpace, bool mipmapped) {
    SkImage::RequiredProperties props{mipmapped};
    return image->makeColorSpace(nullptr, sk_ref_sp(targetColorSpace), props).release();
}

SkImage* skialin_bridge_Image_makeColorTypeAndColorSpace(const SkImage* image, SkColorType targetColorType, SkColorSpace* targetColorSpace, bool mipmapped) {
    SkImage::RequiredProperties props{mipmapped};
    return image->makeColorTypeAndColorSpace(nullptr, targetColorType, sk_ref_sp(targetColorSpace), props).release();
}

SkImage* skialin_bridge_Image_reinterpretColorSpace(const SkImage* image, SkColorSpace* newColorSpace) {
    return image->reinterpretColorSpace(sk_ref_sp(newColorSpace)).release();
}

SkImage* skialin_bridge_Image_RasterFromPixmapCopy(const SkPixmap* pixmap) {
    return SkImages::RasterFromPixmapCopy(*pixmap).release();
}

SkImage* skialin_bridge_Image_MakeFromBitmap(const SkBitmap* bitmap) {
    return SkImages::RasterFromBitmap(*bitmap).release();
}

SkImage* skialin_bridge_Image_RasterFromData(const SkImageInfo* info, SkData* pixels, size_t rowBytes) {
    return SkImages::RasterFromData(*info, sk_ref_sp(pixels), rowBytes).release();
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

SkImageInfo* skialin_bridge_ImageInfo_make(int32_t width, int32_t height, SkColorType colorType, SkAlphaType alphaType, SkColorSpace* colorSpace) {
    return new SkImageInfo(SkImageInfo::Make(width, height, colorType, alphaType, sk_ref_sp(colorSpace)));
}

void skialin_bridge_ImageInfo_delete(SkImageInfo* info) {
    delete info;
}

SkImageInfo* skialin_bridge_ImageInfo_makeWH(const SkImageInfo* info, int32_t width, int32_t height) {
    return new SkImageInfo(info->makeWH(width, height));
}

SkImageInfo* skialin_bridge_ImageInfo_makeColorType(const SkImageInfo* info, SkColorType colorType) {
    return new SkImageInfo(info->makeColorType(colorType));
}

SkImageInfo* skialin_bridge_ImageInfo_makeAlphaType(const SkImageInfo* info, SkAlphaType alphaType) {
    return new SkImageInfo(info->makeAlphaType(alphaType));
}

SkImageInfo* skialin_bridge_ImageInfo_makeColorSpace(const SkImageInfo* info, SkColorSpace* colorSpace) {
    return new SkImageInfo(info->makeColorSpace(sk_ref_sp(colorSpace)));
}

SkColorSpace* skialin_bridge_ImageInfo_colorSpace(const SkImageInfo* info) {
    return info->colorSpace();
}

SkColorSpace* skialin_bridge_ImageInfo_refColorSpace(const SkImageInfo* info) {
    return info->refColorSpace().release();
}

bool skialin_bridge_ImageInfo_equals(const SkImageInfo* a, const SkImageInfo* b) {
    return *a == *b;
}

SkPixmap* skialin_bridge_Pixmap_make(const SkImageInfo* info, const void* addr, size_t rowBytes) {
    return new SkPixmap(*info, addr, rowBytes);
}

SkPixmap* skialin_bridge_Pixmap_makeEmpty(void) {
    return new SkPixmap();
}

void skialin_bridge_Pixmap_delete(SkPixmap* pixmap) {
    delete pixmap;
}

SkColorSpace* skialin_bridge_Pixmap_refColorSpace(const SkPixmap* pixmap) {
    return pixmap->refColorSpace().release();
}

SkPixmap* skialin_bridge_Pixmap_extractSubset(const SkPixmap* pixmap, int32_t left, int32_t top, int32_t right, int32_t bottom) {
    auto subset = std::make_unique<SkPixmap>();
    if (!pixmap->extractSubset(subset.get(), SkIRect::MakeLTRB(left, top, right, bottom))) {
        return nullptr;
    }
    return subset.release();
}

void skialin_bridge_Shader_unref(SkShader* shader) {
    SkSafeUnref(shader);
}

SkShader* skialin_bridge_Shader_makeEmpty(void) {
    return SkShaders::Empty().release();
}

SkShader* skialin_bridge_Shader_makeColor(uint32_t argb) {
    return SkShaders::Color(argb).release();
}

SkShader* skialin_bridge_Shader_makeWithLocalMatrix(const SkShader* shader, const SkMatrix* localMatrix) {
    return shader->makeWithLocalMatrix(*localMatrix).release();
}

bool skialin_bridge_Shader_isOpaque(const SkShader* shader) {
    return shader->isOpaque();
}

void skialin_bridge_Paint_setShader(SkPaint* paint, SkShader* shader) {
    paint->setShader(sk_ref_sp(shader));
}

SkShader* skialin_bridge_Shader_makeLinearGradient(
    const SkPoint* pts, const uint32_t* colors, const float* positions, size_t count, SkTileMode tileMode, const SkMatrix* localMatrix) {
    std::vector<SkColor4f> color4fs = toColor4fVec(colors, count);
    SkGradient::Colors gradColors(color4fs, positions ? SkSpan<const float>(positions, count) : SkSpan<const float>(), tileMode);
    SkGradient gradient(gradColors, {});
    return SkShaders::LinearGradient(pts, gradient, localMatrix).release();
}

SkShader* skialin_bridge_Shader_makeRadialGradient(
    SkPoint center, float radius, const uint32_t* colors, const float* positions, size_t count, SkTileMode tileMode, const SkMatrix* localMatrix) {
    std::vector<SkColor4f> color4fs = toColor4fVec(colors, count);
    SkGradient::Colors gradColors(color4fs, positions ? SkSpan<const float>(positions, count) : SkSpan<const float>(), tileMode);
    SkGradient gradient(gradColors, {});
    return SkShaders::RadialGradient(center, radius, gradient, localMatrix).release();
}

SkShader* skialin_bridge_Shader_makeTwoPointConicalGradient(
    SkPoint start, float startRadius, SkPoint end, float endRadius,
    const uint32_t* colors, const float* positions, size_t count, SkTileMode tileMode, const SkMatrix* localMatrix) {
    std::vector<SkColor4f> color4fs = toColor4fVec(colors, count);
    SkGradient::Colors gradColors(color4fs, positions ? SkSpan<const float>(positions, count) : SkSpan<const float>(), tileMode);
    SkGradient gradient(gradColors, {});
    return SkShaders::TwoPointConicalGradient(start, startRadius, end, endRadius, gradient, localMatrix).release();
}

SkShader* skialin_bridge_Shader_makeSweepGradient(
    SkPoint center, float startAngle, float endAngle,
    const uint32_t* colors, const float* positions, size_t count, SkTileMode tileMode, const SkMatrix* localMatrix) {
    std::vector<SkColor4f> color4fs = toColor4fVec(colors, count);
    SkGradient::Colors gradColors(color4fs, positions ? SkSpan<const float>(positions, count) : SkSpan<const float>(), tileMode);
    SkGradient gradient(gradColors, {});
    return SkShaders::SweepGradient(center, startAngle, endAngle, gradient, localMatrix).release();
}

SkShader* skialin_bridge_Shader_BlendBlender(SkBlender* blender, SkShader* dst, SkShader* src) {
    return SkShaders::Blend(sk_ref_sp(blender), sk_ref_sp(dst), sk_ref_sp(src)).release();
}

SkShader* skialin_bridge_Shader_Blend(SkBlendMode mode, SkShader* dst, SkShader* src) {
    return SkShaders::Blend(mode, sk_ref_sp(dst), sk_ref_sp(src)).release();
}

SkShader* skialin_bridge_Shader_MakeFractalNoise(float baseFreqX, float baseFreqY, int32_t numOctaves, float seed) {
    return SkShaders::MakeFractalNoise(baseFreqX, baseFreqY, numOctaves, seed).release();
}

SkShader* skialin_bridge_Shader_MakeTurbulence(float baseFreqX, float baseFreqY, int32_t numOctaves, float seed) {
    return SkShaders::MakeTurbulence(baseFreqX, baseFreqY, numOctaves, seed).release();
}

SkRuntimeEffect* skialin_bridge_RuntimeEffect_MakeForShader(const char* sksl, size_t length, SkData** outError) {
    SkRuntimeEffect::Result result = SkRuntimeEffect::MakeForShader(SkString(sksl, length));
    if (!result.effect) {
        *outError = SkData::MakeWithCopy(result.errorText.c_str(), result.errorText.size()).release();
        return nullptr;
    }
    return result.effect.release();
}

SkRuntimeEffect* skialin_bridge_RuntimeEffect_MakeForColorFilter(const char* sksl, size_t length, SkData** outError) {
    SkRuntimeEffect::Result result = SkRuntimeEffect::MakeForColorFilter(SkString(sksl, length));
    if (!result.effect) {
        *outError = SkData::MakeWithCopy(result.errorText.c_str(), result.errorText.size()).release();
        return nullptr;
    }
    return result.effect.release();
}

void skialin_bridge_RuntimeEffect_unref(SkRuntimeEffect* effect) {
    SkSafeUnref(effect);
}

SkShader* skialin_bridge_RuntimeEffect_makeShader(
    const SkRuntimeEffect* effect, const uint8_t* uniforms, size_t uniformsLength,
    SkShader* const* children, size_t childCount, const SkMatrix* localMatrix) {
    sk_sp<SkData> uniformData = SkData::MakeWithCopy(uniforms, uniformsLength);
    std::vector<sk_sp<SkShader>> childRefs;
    childRefs.reserve(childCount);
    for (size_t i = 0; i < childCount; ++i) {
        childRefs.push_back(sk_ref_sp(children[i]));
    }
    return effect->makeShader(uniformData, childRefs.data(), childCount, localMatrix).release();
}

SkColorFilter* skialin_bridge_RuntimeEffect_makeColorFilter(
    const SkRuntimeEffect* effect, const uint8_t* uniforms, size_t uniformsLength,
    SkColorFilter* const* children, size_t childCount) {
    sk_sp<SkData> uniformData = SkData::MakeWithCopy(uniforms, uniformsLength);
    std::vector<sk_sp<SkColorFilter>> childRefs;
    childRefs.reserve(childCount);
    for (size_t i = 0; i < childCount; ++i) {
        childRefs.push_back(sk_ref_sp(children[i]));
    }
    return effect->makeColorFilter(uniformData, childRefs.data(), childCount).release();
}

SkRuntimeEffect* skialin_bridge_RuntimeEffect_MakeForBlender(const char* sksl, size_t length, SkData** outError) {
    SkRuntimeEffect::Result result = SkRuntimeEffect::MakeForBlender(SkString(sksl, length));
    if (!result.effect) {
        *outError = SkData::MakeWithCopy(result.errorText.c_str(), result.errorText.size()).release();
        return nullptr;
    }
    return result.effect.release();
}

SkBlender* skialin_bridge_RuntimeEffect_makeBlender(
    const SkRuntimeEffect* effect, const uint8_t* uniforms, size_t uniformsLength,
    SkShader* const* children, size_t childCount) {
    sk_sp<SkData> uniformData = SkData::MakeWithCopy(uniforms, uniformsLength);
    std::vector<SkRuntimeEffect::ChildPtr> childRefs;
    childRefs.reserve(childCount);
    for (size_t i = 0; i < childCount; ++i) {
        childRefs.push_back(sk_ref_sp(children[i]));
    }
    return effect->makeBlender(uniformData, SkSpan<const SkRuntimeEffect::ChildPtr>(childRefs.data(), childRefs.size())).release();
}

SkBlender* skialin_bridge_Blender_Mode(SkBlendMode mode) {
    return SkBlender::Mode(mode).release();
}

void skialin_bridge_Blender_unref(SkBlender* blender) {
    SkSafeUnref(blender);
}

void skialin_bridge_Typeface_unref(SkTypeface* typeface) {
    SkSafeUnref(typeface);
}

SkTypeface* skialin_bridge_Typeface_MakeEmpty(void) {
    return SkTypeface::MakeEmpty().release();
}

uint32_t skialin_bridge_Typeface_uniqueID(const SkTypeface* typeface) {
    return typeface->uniqueID();
}

bool skialin_bridge_Typeface_isBold(const SkTypeface* typeface) {
    return typeface->isBold();
}

bool skialin_bridge_Typeface_isItalic(const SkTypeface* typeface) {
    return typeface->isItalic();
}

bool skialin_bridge_Typeface_isFixedPitch(const SkTypeface* typeface) {
    return typeface->isFixedPitch();
}

int32_t skialin_bridge_Typeface_countGlyphs(const SkTypeface* typeface) {
    return typeface->countGlyphs();
}

int32_t skialin_bridge_Typeface_getUnitsPerEm(const SkTypeface* typeface) {
    return typeface->getUnitsPerEm();
}

uint16_t skialin_bridge_Typeface_unicharToGlyph(const SkTypeface* typeface, int32_t unichar) {
    return typeface->unicharToGlyph(unichar);
}

void skialin_bridge_Typeface_fontStyle(const SkTypeface* typeface, int32_t* weight, int32_t* width, int32_t* slant) {
    SkFontStyle style = typeface->fontStyle();
    *weight = style.weight();
    *width = style.width();
    *slant = style.slant();
}

SkData* skialin_bridge_Typeface_familyName(const SkTypeface* typeface) {
    SkString name;
    typeface->getFamilyName(&name);
    return SkData::MakeWithCopy(name.c_str(), name.size()).release();
}

void skialin_bridge_FontMgr_unref(SkFontMgr* mgr) {
    SkSafeUnref(mgr);
}

SkFontMgr* skialin_bridge_FontMgr_RefSystem(void) {
#if defined(SK_BUILD_FOR_WIN)
    return SkFontMgr_New_DirectWrite().release();
#elif defined(SK_BUILD_FOR_MAC)
    return SkFontMgr_New_CoreText(nullptr).release();
#elif defined(SK_BUILD_FOR_UNIX)
    return SkFontMgr_New_FontConfig(nullptr, SkFontScanner_Make_FreeType()).release();
#endif
}

SkFontMgr* skialin_bridge_FontMgr_RefEmpty(void) {
    return SkFontMgr::RefEmpty().release();
}

int32_t skialin_bridge_FontMgr_countFamilies(const SkFontMgr* mgr) {
    return mgr->countFamilies();
}

SkData* skialin_bridge_FontMgr_familyName(const SkFontMgr* mgr, int32_t index) {
    SkString name;
    mgr->getFamilyName(index, &name);
    return SkData::MakeWithCopy(name.c_str(), name.size()).release();
}

SkTypeface* skialin_bridge_FontMgr_matchFamilyStyle(const SkFontMgr* mgr, const char* familyName, int32_t weight, int32_t width, int32_t slant) {
    SkFontStyle style(weight, width, static_cast<SkFontStyle::Slant>(slant));
    return mgr->matchFamilyStyle(familyName, style).release();
}

SkTypeface* skialin_bridge_FontMgr_makeFromData(const SkFontMgr* mgr, SkData* data, int32_t ttcIndex) {
    return mgr->makeFromData(sk_ref_sp(data), ttcIndex).release();
}

SkTypeface* skialin_bridge_FontMgr_makeFromFile(const SkFontMgr* mgr, const char* path, int32_t ttcIndex) {
    return mgr->makeFromFile(path, ttcIndex).release();
}

SkFont* skialin_bridge_Font_MakeDefault(void) {
    return new SkFont();
}

SkFont* skialin_bridge_Font_MakeWithTypeface(SkTypeface* typeface, float size) {
    return new SkFont(sk_ref_sp(typeface), size);
}

void skialin_bridge_Font_delete(SkFont* font) {
    delete font;
}

SkTypeface* skialin_bridge_Font_refTypeface(const SkFont* font) {
    return font->refTypeface().release();
}

void skialin_bridge_Font_setTypeface(SkFont* font, SkTypeface* typeface) {
    font->setTypeface(sk_ref_sp(typeface));
}

SkPath* skialin_bridge_Font_getPath(const SkFont* font, uint16_t glyphID) {
    auto path = font->getPath(glyphID);
    if (!path) {
        return nullptr;
    }
    return new SkPath(*path);
}

SkFont* skialin_bridge_Font_makeWithSize(const SkFont* font, float size) {
    return new SkFont(font->makeWithSize(size));
}

void skialin_bridge_TextBlob_unref(SkTextBlob* blob) {
    SkSafeUnref(blob);
}

SkTextBlob* skialin_bridge_TextBlob_MakeFromText(const void* text, size_t byteLength, const SkFont* font, int32_t encoding) {
    return SkTextBlob::MakeFromText(text, byteLength, *font, static_cast<SkTextEncoding>(encoding)).release();
}

SkTextBlob* skialin_bridge_TextBlob_MakeFromPosTextH(const void* text, size_t byteLength, const float* xpos, size_t xposLength, float constY, const SkFont* font, int32_t encoding) {
    return SkTextBlob::MakeFromPosTextH(text, byteLength, {xpos, xposLength}, constY, *font, static_cast<SkTextEncoding>(encoding)).release();
}

SkTextBlob* skialin_bridge_TextBlob_MakeFromPosText(const void* text, size_t byteLength, const SkPoint* pos, size_t posLength, const SkFont* font, int32_t encoding) {
    return SkTextBlob::MakeFromPosText(text, byteLength, {pos, posLength}, *font, static_cast<SkTextEncoding>(encoding)).release();
}

SkTextBlob* skialin_bridge_TextBlob_MakeFromRSXform(const void* text, size_t byteLength, const SkRSXform* xform, size_t xformLength, const SkFont* font, int32_t encoding) {
    return SkTextBlob::MakeFromRSXform(text, byteLength, {xform, xformLength}, *font, static_cast<SkTextEncoding>(encoding)).release();
}

SkData* skialin_bridge_TextBlob_serialize(const SkTextBlob* blob) {
    SkSerialProcs procs;
    return blob->serialize(procs).release();
}

SkTextBlob* skialin_bridge_TextBlob_Deserialize(const void* data, size_t size) {
    SkDeserialProcs procs;
    return SkTextBlob::Deserialize(data, size, procs).release();
}

using skia::textlayout::TextStyle;
using skia::textlayout::ParagraphStyle;

skia::textlayout::TextStyle* skialin_bridge_TextStyle_new(void) {
    return new TextStyle();
}

skia::textlayout::TextStyle* skialin_bridge_TextStyle_clone(const skia::textlayout::TextStyle* style) {
    return new TextStyle(*style);
}

void skialin_bridge_TextStyle_delete(skia::textlayout::TextStyle* style) {
    delete style;
}

uint32_t skialin_bridge_TextStyle_getColor(const skia::textlayout::TextStyle* style) {
    return style->getColor();
}

void skialin_bridge_TextStyle_setColor(skia::textlayout::TextStyle* style, uint32_t color) {
    style->setColor(color);
}

void skialin_bridge_TextStyle_setFontFamilies(skia::textlayout::TextStyle* style, const char* const* families, const size_t* lengths, size_t count) {
    std::vector<SkString> result;
    result.reserve(count);
    for (size_t i = 0; i < count; ++i) {
        result.emplace_back(families[i], lengths[i]);
    }
    style->setFontFamilies(std::move(result));
}

size_t skialin_bridge_TextStyle_countFontFamilies(const skia::textlayout::TextStyle* style) {
    return style->getFontFamilies().size();
}

SkData* skialin_bridge_TextStyle_fontFamily(const skia::textlayout::TextStyle* style, size_t index) {
    const SkString& name = style->getFontFamilies()[index];
    return SkData::MakeWithCopy(name.c_str(), name.size()).release();
}

float skialin_bridge_TextStyle_getFontSize(const skia::textlayout::TextStyle* style) {
    return style->getFontSize();
}

void skialin_bridge_TextStyle_setFontSize(skia::textlayout::TextStyle* style, float size) {
    style->setFontSize(size);
}

void skialin_bridge_TextStyle_getFontStyle(const skia::textlayout::TextStyle* style, int32_t* weight, int32_t* width, int32_t* slant) {
    SkFontStyle fontStyle = style->getFontStyle();
    *weight = fontStyle.weight();
    *width = fontStyle.width();
    *slant = fontStyle.slant();
}

void skialin_bridge_TextStyle_setFontStyle(skia::textlayout::TextStyle* style, int32_t weight, int32_t width, int32_t slant) {
    style->setFontStyle(SkFontStyle(weight, width, static_cast<SkFontStyle::Slant>(slant)));
}

void skialin_bridge_TextStyle_getDecoration(const skia::textlayout::TextStyle* style, int32_t* type, int32_t* mode, uint32_t* color, int32_t* decorationStyle, float* thicknessMultiplier) {
    skia::textlayout::Decoration decoration = style->getDecoration();
    *type = decoration.fType;
    *mode = decoration.fMode;
    *color = decoration.fColor;
    *decorationStyle = decoration.fStyle;
    *thicknessMultiplier = decoration.fThicknessMultiplier;
}

void skialin_bridge_TextStyle_setDecoration(skia::textlayout::TextStyle* style, int32_t type) {
    style->setDecoration(static_cast<skia::textlayout::TextDecoration>(type));
}

void skialin_bridge_TextStyle_setDecorationMode(skia::textlayout::TextStyle* style, int32_t mode) {
    style->setDecorationMode(static_cast<skia::textlayout::TextDecorationMode>(mode));
}

void skialin_bridge_TextStyle_setDecorationColor(skia::textlayout::TextStyle* style, uint32_t color) {
    style->setDecorationColor(color);
}

void skialin_bridge_TextStyle_setDecorationStyle(skia::textlayout::TextStyle* style, int32_t decorationStyle) {
    style->setDecorationStyle(static_cast<skia::textlayout::TextDecorationStyle>(decorationStyle));
}

void skialin_bridge_TextStyle_setDecorationThicknessMultiplier(skia::textlayout::TextStyle* style, float multiplier) {
    style->setDecorationThicknessMultiplier(multiplier);
}

float skialin_bridge_TextStyle_getLetterSpacing(const skia::textlayout::TextStyle* style) {
    return style->getLetterSpacing();
}

void skialin_bridge_TextStyle_setLetterSpacing(skia::textlayout::TextStyle* style, float letterSpacing) {
    style->setLetterSpacing(letterSpacing);
}

float skialin_bridge_TextStyle_getWordSpacing(const skia::textlayout::TextStyle* style) {
    return style->getWordSpacing();
}

void skialin_bridge_TextStyle_setWordSpacing(skia::textlayout::TextStyle* style, float wordSpacing) {
    style->setWordSpacing(wordSpacing);
}

float skialin_bridge_TextStyle_getHeight(const skia::textlayout::TextStyle* style) {
    return style->getHeight();
}

void skialin_bridge_TextStyle_setHeight(skia::textlayout::TextStyle* style, float height) {
    style->setHeight(height);
}

bool skialin_bridge_TextStyle_getHeightOverride(const skia::textlayout::TextStyle* style) {
    return style->getHeightOverride();
}

void skialin_bridge_TextStyle_setHeightOverride(skia::textlayout::TextStyle* style, bool heightOverride) {
    style->setHeightOverride(heightOverride);
}

SkTypeface* skialin_bridge_TextStyle_refTypeface(const skia::textlayout::TextStyle* style) {
    return style->refTypeface().release();
}

void skialin_bridge_TextStyle_setTypeface(skia::textlayout::TextStyle* style, SkTypeface* typeface) {
    style->setTypeface(sk_ref_sp(typeface));
}

SkData* skialin_bridge_TextStyle_getLocale(const skia::textlayout::TextStyle* style) {
    SkString locale = style->getLocale();
    return SkData::MakeWithCopy(locale.c_str(), locale.size()).release();
}

void skialin_bridge_TextStyle_setLocale(skia::textlayout::TextStyle* style, const char* locale, size_t length) {
    style->setLocale(SkString(locale, length));
}

skia::textlayout::ParagraphStyle* skialin_bridge_ParagraphStyle_new(void) {
    return new ParagraphStyle();
}

void skialin_bridge_ParagraphStyle_delete(skia::textlayout::ParagraphStyle* style) {
    delete style;
}

int32_t skialin_bridge_ParagraphStyle_getTextDirection(const skia::textlayout::ParagraphStyle* style) {
    return static_cast<int32_t>(style->getTextDirection());
}

void skialin_bridge_ParagraphStyle_setTextDirection(skia::textlayout::ParagraphStyle* style, int32_t direction) {
    style->setTextDirection(static_cast<skia::textlayout::TextDirection>(direction));
}

int32_t skialin_bridge_ParagraphStyle_getTextAlign(const skia::textlayout::ParagraphStyle* style) {
    return static_cast<int32_t>(style->getTextAlign());
}

void skialin_bridge_ParagraphStyle_setTextAlign(skia::textlayout::ParagraphStyle* style, int32_t align) {
    style->setTextAlign(static_cast<skia::textlayout::TextAlign>(align));
}

size_t skialin_bridge_ParagraphStyle_getMaxLines(const skia::textlayout::ParagraphStyle* style) {
    return style->getMaxLines();
}

void skialin_bridge_ParagraphStyle_setMaxLines(skia::textlayout::ParagraphStyle* style, size_t maxLines) {
    style->setMaxLines(maxLines);
}

SkData* skialin_bridge_ParagraphStyle_getEllipsis(const skia::textlayout::ParagraphStyle* style) {
    SkString ellipsis = style->getEllipsis();
    return SkData::MakeWithCopy(ellipsis.c_str(), ellipsis.size()).release();
}

void skialin_bridge_ParagraphStyle_setEllipsis(skia::textlayout::ParagraphStyle* style, const char* ellipsis, size_t length) {
    style->setEllipsis(SkString(ellipsis, length));
}

float skialin_bridge_ParagraphStyle_getHeight(const skia::textlayout::ParagraphStyle* style) {
    return style->getHeight();
}

void skialin_bridge_ParagraphStyle_setHeight(skia::textlayout::ParagraphStyle* style, float height) {
    style->setHeight(height);
}

int32_t skialin_bridge_ParagraphStyle_getTextHeightBehavior(const skia::textlayout::ParagraphStyle* style) {
    return static_cast<int32_t>(style->getTextHeightBehavior());
}

void skialin_bridge_ParagraphStyle_setTextHeightBehavior(skia::textlayout::ParagraphStyle* style, int32_t behavior) {
    style->setTextHeightBehavior(static_cast<skia::textlayout::TextHeightBehavior>(behavior));
}

skia::textlayout::TextStyle* skialin_bridge_ParagraphStyle_getTextStyle(const skia::textlayout::ParagraphStyle* style) {
    return new TextStyle(style->getTextStyle());
}

void skialin_bridge_ParagraphStyle_setTextStyle(skia::textlayout::ParagraphStyle* paragraphStyle, const skia::textlayout::TextStyle* style) {
    paragraphStyle->setTextStyle(*style);
}

using skia::textlayout::StrutStyle;

skia::textlayout::StrutStyle* skialin_bridge_StrutStyle_new(void) {
    return new StrutStyle();
}

void skialin_bridge_StrutStyle_delete(skia::textlayout::StrutStyle* style) {
    delete style;
}

SkData* skialin_bridge_StrutStyle_fontFamily(const skia::textlayout::StrutStyle* style, size_t index) {
    const SkString& name = style->getFontFamilies()[index];
    return SkData::MakeWithCopy(name.c_str(), name.size()).release();
}

size_t skialin_bridge_StrutStyle_countFontFamilies(const skia::textlayout::StrutStyle* style) {
    return style->getFontFamilies().size();
}

void skialin_bridge_StrutStyle_setFontFamilies(skia::textlayout::StrutStyle* style, const char* const* families, const size_t* lengths, size_t count) {
    std::vector<SkString> result;
    result.reserve(count);
    for (size_t i = 0; i < count; ++i) {
        result.emplace_back(families[i], lengths[i]);
    }
    style->setFontFamilies(std::move(result));
}

void skialin_bridge_StrutStyle_getFontStyle(const skia::textlayout::StrutStyle* style, int32_t* weight, int32_t* width, int32_t* slant) {
    SkFontStyle fontStyle = style->getFontStyle();
    *weight = fontStyle.weight();
    *width = fontStyle.width();
    *slant = fontStyle.slant();
}

void skialin_bridge_StrutStyle_setFontStyle(skia::textlayout::StrutStyle* style, int32_t weight, int32_t width, int32_t slant) {
    style->setFontStyle(SkFontStyle(weight, width, static_cast<SkFontStyle::Slant>(slant)));
}

float skialin_bridge_StrutStyle_getFontSize(const skia::textlayout::StrutStyle* style) {
    return style->getFontSize();
}

void skialin_bridge_StrutStyle_setFontSize(skia::textlayout::StrutStyle* style, float size) {
    style->setFontSize(size);
}

float skialin_bridge_StrutStyle_getHeight(const skia::textlayout::StrutStyle* style) {
    return style->getHeight();
}

void skialin_bridge_StrutStyle_setHeight(skia::textlayout::StrutStyle* style, float height) {
    style->setHeight(height);
}

float skialin_bridge_StrutStyle_getLeading(const skia::textlayout::StrutStyle* style) {
    return style->getLeading();
}

void skialin_bridge_StrutStyle_setLeading(skia::textlayout::StrutStyle* style, float leading) {
    style->setLeading(leading);
}

bool skialin_bridge_StrutStyle_getStrutEnabled(const skia::textlayout::StrutStyle* style) {
    return style->getStrutEnabled();
}

void skialin_bridge_StrutStyle_setStrutEnabled(skia::textlayout::StrutStyle* style, bool enabled) {
    style->setStrutEnabled(enabled);
}

bool skialin_bridge_StrutStyle_getForceStrutHeight(const skia::textlayout::StrutStyle* style) {
    return style->getForceStrutHeight();
}

void skialin_bridge_StrutStyle_setForceStrutHeight(skia::textlayout::StrutStyle* style, bool force) {
    style->setForceStrutHeight(force);
}

bool skialin_bridge_StrutStyle_getHeightOverride(const skia::textlayout::StrutStyle* style) {
    return style->getHeightOverride();
}

void skialin_bridge_StrutStyle_setHeightOverride(skia::textlayout::StrutStyle* style, bool heightOverride) {
    style->setHeightOverride(heightOverride);
}

bool skialin_bridge_StrutStyle_getHalfLeading(const skia::textlayout::StrutStyle* style) {
    return style->getHalfLeading();
}

void skialin_bridge_StrutStyle_setHalfLeading(skia::textlayout::StrutStyle* style, bool halfLeading) {
    style->setHalfLeading(halfLeading);
}

skia::textlayout::StrutStyle* skialin_bridge_ParagraphStyle_getStrutStyle(const skia::textlayout::ParagraphStyle* style) {
    return new StrutStyle(style->getStrutStyle());
}

void skialin_bridge_ParagraphStyle_setStrutStyle(skia::textlayout::ParagraphStyle* paragraphStyle, const skia::textlayout::StrutStyle* strutStyle) {
    paragraphStyle->setStrutStyle(*strutStyle);
}

using skia::textlayout::FontCollection;
using skia::textlayout::ParagraphBuilder;
using skia::textlayout::Paragraph;
using skia::textlayout::LineMetrics;

skia::textlayout::FontCollection* skialin_bridge_FontCollection_new(void) {
    return new FontCollection();
}

void skialin_bridge_FontCollection_unref(skia::textlayout::FontCollection* collection) {
    SkSafeUnref(collection);
}

void skialin_bridge_FontCollection_setDefaultFontManager(skia::textlayout::FontCollection* collection, SkFontMgr* fontManager) {
    collection->setDefaultFontManager(sk_ref_sp(fontManager));
}

skia::textlayout::ParagraphBuilder* skialin_bridge_ParagraphBuilder_make(const skia::textlayout::ParagraphStyle* style, skia::textlayout::FontCollection* fontCollection) {
    return ParagraphBuilder::make(*style, sk_ref_sp(fontCollection), SkUnicodes::ICU::Make()).release();
}

void skialin_bridge_ParagraphBuilder_delete(skia::textlayout::ParagraphBuilder* builder) {
    delete builder;
}

void skialin_bridge_ParagraphBuilder_pushStyle(skia::textlayout::ParagraphBuilder* builder, const skia::textlayout::TextStyle* style) {
    builder->pushStyle(*style);
}

void skialin_bridge_ParagraphBuilder_pop(skia::textlayout::ParagraphBuilder* builder) {
    builder->pop();
}

void skialin_bridge_ParagraphBuilder_addText(skia::textlayout::ParagraphBuilder* builder, const char* text, size_t length) {
    builder->addText(text, length);
}

void skialin_bridge_ParagraphBuilder_addPlaceholder(
    skia::textlayout::ParagraphBuilder* builder, float width, float height, int32_t alignment, int32_t baseline, float baselineOffset) {
    skia::textlayout::PlaceholderStyle style(
        width, height,
        static_cast<skia::textlayout::PlaceholderAlignment>(alignment),
        static_cast<skia::textlayout::TextBaseline>(baseline),
        baselineOffset);
    builder->addPlaceholder(style);
}

skia::textlayout::Paragraph* skialin_bridge_ParagraphBuilder_build(skia::textlayout::ParagraphBuilder* builder) {
    return builder->Build().release();
}

void skialin_bridge_Paragraph_delete(skia::textlayout::Paragraph* paragraph) {
    delete paragraph;
}

void skialin_bridge_Paragraph_layout(skia::textlayout::Paragraph* paragraph, float width) {
    paragraph->layout(width);
}

void skialin_bridge_Paragraph_paint(skia::textlayout::Paragraph* paragraph, SkCanvas* canvas, float x, float y) {
    paragraph->paint(canvas, x, y);
}

float skialin_bridge_Paragraph_getMaxWidth(const skia::textlayout::Paragraph* paragraph) {
    return const_cast<Paragraph*>(paragraph)->getMaxWidth();
}

float skialin_bridge_Paragraph_getHeight(const skia::textlayout::Paragraph* paragraph) {
    return const_cast<Paragraph*>(paragraph)->getHeight();
}

float skialin_bridge_Paragraph_getMinIntrinsicWidth(const skia::textlayout::Paragraph* paragraph) {
    return const_cast<Paragraph*>(paragraph)->getMinIntrinsicWidth();
}

float skialin_bridge_Paragraph_getMaxIntrinsicWidth(const skia::textlayout::Paragraph* paragraph) {
    return const_cast<Paragraph*>(paragraph)->getMaxIntrinsicWidth();
}

float skialin_bridge_Paragraph_getAlphabeticBaseline(const skia::textlayout::Paragraph* paragraph) {
    return const_cast<Paragraph*>(paragraph)->getAlphabeticBaseline();
}

float skialin_bridge_Paragraph_getIdeographicBaseline(const skia::textlayout::Paragraph* paragraph) {
    return const_cast<Paragraph*>(paragraph)->getIdeographicBaseline();
}

float skialin_bridge_Paragraph_getLongestLine(const skia::textlayout::Paragraph* paragraph) {
    return const_cast<Paragraph*>(paragraph)->getLongestLine();
}

bool skialin_bridge_Paragraph_didExceedMaxLines(const skia::textlayout::Paragraph* paragraph) {
    return const_cast<Paragraph*>(paragraph)->didExceedMaxLines();
}

size_t skialin_bridge_Paragraph_lineNumber(skia::textlayout::Paragraph* paragraph) {
    return paragraph->lineNumber();
}

int32_t skialin_bridge_Paragraph_unresolvedGlyphs(skia::textlayout::Paragraph* paragraph) {
    return paragraph->unresolvedGlyphs();
}

int32_t skialin_bridge_Paragraph_getGlyphPositionAtCoordinate(skia::textlayout::Paragraph* paragraph, float dx, float dy, int32_t* affinity) {
    skia::textlayout::PositionWithAffinity result = paragraph->getGlyphPositionAtCoordinate(dx, dy);
    *affinity = result.affinity == skia::textlayout::kUpstream ? 0 : 1;
    return result.position;
}

void skialin_bridge_Paragraph_getWordBoundary(skia::textlayout::Paragraph* paragraph, uint32_t offset, size_t* start, size_t* end) {
    skia::textlayout::SkRange<size_t> range = paragraph->getWordBoundary(offset);
    *start = range.start;
    *end = range.end;
}

bool skialin_bridge_Paragraph_getLineMetricsAt(
    skia::textlayout::Paragraph* paragraph, int32_t lineNumber,
    size_t* startIndex, size_t* endIndex, size_t* endExcludingWhitespaces, size_t* endIncludingNewline, int32_t* hardBreak,
    double* ascent, double* descent, double* unscaledAscent, double* height, double* width, double* left, double* baseline) {
    LineMetrics metrics;
    if (!paragraph->getLineMetricsAt(lineNumber, &metrics)) {
        return false;
    }
    *startIndex = metrics.fStartIndex;
    *endIndex = metrics.fEndIndex;
    *endExcludingWhitespaces = metrics.fEndExcludingWhitespaces;
    *endIncludingNewline = metrics.fEndIncludingNewline;
    *hardBreak = metrics.fHardBreak ? 1 : 0;
    *ascent = metrics.fAscent;
    *descent = metrics.fDescent;
    *unscaledAscent = metrics.fUnscaledAscent;
    *height = metrics.fHeight;
    *width = metrics.fWidth;
    *left = metrics.fLeft;
    *baseline = metrics.fBaseline;
    return true;
}

int32_t skialin_bridge_Paragraph_getRectsForRange(
    skia::textlayout::Paragraph* paragraph, uint32_t start, uint32_t end, int32_t rectHeightStyle, int32_t rectWidthStyle, float* outBuf, int32_t capacity) {
    std::vector<skia::textlayout::TextBox> boxes = paragraph->getRectsForRange(
        start, end, static_cast<skia::textlayout::RectHeightStyle>(rectHeightStyle), static_cast<skia::textlayout::RectWidthStyle>(rectWidthStyle));
    int32_t count = static_cast<int32_t>(boxes.size());
    int32_t toWrite = count < capacity ? count : capacity;
    for (int32_t i = 0; i < toWrite; i++) {
        const skia::textlayout::TextBox& box = boxes[i];
        outBuf[i * 5 + 0] = box.rect.fLeft;
        outBuf[i * 5 + 1] = box.rect.fTop;
        outBuf[i * 5 + 2] = box.rect.fRight;
        outBuf[i * 5 + 3] = box.rect.fBottom;
        outBuf[i * 5 + 4] = box.direction == skia::textlayout::TextDirection::kLtr ? 1.0f : 0.0f;
    }
    return count;
}

void skialin_bridge_ColorFilter_unref(SkColorFilter* filter) {
    SkSafeUnref(filter);
}

SkColorFilter* skialin_bridge_ColorFilter_Blend(uint32_t argb, SkBlendMode mode) {
    return SkColorFilters::Blend(static_cast<SkColor>(argb), mode).release();
}

SkColorFilter* skialin_bridge_ColorFilter_Matrix(const float* rowMajor20, bool clamp) {
    return SkColorFilters::Matrix(rowMajor20, clamp ? SkColorFilters::Clamp::kYes : SkColorFilters::Clamp::kNo).release();
}

SkColorFilter* skialin_bridge_ColorFilter_Compose(SkColorFilter* outer, SkColorFilter* inner) {
    return SkColorFilters::Compose(sk_ref_sp(outer), sk_ref_sp(inner)).release();
}

SkColorFilter* skialin_bridge_ColorFilter_Lerp(float t, SkColorFilter* dst, SkColorFilter* src) {
    return SkColorFilters::Lerp(t, sk_ref_sp(dst), sk_ref_sp(src)).release();
}

SkColorFilter* skialin_bridge_ColorFilter_HSLAMatrix(const float* rowMajor20) {
    return SkColorFilters::HSLAMatrix(rowMajor20).release();
}

SkColorFilter* skialin_bridge_ColorFilter_LinearToSRGBGamma(void) {
    return SkColorFilters::LinearToSRGBGamma().release();
}

SkColorFilter* skialin_bridge_ColorFilter_SRGBToLinearGamma(void) {
    return SkColorFilters::SRGBToLinearGamma().release();
}

SkColorFilter* skialin_bridge_ColorFilter_Table(const uint8_t* table256) {
    return SkColorFilters::Table(table256).release();
}

SkColorFilter* skialin_bridge_ColorFilter_TableARGB(const uint8_t* tableA256, const uint8_t* tableR256, const uint8_t* tableG256, const uint8_t* tableB256) {
    return SkColorFilters::TableARGB(tableA256, tableR256, tableG256, tableB256).release();
}

SkColorFilter* skialin_bridge_ColorFilter_Lighting(uint32_t mul, uint32_t add) {
    return SkColorFilters::Lighting(static_cast<SkColor>(mul), static_cast<SkColor>(add)).release();
}

SkColorFilter* skialin_bridge_ColorFilter_HighContrast(bool grayscale, int32_t invertStyle, float contrast) {
    SkHighContrastConfig config(grayscale, static_cast<SkHighContrastConfig::InvertStyle>(invertStyle), contrast);
    return SkHighContrastFilter::Make(config).release();
}

SkColorFilter* skialin_bridge_ColorFilter_Luma(void) {
    return SkLumaColorFilter::Make().release();
}

void skialin_bridge_ColorMatrix_setIdentity(float* mat20) {
    reinterpret_cast<SkColorMatrix*>(mat20)->setIdentity();
}

void skialin_bridge_ColorMatrix_setScale(float* mat20, float rScale, float gScale, float bScale, float aScale) {
    reinterpret_cast<SkColorMatrix*>(mat20)->setScale(rScale, gScale, bScale, aScale);
}

void skialin_bridge_ColorMatrix_postTranslate(float* mat20, float dr, float dg, float db, float da) {
    reinterpret_cast<SkColorMatrix*>(mat20)->postTranslate(dr, dg, db, da);
}

void skialin_bridge_ColorMatrix_setConcat(float* outMat20, const float* aMat20, const float* bMat20) {
    reinterpret_cast<SkColorMatrix*>(outMat20)->setConcat(
        *reinterpret_cast<const SkColorMatrix*>(aMat20), *reinterpret_cast<const SkColorMatrix*>(bMat20));
}

void skialin_bridge_ColorMatrix_setSaturation(float* mat20, float sat) {
    reinterpret_cast<SkColorMatrix*>(mat20)->setSaturation(sat);
}

void skialin_bridge_ImageFilter_unref(SkImageFilter* filter) {
    SkSafeUnref(filter);
}

SkImageFilter* skialin_bridge_ImageFilter_Blur(float sigmaX, float sigmaY, SkTileMode tileMode, SkImageFilter* input) {
    return SkImageFilters::Blur(sigmaX, sigmaY, tileMode, sk_ref_sp(input)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_DropShadow(float dx, float dy, float sigmaX, float sigmaY, uint32_t color, SkImageFilter* input) {
    return SkImageFilters::DropShadow(dx, dy, sigmaX, sigmaY, static_cast<SkColor>(color), sk_ref_sp(input)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_DropShadowOnly(float dx, float dy, float sigmaX, float sigmaY, uint32_t color, SkImageFilter* input) {
    return SkImageFilters::DropShadowOnly(dx, dy, sigmaX, sigmaY, static_cast<SkColor>(color), sk_ref_sp(input)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_Offset(float dx, float dy, SkImageFilter* input) {
    return SkImageFilters::Offset(dx, dy, sk_ref_sp(input)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_ColorFilter(SkColorFilter* cf, SkImageFilter* input) {
    return SkImageFilters::ColorFilter(sk_ref_sp(cf), sk_ref_sp(input)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_Compose(SkImageFilter* outer, SkImageFilter* inner) {
    return SkImageFilters::Compose(sk_ref_sp(outer), sk_ref_sp(inner)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_MatrixTransform(
    const SkMatrix* matrix, int32_t maxAniso, bool useCubic, float cubicB, float cubicC, SkFilterMode filter, SkMipmapMode mipmap,
    SkImageFilter* input) {
    SkSamplingOptions sampling = toSamplingOptions(maxAniso, useCubic, cubicB, cubicC, filter, mipmap);
    return SkImageFilters::MatrixTransform(*matrix, sampling, sk_ref_sp(input)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_Dilate(float radiusX, float radiusY, SkImageFilter* input) {
    return SkImageFilters::Dilate(radiusX, radiusY, sk_ref_sp(input)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_Erode(float radiusX, float radiusY, SkImageFilter* input) {
    return SkImageFilters::Erode(radiusX, radiusY, sk_ref_sp(input)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_Blend(SkBlendMode mode, SkImageFilter* background, SkImageFilter* foreground) {
    return SkImageFilters::Blend(mode, sk_ref_sp(background), sk_ref_sp(foreground)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_BlendBlender(SkBlender* blender, SkImageFilter* background, SkImageFilter* foreground) {
    return SkImageFilters::Blend(sk_ref_sp(blender), sk_ref_sp(background), sk_ref_sp(foreground)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_Merge(SkImageFilter* first, SkImageFilter* second) {
    return SkImageFilters::Merge(sk_ref_sp(first), sk_ref_sp(second)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_Shader(SkShader* shader) {
    return SkImageFilters::Shader(sk_ref_sp(shader)).release();
}

SkImageFilter* skialin_bridge_ImageFilter_Tile(const SkRect* src, const SkRect* dst, SkImageFilter* input) {
    return SkImageFilters::Tile(*src, *dst, sk_ref_sp(input)).release();
}

void skialin_bridge_MaskFilter_unref(SkMaskFilter* filter) {
    SkSafeUnref(filter);
}

SkMaskFilter* skialin_bridge_MaskFilter_MakeBlur(int32_t style, float sigma, bool respectCTM) {
    return SkMaskFilter::MakeBlur(static_cast<SkBlurStyle>(style), sigma, respectCTM).release();
}

void skialin_bridge_Paint_setColorFilter(SkPaint* paint, SkColorFilter* filter) {
    paint->setColorFilter(sk_ref_sp(filter));
}

void skialin_bridge_Paint_setImageFilter(SkPaint* paint, SkImageFilter* filter) {
    paint->setImageFilter(sk_ref_sp(filter));
}

void skialin_bridge_Paint_setMaskFilter(SkPaint* paint, SkMaskFilter* filter) {
    paint->setMaskFilter(sk_ref_sp(filter));
}

void skialin_bridge_Paint_setBlender(SkPaint* paint, SkBlender* blender) {
    paint->setBlender(sk_ref_sp(blender));
}

SkShader* skialin_bridge_Paint_refShader(const SkPaint* paint) {
    return paint->refShader().release();
}

SkColorFilter* skialin_bridge_Paint_refColorFilter(const SkPaint* paint) {
    return paint->refColorFilter().release();
}

SkImageFilter* skialin_bridge_Paint_refImageFilter(const SkPaint* paint) {
    return paint->refImageFilter().release();
}

SkMaskFilter* skialin_bridge_Paint_refMaskFilter(const SkPaint* paint) {
    return paint->refMaskFilter().release();
}

SkBlender* skialin_bridge_Paint_refBlender(const SkPaint* paint) {
    return paint->refBlender().release();
}

SkPathEffect* skialin_bridge_Paint_refPathEffect(const SkPaint* paint) {
    return paint->refPathEffect().release();
}

namespace {
void radiiFromFlat(const float* radii8, SkVector out[4]) {
    for (int i = 0; i < 4; ++i) {
        out[i] = SkVector::Make(radii8[i * 2], radii8[i * 2 + 1]);
    }
}
}  // namespace

SkRRect* skialin_bridge_RRect_MakeRect(const SkRect* rect) {
    return new SkRRect(SkRRect::MakeRect(*rect));
}

SkRRect* skialin_bridge_RRect_MakeOval(const SkRect* oval) {
    return new SkRRect(SkRRect::MakeOval(*oval));
}

SkRRect* skialin_bridge_RRect_MakeRectXY(const SkRect* rect, float xRad, float yRad) {
    return new SkRRect(SkRRect::MakeRectXY(*rect, xRad, yRad));
}

SkRRect* skialin_bridge_RRect_MakeRectRadii(const SkRect* rect, const float* radii8) {
    SkVector radii[4];
    radiiFromFlat(radii8, radii);
    return new SkRRect(SkRRect::MakeRectRadii(*rect, radii));
}

void skialin_bridge_RRect_delete(SkRRect* rrect) {
    delete rrect;
}

SkRRect* skialin_bridge_RRect_clone(const SkRRect* rrect) {
    return new SkRRect(*rrect);
}

void skialin_bridge_RRect_rect(const SkRRect* rrect, SkRect* outRect) {
    *outRect = rrect->rect();
}

void skialin_bridge_RRect_radii(const SkRRect* rrect, float* outRadii8) {
    for (int i = 0; i < 4; ++i) {
        SkVector v = rrect->radii(static_cast<SkRRect::Corner>(i));
        outRadii8[i * 2] = v.x();
        outRadii8[i * 2 + 1] = v.y();
    }
}

int32_t skialin_bridge_RRect_type(const SkRRect* rrect) {
    return static_cast<int32_t>(rrect->getType());
}

bool skialin_bridge_RRect_containsPoint(const SkRRect* rrect, SkPoint point) {
    return rrect->contains(point);
}

bool skialin_bridge_RRect_containsRect(const SkRRect* rrect, const SkRect* rect) {
    return rrect->contains(*rect);
}

bool skialin_bridge_RRect_isValid(const SkRRect* rrect) {
    return rrect->isValid();
}

SkRRect* skialin_bridge_RRect_inset(const SkRRect* rrect, float dx, float dy) {
    SkRRect* result = new SkRRect();
    rrect->inset(dx, dy, result);
    return result;
}

SkRRect* skialin_bridge_RRect_outset(const SkRRect* rrect, float dx, float dy) {
    SkRRect* result = new SkRRect();
    rrect->outset(dx, dy, result);
    return result;
}

SkRRect* skialin_bridge_RRect_transform(const SkRRect* rrect, const SkMatrix* matrix) {
    SkRRect result;
    if (!rrect->transform(*matrix, &result)) {
        return nullptr;
    }
    return new SkRRect(result);
}

void skialin_bridge_Canvas_drawRRect(SkCanvas* canvas, const SkRRect* rrect, const SkPaint* paint) {
    canvas->drawRRect(*rrect, *paint);
}

void skialin_bridge_Canvas_drawDRRect(SkCanvas* canvas, const SkRRect* outer, const SkRRect* inner, const SkPaint* paint) {
    canvas->drawDRRect(*outer, *inner, *paint);
}

SkRegion* skialin_bridge_Region_MakeEmpty(void) {
    return new SkRegion();
}

SkRegion* skialin_bridge_Region_MakeRect(const SkIRect* rect) {
    return new SkRegion(*rect);
}

void skialin_bridge_Region_delete(SkRegion* region) {
    delete region;
}

SkRegion* skialin_bridge_Region_clone(const SkRegion* region) {
    return new SkRegion(*region);
}

bool skialin_bridge_Region_setRect(SkRegion* region, const SkIRect* rect) {
    return region->setRect(*rect);
}

bool skialin_bridge_Region_setPath(SkRegion* region, const SkPath* path, const SkRegion* clip) {
    return region->setPath(*path, *clip);
}

bool skialin_bridge_Region_opRegion(SkRegion* region, const SkRegion* other, int32_t op) {
    return region->op(*other, static_cast<SkRegion::Op>(op));
}

bool skialin_bridge_Region_opRect(SkRegion* region, const SkIRect* rect, int32_t op) {
    return region->op(*rect, static_cast<SkRegion::Op>(op));
}

bool skialin_bridge_Region_isEmpty(const SkRegion* region) {
    return region->isEmpty();
}

bool skialin_bridge_Region_isRect(const SkRegion* region) {
    return region->isRect();
}

bool skialin_bridge_Region_isComplex(const SkRegion* region) {
    return region->isComplex();
}

void skialin_bridge_Region_getBounds(const SkRegion* region, SkIRect* outRect) {
    *outRect = region->getBounds();
}

bool skialin_bridge_Region_containsPoint(const SkRegion* region, int32_t x, int32_t y) {
    return region->contains(x, y);
}

bool skialin_bridge_Region_containsRect(const SkRegion* region, const SkIRect* rect) {
    return region->contains(*rect);
}

bool skialin_bridge_Region_containsRegion(const SkRegion* region, const SkRegion* other) {
    return region->contains(*other);
}

bool skialin_bridge_Region_intersectsRect(const SkRegion* region, const SkIRect* rect) {
    return region->intersects(*rect);
}

bool skialin_bridge_Region_intersectsRegion(const SkRegion* region, const SkRegion* other) {
    return region->intersects(*other);
}

bool skialin_bridge_Region_equals(const SkRegion* a, const SkRegion* b) {
    return *a == *b;
}

SkPath* skialin_bridge_Region_getBoundaryPath(const SkRegion* region) {
    return new SkPath(region->getBoundaryPath());
}

void skialin_bridge_Canvas_clipRRect(SkCanvas* canvas, const SkRRect* rrect, SkClipOp op, bool doAntiAlias) {
    canvas->clipRRect(*rrect, op, doAntiAlias);
}

void skialin_bridge_PathEffect_unref(SkPathEffect* effect) {
    SkSafeUnref(effect);
}

SkPathEffect* skialin_bridge_PathEffect_MakeDash(const float* intervals, size_t count, float phase) {
    return SkDashPathEffect::Make({intervals, count}, phase).release();
}

SkPathEffect* skialin_bridge_PathEffect_MakeCorner(float radius) {
    return SkCornerPathEffect::Make(radius).release();
}

SkPathEffect* skialin_bridge_PathEffect_MakeDiscrete(float segLength, float deviation, uint32_t seedAssist) {
    return SkDiscretePathEffect::Make(segLength, deviation, seedAssist).release();
}

SkPathEffect* skialin_bridge_PathEffect_MakeTrim(float startT, float stopT, int32_t mode) {
    auto trimMode = mode == 1 ? SkTrimPathEffect::Mode::kInverted : SkTrimPathEffect::Mode::kNormal;
    return SkTrimPathEffect::Make(startT, stopT, trimMode).release();
}

SkPathEffect* skialin_bridge_PathEffect_MakeCompose(SkPathEffect* outer, SkPathEffect* inner) {
    return SkPathEffect::MakeCompose(sk_ref_sp(outer), sk_ref_sp(inner)).release();
}

SkPathEffect* skialin_bridge_PathEffect_MakeSum(SkPathEffect* first, SkPathEffect* second) {
    return SkPathEffect::MakeSum(sk_ref_sp(first), sk_ref_sp(second)).release();
}

SkPathEffect* skialin_bridge_PathEffect_MakePath1D(const SkPath* path, float advance, float phase, int32_t style) {
    return SkPath1DPathEffect::Make(*path, advance, phase, static_cast<SkPath1DPathEffect::Style>(style)).release();
}

SkPathEffect* skialin_bridge_PathEffect_MakePath2D(const SkMatrix* matrix, const SkPath* path) {
    return SkPath2DPathEffect::Make(*matrix, *path).release();
}

SkPathEffect* skialin_bridge_PathEffect_MakeLine2D(float width, const SkMatrix* matrix) {
    return SkLine2DPathEffect::Make(width, *matrix).release();
}

void skialin_bridge_Paint_setPathEffect(SkPaint* paint, SkPathEffect* effect) {
    paint->setPathEffect(sk_ref_sp(effect));
}

SkPathMeasure* skialin_bridge_PathMeasure_new(const SkPath* path, bool forceClosed, float resScale) {
    if (path) {
        return new SkPathMeasure(*path, forceClosed, resScale);
    }
    return new SkPathMeasure();
}

void skialin_bridge_PathMeasure_delete(SkPathMeasure* measure) {
    delete measure;
}

void skialin_bridge_PathMeasure_setPath(SkPathMeasure* measure, const SkPath* path, bool forceClosed) {
    measure->setPath(path, forceClosed);
}

float skialin_bridge_PathMeasure_getLength(SkPathMeasure* measure) {
    return measure->getLength();
}

bool skialin_bridge_PathMeasure_getPosTan(SkPathMeasure* measure, float distance, SkPoint* outPosition, SkPoint* outTangent) {
    return measure->getPosTan(distance, outPosition, outTangent);
}

bool skialin_bridge_PathMeasure_getMatrix(SkPathMeasure* measure, float distance, SkMatrix* outMatrix, int32_t flags) {
    return measure->getMatrix(distance, outMatrix, static_cast<SkPathMeasure::MatrixFlags>(flags));
}

bool skialin_bridge_PathMeasure_getSegment(SkPathMeasure* measure, float startD, float stopD, SkPathBuilder* dst, bool startWithMoveTo) {
    return measure->getSegment(startD, stopD, dst, startWithMoveTo);
}

bool skialin_bridge_PathMeasure_isClosed(SkPathMeasure* measure) {
    return measure->isClosed();
}

bool skialin_bridge_PathMeasure_nextContour(SkPathMeasure* measure) {
    return measure->nextContour();
}

SkPath* skialin_bridge_Path_op(const SkPath* one, const SkPath* two, int32_t op) {
    auto result = Op(*one, *two, static_cast<SkPathOp>(op));
    if (!result) {
        return nullptr;
    }
    return new SkPath(*result);
}

SkPath* skialin_bridge_Path_simplify(const SkPath* path) {
    auto result = Simplify(*path);
    if (!result) {
        return nullptr;
    }
    return new SkPath(*result);
}

void skialin_bridge_Path_computeTightBounds(const SkPath* path, SkRect* outRect) {
    *outRect = path->computeTightBounds();
}

SkM44* skialin_bridge_M44_MakeIdentity(void) {
    return new SkM44();
}

SkM44* skialin_bridge_M44_MakeFromRowMajor(const float* rowMajor16) {
    return new SkM44(SkM44::RowMajor(rowMajor16));
}

SkM44* skialin_bridge_M44_MakeTranslate(float x, float y, float z) {
    return new SkM44(SkM44::Translate(x, y, z));
}

SkM44* skialin_bridge_M44_MakeScale(float x, float y, float z) {
    return new SkM44(SkM44::Scale(x, y, z));
}

SkM44* skialin_bridge_M44_MakeRotate(float axisX, float axisY, float axisZ, float radians) {
    return new SkM44(SkM44::Rotate(SkV3{axisX, axisY, axisZ}, radians));
}

void skialin_bridge_M44_delete(SkM44* m) {
    delete m;
}

SkM44* skialin_bridge_M44_clone(const SkM44* m) {
    return new SkM44(*m);
}

void skialin_bridge_M44_getRowMajor(const SkM44* m, float* outRowMajor16) {
    m->getRowMajor(outRowMajor16);
}

SkM44* skialin_bridge_M44_concat(const SkM44* a, const SkM44* b) {
    return new SkM44(*a, *b);
}

SkM44* skialin_bridge_M44_invert(const SkM44* m) {
    SkM44 inverse;
    if (!m->invert(&inverse)) {
        return nullptr;
    }
    return new SkM44(inverse);
}

void skialin_bridge_M44_mapV4(const SkM44* m, const float* v4, float* outV4) {
    SkV4 result = *m * SkV4{v4[0], v4[1], v4[2], v4[3]};
    outV4[0] = result.x;
    outV4[1] = result.y;
    outV4[2] = result.z;
    outV4[3] = result.w;
}

bool skialin_bridge_M44_equals(const SkM44* a, const SkM44* b) {
    return *a == *b;
}

SkM44* skialin_bridge_M44_transpose(const SkM44* m) {
    return new SkM44(m->transpose());
}

float skialin_bridge_M44_rc(const SkM44* m, int row, int col) {
    return m->rc(row, col);
}

void skialin_bridge_Vertices_unref(SkVertices* vertices) {
    SkSafeUnref(vertices);
}

SkVertices* skialin_bridge_Vertices_MakeCopy(
    int32_t mode, int32_t vertexCount, const SkPoint* positions, const SkPoint* texs, const uint32_t* colors,
    int32_t indexCount, const uint16_t* indices) {
    return SkVertices::MakeCopy(
                   static_cast<SkVertices::VertexMode>(mode), vertexCount, positions, texs,
                   reinterpret_cast<const SkColor*>(colors), indexCount, indices)
        .release();
}

void skialin_bridge_Canvas_drawVertices(SkCanvas* canvas, const SkVertices* vertices, SkBlendMode mode, const SkPaint* paint) {
    canvas->drawVertices(vertices, mode, *paint);
}

void skialin_bridge_Canvas_concat44(SkCanvas* canvas, const SkM44* matrix) {
    canvas->concat(*matrix);
}

SkSurfaceProps* skialin_bridge_SurfaceProps_make(uint32_t flags, SkPixelGeometry pixelGeometry, float textContrast, float textGamma) {
    return new SkSurfaceProps(flags, pixelGeometry, textContrast, textGamma);
}

void skialin_bridge_SurfaceProps_delete(SkSurfaceProps* props) {
    delete props;
}

SkSurfaceProps* skialin_bridge_SurfaceProps_clone(const SkSurfaceProps* props) {
    return new SkSurfaceProps(*props);
}

SkSurfaceProps* skialin_bridge_SurfaceProps_cloneWithPixelGeometry(const SkSurfaceProps* props, SkPixelGeometry pixelGeometry) {
    return new SkSurfaceProps(props->cloneWithPixelGeometry(pixelGeometry));
}

uint32_t skialin_bridge_SurfaceProps_flags(const SkSurfaceProps* props) {
    return props->flags();
}

SkPixelGeometry skialin_bridge_SurfaceProps_pixelGeometry(const SkSurfaceProps* props) {
    return props->pixelGeometry();
}

float skialin_bridge_SurfaceProps_textContrast(const SkSurfaceProps* props) {
    return props->textContrast();
}

float skialin_bridge_SurfaceProps_textGamma(const SkSurfaceProps* props) {
    return props->textGamma();
}

bool skialin_bridge_SurfaceProps_equals(const SkSurfaceProps* a, const SkSurfaceProps* b) {
    return *a == *b;
}

SkSVGDOM* skialin_bridge_SVGDOM_MakeFromStream(const uint8_t* bytes, size_t length) {
    SkMemoryStream stream(bytes, length, false);
    return SkSVGDOM::MakeFromStream(stream).release();
}

void skialin_bridge_SVGDOM_unref(SkSVGDOM* dom) {
    SkSafeUnref(dom);
}

void skialin_bridge_SVGDOM_setContainerSize(SkSVGDOM* dom, float width, float height) {
    dom->setContainerSize(SkSize::Make(width, height));
}

void skialin_bridge_SVGDOM_getContainerSize(const SkSVGDOM* dom, float* outWidth, float* outHeight) {
    const SkSize& size = dom->containerSize();
    *outWidth = size.width();
    *outHeight = size.height();
}

void skialin_bridge_SVGDOM_render(const SkSVGDOM* dom, SkCanvas* canvas) {
    dom->render(canvas);
}

struct SkialinSvgCanvas {
    SkDynamicMemoryWStream stream;
    std::unique_ptr<SkCanvas> canvas;
};

SkialinSvgCanvas* skialin_bridge_SVGCanvas_Make(const SkRect* bounds, uint32_t flags) {
    auto* svgCanvas = new SkialinSvgCanvas();
    svgCanvas->canvas = SkSVGCanvas::Make(*bounds, &svgCanvas->stream, flags);
    return svgCanvas;
}

SkCanvas* skialin_bridge_SVGCanvas_getCanvas(SkialinSvgCanvas* svgCanvas) {
    return svgCanvas->canvas.get();
}

SkData* skialin_bridge_SVGCanvas_finish(SkialinSvgCanvas* svgCanvas) {
    svgCanvas->canvas.reset();
    SkData* result = svgCanvas->stream.detachAsData().release();
    delete svgCanvas;
    return result;
}

SkCodec* skialin_bridge_Codec_MakeFromData(const uint8_t* bytes, size_t length) {
    auto result = SkCodec::MakeFromData(SkData::MakeWithCopy(bytes, length));
    return result.release();
}

void skialin_bridge_Codec_delete(SkCodec* codec) {
    delete codec;
}

void skialin_bridge_Codec_dimensions(const SkCodec* codec, int32_t* outWidth, int32_t* outHeight) {
    SkISize size = codec->dimensions();
    *outWidth = size.width();
    *outHeight = size.height();
}

int32_t skialin_bridge_Codec_getEncodedFormat(const SkCodec* codec) {
    return static_cast<int32_t>(codec->getEncodedFormat());
}

int32_t skialin_bridge_Codec_getFrameCount(SkCodec* codec) {
    return codec->getFrameCount();
}

bool skialin_bridge_Codec_getFrameInfo(const SkCodec* codec, int32_t index, int32_t* outDurationMs, int32_t* outRequiredFrame, bool* outFullyReceived) {
    SkCodec::FrameInfo info;
    if (!codec->getFrameInfo(index, &info)) {
        return false;
    }
    *outDurationMs = info.fDuration;
    *outRequiredFrame = info.fRequiredFrame;
    *outFullyReceived = info.fFullyReceived;
    return true;
}

int32_t skialin_bridge_Codec_getPixels(SkCodec* codec, const SkImageInfo* info, void* pixels, size_t rowBytes, int32_t frameIndex) {
    SkCodec::Options options;
    options.fFrameIndex = frameIndex;
    return static_cast<int32_t>(codec->getPixels(*info, pixels, rowBytes, &options));
}

skottie::Animation* skialin_bridge_SkottieAnimation_Make(const char* data, size_t length) {
    return skottie::Animation::Make(data, length).release();
}

void skialin_bridge_SkottieAnimation_unref(skottie::Animation* animation) {
    SkSafeUnref(animation);
}

void skialin_bridge_SkottieAnimation_render(const skottie::Animation* animation, SkCanvas* canvas, const SkRect* dst) {
    animation->render(canvas, dst);
}

void skialin_bridge_SkottieAnimation_seek(skottie::Animation* animation, float t) {
    animation->seek(t);
}

void skialin_bridge_SkottieAnimation_seekFrame(skottie::Animation* animation, double frame) {
    animation->seekFrame(frame);
}

double skialin_bridge_SkottieAnimation_duration(const skottie::Animation* animation) {
    return animation->duration();
}

double skialin_bridge_SkottieAnimation_fps(const skottie::Animation* animation) {
    return animation->fps();
}

void skialin_bridge_SkottieAnimation_size(const skottie::Animation* animation, float* outWidth, float* outHeight) {
    const SkSize& size = animation->size();
    *outWidth = size.width();
    *outHeight = size.height();
}

namespace {
class SkialinDrawable final : public SkDrawable {
public:
    SkialinDrawable(void* context, SkialinDrawableDrawFn onDraw, SkialinDrawableGetBoundsFn onGetBounds, SkialinDrawableDisposeFn onDispose)
        : fContext(context), fOnDraw(onDraw), fOnGetBounds(onGetBounds), fOnDispose(onDispose) {}

    ~SkialinDrawable() override {
        fOnDispose(fContext);
    }

protected:
    void onDraw(SkCanvas* canvas) override {
        fOnDraw(fContext, canvas);
    }

    SkRect onGetBounds() override {
        SkRect bounds = SkRect::MakeEmpty();
        fOnGetBounds(fContext, &bounds);
        return bounds;
    }

private:
    void* fContext;
    SkialinDrawableDrawFn fOnDraw;
    SkialinDrawableGetBoundsFn fOnGetBounds;
    SkialinDrawableDisposeFn fOnDispose;
};
}  // namespace

SkDrawable* skialin_bridge_Drawable_Make(void* context, SkialinDrawableDrawFn onDraw, SkialinDrawableGetBoundsFn onGetBounds, SkialinDrawableDisposeFn onDispose) {
    return new SkialinDrawable(context, onDraw, onGetBounds, onDispose);
}

void skialin_bridge_Drawable_unref(SkDrawable* drawable) {
    SkSafeUnref(drawable);
}

SkPicture* skialin_bridge_Drawable_makePictureSnapshot(SkDrawable* drawable) {
    return drawable->makePictureSnapshot().release();
}

void skialin_bridge_Drawable_getBounds(SkDrawable* drawable, SkRect* outBounds) {
    *outBounds = drawable->getBounds();
}

uint32_t skialin_bridge_Drawable_getGenerationID(SkDrawable* drawable) {
    return drawable->getGenerationID();
}

void skialin_bridge_Drawable_notifyDrawingChanged(SkDrawable* drawable) {
    drawable->notifyDrawingChanged();
}

GrDirectContext* skialin_bridge_DirectContext_MakeGL(void) {
    return GrDirectContexts::MakeGL().release();
}

GrDirectContext* skialin_bridge_DirectContext_MakeGLAssembled(void* ctx, GrGLGetProc get) {
    sk_sp<const GrGLInterface> interface = GrGLMakeAssembledInterface(ctx, get);
    if (!interface) {
        return nullptr;
    }
    return GrDirectContexts::MakeGL(std::move(interface)).release();
}

void skialin_bridge_DirectContext_unref(GrDirectContext* context) {
    SkSafeUnref(context);
}

void skialin_bridge_DirectContext_flush(GrDirectContext* context) {
    context->flush();
}

void skialin_bridge_DirectContext_submit(GrDirectContext* context, bool syncCpu) {
    context->submit(syncCpu ? GrSyncCpu::kYes : GrSyncCpu::kNo);
}

void skialin_bridge_DirectContext_abandonContext(GrDirectContext* context) {
    context->abandonContext();
}

int64_t skialin_bridge_DirectContext_getResourceCacheLimit(GrDirectContext* context) {
    return static_cast<int64_t>(context->getResourceCacheLimit());
}

void skialin_bridge_DirectContext_setResourceCacheLimit(GrDirectContext* context, int64_t maxResourceBytes) {
    context->setResourceCacheLimit(static_cast<size_t>(maxResourceBytes));
}

GrDirectContext* skialin_bridge_DirectContext_MakeVulkan(
    VkInstance instance, VkPhysicalDevice physicalDevice, VkDevice device, VkQueue queue,
    uint32_t graphicsQueueIndex, uint32_t maxAPIVersion, void* getProcCtx, SkialinVulkanGetProc getProc,
    bool protectedContext) {
    skgpu::VulkanBackendContext backendContext;
    backendContext.fInstance = instance;
    backendContext.fPhysicalDevice = physicalDevice;
    backendContext.fDevice = device;
    backendContext.fQueue = queue;
    backendContext.fGraphicsQueueIndex = graphicsQueueIndex;
    backendContext.fMaxAPIVersion = maxAPIVersion;
    backendContext.fProtectedContext = protectedContext ? skgpu::Protected::kYes : skgpu::Protected::kNo;
    backendContext.fGetProc = [getProcCtx, getProc](const char* name, VkInstance inst, VkDevice dev) {
        return getProc(getProcCtx, name, inst, dev);
    };
    backendContext.fMemoryAllocator = skgpu::VulkanMemoryAllocators::Make(backendContext, skgpu::ThreadSafe::kNo);
    return GrDirectContexts::MakeVulkan(backendContext).release();
}

SkSurface* skialin_bridge_Surface_MakeRenderTarget(
    GrDirectContext* context, skgpu::Budgeted budgeted, const SkImageInfo* info,
    int32_t sampleCount, GrSurfaceOrigin surfaceOrigin, const SkSurfaceProps* surfaceProps,
    bool shouldCreateWithMips, bool isProtected) {
    return SkSurfaces::RenderTarget(context, budgeted, *info, sampleCount, surfaceOrigin, surfaceProps, shouldCreateWithMips, isProtected)
        .release();
}

GrBackendTexture* skialin_bridge_BackendTexture_MakeVk(int32_t width, int32_t height, const GrVkImageInfo* imageInfo, const char* label, size_t labelLength) {
    std::string_view labelView = label ? std::string_view(label, labelLength) : std::string_view();
    return new GrBackendTexture(GrBackendTextures::MakeVk(width, height, *imageInfo, labelView));
}

GrBackendTexture* skialin_bridge_BackendTexture_MakeGL(int32_t width, int32_t height, skgpu::Mipmapped mipmapped, const GrGLTextureInfo* glInfo, const char* label, size_t labelLength) {
    std::string_view labelView = label ? std::string_view(label, labelLength) : std::string_view();
    return new GrBackendTexture(GrBackendTextures::MakeGL(width, height, mipmapped, *glInfo, labelView));
}

void skialin_bridge_BackendTexture_delete(GrBackendTexture* texture) {
    delete texture;
}

GrBackendTexture* skialin_bridge_BackendTexture_clone(const GrBackendTexture* texture) {
    return new GrBackendTexture(*texture);
}

int32_t skialin_bridge_BackendTexture_width(const GrBackendTexture* texture) {
    return texture->width();
}

int32_t skialin_bridge_BackendTexture_height(const GrBackendTexture* texture) {
    return texture->height();
}

bool skialin_bridge_BackendTexture_isValid(const GrBackendTexture* texture) {
    return texture->isValid();
}

bool skialin_bridge_BackendTexture_isProtected(const GrBackendTexture* texture) {
    return texture->isProtected();
}

bool skialin_bridge_BackendTexture_hasMipmaps(const GrBackendTexture* texture) {
    return texture->hasMipmaps();
}

SkSurface* skialin_bridge_Surface_WrapBackendTexture(
    GrDirectContext* context, const GrBackendTexture* backendTexture, GrSurfaceOrigin origin,
    int32_t sampleCnt, SkColorType colorType, SkColorSpace* colorSpace, const SkSurfaceProps* surfaceProps,
    SkialinTextureReleaseProc releaseProc, void* releaseContext) {
    return SkSurfaces::WrapBackendTexture(
                   context, *backendTexture, origin, sampleCnt, colorType, sk_ref_sp(colorSpace), surfaceProps,
                   releaseProc, releaseContext)
        .release();
}

skgpu::graphite::Context* skialin_bridge_GraphiteContext_MakeVulkan(
    VkInstance instance, VkPhysicalDevice physicalDevice, VkDevice device, VkQueue queue,
    uint32_t graphicsQueueIndex, uint32_t maxAPIVersion, void* getProcCtx, SkialinVulkanGetProc getProc,
    bool protectedContext) {
    skgpu::VulkanBackendContext backendContext;
    backendContext.fInstance = instance;
    backendContext.fPhysicalDevice = physicalDevice;
    backendContext.fDevice = device;
    backendContext.fQueue = queue;
    backendContext.fGraphicsQueueIndex = graphicsQueueIndex;
    backendContext.fMaxAPIVersion = maxAPIVersion;
    backendContext.fProtectedContext = protectedContext ? skgpu::Protected::kYes : skgpu::Protected::kNo;
    backendContext.fGetProc = [getProcCtx, getProc](const char* name, VkInstance inst, VkDevice dev) {
        return getProc(getProcCtx, name, inst, dev);
    };
    backendContext.fMemoryAllocator = skgpu::VulkanMemoryAllocators::Make(backendContext, skgpu::ThreadSafe::kNo);
    return skgpu::graphite::ContextFactory::MakeVulkan(backendContext, skgpu::graphite::ContextOptions()).release();
}

void skialin_bridge_GraphiteContext_delete(skgpu::graphite::Context* context) {
    delete context;
}

skgpu::graphite::Recorder* skialin_bridge_GraphiteContext_makeRecorder(skgpu::graphite::Context* context) {
    return context->makeRecorder().release();
}

int32_t skialin_bridge_GraphiteContext_insertRecording(skgpu::graphite::Context* context, skgpu::graphite::Recording* recording, SkSurface* targetSurface) {
    skgpu::graphite::InsertRecordingInfo info;
    info.fRecording = recording;
    info.fTargetSurface = targetSurface;
    return static_cast<int32_t>(static_cast<skgpu::graphite::InsertStatus::V>(context->insertRecording(info)));
}

bool skialin_bridge_GraphiteContext_submit(skgpu::graphite::Context* context, bool syncToCpu) {
    return context->submit(skgpu::graphite::SubmitInfo(syncToCpu ? skgpu::graphite::SyncToCpu::kYes : skgpu::graphite::SyncToCpu::kNo));
}

void skialin_bridge_GraphiteRecorder_delete(skgpu::graphite::Recorder* recorder) {
    delete recorder;
}

skgpu::graphite::Recording* skialin_bridge_GraphiteRecorder_snap(skgpu::graphite::Recorder* recorder) {
    return recorder->snap().release();
}

SkSurface* skialin_bridge_GraphiteSurface_MakeRenderTarget(skgpu::graphite::Recorder* recorder, const SkImageInfo* info, skgpu::Mipmapped mipmapped, const SkSurfaceProps* surfaceProps) {
    return SkSurfaces::RenderTarget(recorder, *info, mipmapped, surfaceProps).release();
}

SkSurface* skialin_bridge_GraphiteSurface_WrapBackendTexture(
    skgpu::graphite::Recorder* recorder, const skgpu::graphite::BackendTexture* backendTexture, SkColorType colorType,
    SkColorSpace* colorSpace, const SkSurfaceProps* surfaceProps) {
    return SkSurfaces::WrapBackendTexture(recorder, *backendTexture, colorType, sk_ref_sp(colorSpace), surfaceProps).release();
}

void skialin_bridge_GraphiteRecording_delete(skgpu::graphite::Recording* recording) {
    delete recording;
}

skgpu::graphite::BackendTexture* skialin_bridge_GraphiteBackendTexture_MakeVk(
    int32_t width, int32_t height, int32_t sampleCount, bool mipmapped, uint32_t imageCreateFlags,
    VkFormat format, VkImageTiling imageTiling, VkImageUsageFlags imageUsageFlags, VkSharingMode sharingMode,
    VkImageAspectFlags aspectMask, VkImageLayout currentLayout, uint32_t queueFamilyIndex, VkImage image,
    VkDeviceMemory allocMemory, VkDeviceSize allocOffset, VkDeviceSize allocSize, uint32_t allocFlags) {
    skgpu::graphite::VulkanTextureInfo texInfo(
            static_cast<VkSampleCountFlagBits>(sampleCount),
            mipmapped ? skgpu::Mipmapped::kYes : skgpu::Mipmapped::kNo,
            imageCreateFlags, format, imageTiling, imageUsageFlags, sharingMode, aspectMask,
            skgpu::VulkanYcbcrConversionInfo());
    skgpu::VulkanAlloc alloc;
    alloc.fMemory = allocMemory;
    alloc.fOffset = allocOffset;
    alloc.fSize = allocSize;
    alloc.fFlags = allocFlags;
    return new skgpu::graphite::BackendTexture(skgpu::graphite::BackendTextures::MakeVulkan(
            SkISize::Make(width, height), texInfo, currentLayout, queueFamilyIndex, image, alloc));
}

void skialin_bridge_GraphiteBackendTexture_delete(skgpu::graphite::BackendTexture* texture) {
    delete texture;
}

bool skialin_bridge_GraphiteBackendTexture_isValid(const skgpu::graphite::BackendTexture* texture) {
    return texture->isValid();
}

SkPictureRecorder* skialin_bridge_PictureRecorder_new(void) {
    return new SkPictureRecorder();
}

void skialin_bridge_PictureRecorder_delete(SkPictureRecorder* recorder) {
    delete recorder;
}

SkCanvas* skialin_bridge_PictureRecorder_beginRecording(SkPictureRecorder* recorder, const SkRect* bounds) {
    return recorder->beginRecording(*bounds);
}

SkCanvas* skialin_bridge_PictureRecorder_getRecordingCanvas(SkPictureRecorder* recorder) {
    return recorder->getRecordingCanvas();
}

SkPicture* skialin_bridge_PictureRecorder_finishRecordingAsPicture(SkPictureRecorder* recorder) {
    return recorder->finishRecordingAsPicture().release();
}

void skialin_bridge_Picture_unref(SkPicture* picture) {
    SkSafeUnref(picture);
}

void skialin_bridge_Picture_playback(const SkPicture* picture, SkCanvas* canvas) {
    picture->playback(canvas);
}

void skialin_bridge_Picture_cullRect(const SkPicture* picture, SkRect* outRect) {
    *outRect = picture->cullRect();
}

uint32_t skialin_bridge_Picture_uniqueID(const SkPicture* picture) {
    return picture->uniqueID();
}

SkTextBlobBuilder* skialin_bridge_TextBlobBuilder_new(void) {
    return new SkTextBlobBuilder();
}

void skialin_bridge_TextBlobBuilder_delete(SkTextBlobBuilder* builder) {
    delete builder;
}

SkTextBlob* skialin_bridge_TextBlobBuilder_make(SkTextBlobBuilder* builder) {
    return builder->make().release();
}

int32_t skialin_bridge_Picture_approximateOpCount(const SkPicture* picture, bool nested) {
    return picture->approximateOpCount(nested);
}

void skialin_bridge_Canvas_drawPicture(SkCanvas* canvas, const SkPicture* picture) {
    canvas->drawPicture(picture);
}

void skialin_bridge_ShadowUtils_drawShadow(
    SkCanvas* canvas, const SkPath* path, float zPlaneX, float zPlaneY, float zPlaneZ,
    float lightX, float lightY, float lightZ, float lightRadius,
    uint32_t ambientColor, uint32_t spotColor, uint32_t flags) {
    SkShadowUtils::DrawShadow(
            canvas, *path, SkPoint3::Make(zPlaneX, zPlaneY, zPlaneZ), SkPoint3::Make(lightX, lightY, lightZ),
            lightRadius, ambientColor, spotColor, flags);
}

SkImage* skialin_bridge_Image_AdoptTextureFrom(
    GrDirectContext* context, const GrBackendTexture* backendTexture, GrSurfaceOrigin textureOrigin,
    SkColorType colorType, SkAlphaType alphaType, SkColorSpace* colorSpace) {
    return SkImages::AdoptTextureFrom(context, *backendTexture, textureOrigin, colorType, alphaType, sk_ref_sp(colorSpace)).release();
}

SkImage* skialin_bridge_Image_WrapGraphiteTexture(
    skgpu::graphite::Recorder* recorder, const skgpu::graphite::BackendTexture* backendTexture,
    SkAlphaType alphaType, SkColorSpace* colorSpace, skgpu::Origin origin, SkImages::GenerateMipmapsFromBase generateMipmapsFromBase) {
    return SkImages::WrapTexture(recorder, *backendTexture, alphaType, sk_ref_sp(colorSpace), origin, generateMipmapsFromBase).release();
}

}  // extern "C"
