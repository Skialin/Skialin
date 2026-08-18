#include "skialin/bridge.h"

#include <cstring>
#include <memory>

#include "include/core/SkSurface.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkImage.h"
#include "include/core/SkData.h"
#include "include/core/SkBitmap.h"
#include "include/core/SkPath.h"
#include "include/core/SkPathBuilder.h"
#include "include/core/SkColorSpace.h"
#include "include/core/SkPixmap.h"
#include "include/core/SkPaint.h"
#include "include/core/SkShader.h"
#include "include/core/SkSamplingOptions.h"
#include "include/core/SkTypeface.h"
#include "include/core/SkFontStyle.h"
#include "include/core/SkFontMgr.h"
#include "include/core/SkFont.h"
#include "include/core/SkTextBlob.h"
#include "modules/skparagraph/include/TextStyle.h"
#include "modules/skparagraph/include/ParagraphStyle.h"
#include "modules/skparagraph/include/FontCollection.h"
#include "modules/skparagraph/include/ParagraphBuilder.h"
#include "modules/skparagraph/include/Paragraph.h"
#include "modules/skparagraph/include/Metrics.h"
#include "modules/skunicode/include/SkUnicode_icu.h"
#include "include/core/SkString.h"
#include "include/core/SkStream.h"
#include "include/encode/SkPngEncoder.h"
#include "include/ports/SkTypeface_win.h"
#include "modules/skcms/skcms.h"

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
    return SkFontMgr_New_DirectWrite().release();
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

}  // extern "C"
