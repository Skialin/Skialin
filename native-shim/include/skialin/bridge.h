#pragma once

/* Bindgen binds Skia's C++ classes (SkPaint, SkPath, SkMatrix, SkCanvas, ...)
 * directly from Skia's own headers. This bridge exists only for the seams
 * bindgen cannot cross on its own: factory statics and methods that return
 * or consume sk_sp<T>, since sk_sp is a template and its instantiations
 * are not directly callable through a generated binding. Every function
 * here takes/returns a raw, already-ref-managed pointer so the Rust side
 * owns a plain pointer with clear, explicit lifetime rules. */

#include "include/core/SkImageInfo.h"
#include "include/core/SkMatrix.h"
#include "include/core/SkPoint.h"
#include "include/core/SkRect.h"
#include "include/core/SkSamplingOptions.h"
#include "include/core/SkTileMode.h"

class SkSurface;
class SkCanvas;
class SkImage;
class SkData;
class SkBitmap;
class SkPath;
class SkPathBuilder;
class SkColorSpace;
class SkPixmap;
class SkShader;
class SkPaint;

extern "C" {

/* By-value returns from a C++ member function are classified differently
 * by clang (bindgen's parser) than by the MSVC cl.exe that built skia.lib,
 * corrupting registers/stack on return. These wrappers use the plain C ABI,
 * which both compilers implement identically: trivial types are returned
 * through an explicit out-parameter-shaped signature, non-trivial types
 * (SkPath) are heap-allocated and returned as an owned pointer. */
SkPoint skialin_bridge_Matrix_mapPoint(const SkMatrix* matrix, SkPoint point);
SkRect skialin_bridge_Matrix_mapRect(const SkMatrix* matrix, const SkRect* rect);
SkRect skialin_bridge_PathBuilder_computeBounds(const SkPathBuilder* builder);

/* Path: ref-owned by the caller. Free with skialin_bridge_Path_delete. */
SkPath* skialin_bridge_PathBuilder_snapshot(const SkPathBuilder* builder, const SkMatrix* matrix);
SkPath* skialin_bridge_PathBuilder_detach(SkPathBuilder* builder, const SkMatrix* matrix);
void skialin_bridge_Path_delete(SkPath* path);

/* Surface: ref-owned by the caller. Free with skialin_bridge_Surface_unref. */
SkSurface* skialin_bridge_Surface_MakeRasterN32Premul(int32_t width, int32_t height);
SkSurface* skialin_bridge_Surface_MakeRaster(const SkImageInfo* info);
void skialin_bridge_Surface_unref(SkSurface* surface);

/* Borrowed for the lifetime of the surface, never freed independently. */
SkCanvas* skialin_bridge_Surface_getCanvas(SkSurface* surface);

/* Ref-owned by the caller. Free with skialin_bridge_Image_unref. */
SkImage* skialin_bridge_Surface_makeImageSnapshot(SkSurface* surface);

/* Image: ref-owned by the caller. Free with skialin_bridge_Image_unref.
 * SkSamplingOptions has no sk_sp members so is passed as 6 flat scalars
 * (maxAniso, useCubic, cubicB, cubicC, filter, mipmap) rather than needing
 * its own opaque type. */
void skialin_bridge_Image_unref(SkImage* image);
SkImage* skialin_bridge_Image_MakeFromEncoded(const uint8_t* bytes, size_t length);
SkData* skialin_bridge_Image_encodeToData(const SkImage* image);

/* SkImage is abstract (pure virtual methods), so bindgen generates no
 * instance methods for it at all, not even the concrete inline ones: every
 * accessor is routed through the bridge. */
int32_t skialin_bridge_Image_width(const SkImage* image);
int32_t skialin_bridge_Image_height(const SkImage* image);
uint32_t skialin_bridge_Image_uniqueID(const SkImage* image);
SkAlphaType skialin_bridge_Image_alphaType(const SkImage* image);
SkColorType skialin_bridge_Image_colorType(const SkImage* image);
/* Borrowed; null if this Image has no color space. */
SkColorSpace* skialin_bridge_Image_colorSpace(const SkImage* image);

/* Owned by the caller; free with skialin_bridge_ImageInfo_delete. */
SkImageInfo* skialin_bridge_Image_imageInfo(const SkImage* image);
bool skialin_bridge_Image_isAlphaOnly(const SkImage* image);
bool skialin_bridge_Image_isOpaque(const SkImage* image);
bool skialin_bridge_Image_isTextureBacked(const SkImage* image);
bool skialin_bridge_Image_isLazyGenerated(const SkImage* image);
bool skialin_bridge_Image_hasMipmaps(const SkImage* image);
bool skialin_bridge_Image_isProtected(const SkImage* image);
/* Ref-owned by the caller; null if this Image has no color space. */
SkColorSpace* skialin_bridge_Image_refColorSpace(const SkImage* image);

SkShader* skialin_bridge_Image_makeShader(
    const SkImage* image, SkTileMode tmx, SkTileMode tmy,
    int32_t maxAniso, bool useCubic, float cubicB, float cubicC, SkFilterMode filter, SkMipmapMode mipmap,
    const SkMatrix* localMatrix);
SkShader* skialin_bridge_Image_makeRawShader(
    const SkImage* image, SkTileMode tmx, SkTileMode tmy,
    int32_t maxAniso, bool useCubic, float cubicB, float cubicC, SkFilterMode filter, SkMipmapMode mipmap,
    const SkMatrix* localMatrix);

/* True and fills pixmap if the image has direct pixel access. */
bool skialin_bridge_Image_peekPixels(const SkImage* image, SkPixmap* pixmap);
bool skialin_bridge_Image_readPixels(const SkImage* image, const SkImageInfo* dstInfo, void* dstPixels, size_t dstRowBytes, int32_t srcX, int32_t srcY);
bool skialin_bridge_Image_scalePixels(
    const SkImage* image, SkPixmap* dst,
    int32_t maxAniso, bool useCubic, float cubicB, float cubicC, SkFilterMode filter, SkMipmapMode mipmap);
/* Null if the requested ColorInfo/dimensions are unsupported. */
SkImage* skialin_bridge_Image_makeScaled(
    const SkImage* image, const SkImageInfo* info,
    int32_t maxAniso, bool useCubic, float cubicB, float cubicC, SkFilterMode filter, SkMipmapMode mipmap);

/* Null if this Image wasn't created from an encoded stream. */
SkData* skialin_bridge_Image_refEncodedData(const SkImage* image);
/* Null if subset is empty, out of bounds, or pixels can't be read. */
SkImage* skialin_bridge_Image_makeSubset(const SkImage* image, int32_t left, int32_t top, int32_t right, int32_t bottom, bool mipmapped);
SkImage* skialin_bridge_Image_withDefaultMipmaps(const SkImage* image);
/* Null on failure (texture-backed image whose GPU readback fails). */
SkImage* skialin_bridge_Image_makeNonTextureImage(const SkImage* image);
SkImage* skialin_bridge_Image_makeRasterImage(const SkImage* image, bool allowCaching);
bool skialin_bridge_Image_asLegacyBitmap(const SkImage* image, SkBitmap* bitmap);
/* Null if the requested ColorType/ColorSpace is unsupported. */
SkImage* skialin_bridge_Image_makeColorSpace(const SkImage* image, SkColorSpace* targetColorSpace, bool mipmapped);
SkImage* skialin_bridge_Image_makeColorTypeAndColorSpace(const SkImage* image, SkColorType targetColorType, SkColorSpace* targetColorSpace, bool mipmapped);
SkImage* skialin_bridge_Image_reinterpretColorSpace(const SkImage* image, SkColorSpace* newColorSpace);

/* Factories, ref-owned by the caller. */
SkImage* skialin_bridge_Image_RasterFromPixmapCopy(const SkPixmap* pixmap);
/* pixels' bytes become this Image's pixel storage (no copy); pixels is ref'd. */
SkImage* skialin_bridge_Image_RasterFromData(const SkImageInfo* info, SkData* pixels, size_t rowBytes);

/* Bitmap -> Image conversion; result is ref-owned by the caller. */
SkImage* skialin_bridge_Bitmap_asImage(const SkBitmap* bitmap);

/* Data: ref-owned by the caller. Free with skialin_bridge_Data_unref. */
void skialin_bridge_Data_unref(SkData* data);
SkData* skialin_bridge_Data_makeEmpty(void);
SkData* skialin_bridge_Data_makeWithCopy(const void* data, size_t length);
SkData* skialin_bridge_Data_makeUninitialized(size_t length);
SkData* skialin_bridge_Data_makeZeroInitialized(size_t length);
/* Null if the file can't be opened. */
SkData* skialin_bridge_Data_makeFromFileName(const char* path);
/* Null if offset+length is out of range. */
SkData* skialin_bridge_Data_copySubset(const SkData* data, size_t offset, size_t length);
SkData* skialin_bridge_Data_shareSubset(const SkData* data, size_t offset, size_t length);

/* ColorSpace: ref-owned by the caller. Free with skialin_bridge_ColorSpace_unref.
 * skcms_TransferFunction is passed as 7 floats (g,a,b,c,d,e,f); skcms_Matrix3x3
 * as 9 floats, row-major. */
void skialin_bridge_ColorSpace_unref(SkColorSpace* cs);
SkColorSpace* skialin_bridge_ColorSpace_makeSRGB(void);
SkColorSpace* skialin_bridge_ColorSpace_makeSRGBLinear(void);
SkColorSpace* skialin_bridge_ColorSpace_makeRGB(const float* transferFn7, const float* toXyz9);
/* Null for an invalid or unsupported combination of code points. */
SkColorSpace* skialin_bridge_ColorSpace_makeCICP(uint8_t colorPrimaries, uint8_t transferCharacteristics);
/* Null if the bytes don't parse as an ICC profile. */
SkColorSpace* skialin_bridge_ColorSpace_makeFromIccProfile(const uint8_t* bytes, size_t length);
SkColorSpace* skialin_bridge_ColorSpace_deserialize(const uint8_t* bytes, size_t length);
SkColorSpace* skialin_bridge_ColorSpace_makeLinearGamma(const SkColorSpace* cs);
SkColorSpace* skialin_bridge_ColorSpace_makeSRGBGamma(const SkColorSpace* cs);
SkColorSpace* skialin_bridge_ColorSpace_makeColorSpin(const SkColorSpace* cs);
bool skialin_bridge_ColorSpace_toXYZD50(const SkColorSpace* cs, float* outXyz9);
void skialin_bridge_ColorSpace_transferFn(const SkColorSpace* cs, float* outFn7);
void skialin_bridge_ColorSpace_invTransferFn(const SkColorSpace* cs, float* outFn7);
bool skialin_bridge_ColorSpace_isNumericalTransferFn(const SkColorSpace* cs, float* outFn7);
void skialin_bridge_ColorSpace_gamutTransformTo(const SkColorSpace* src, const SkColorSpace* dst, float* outXyz9);
SkData* skialin_bridge_ColorSpace_serialize(const SkColorSpace* cs);
bool skialin_bridge_ColorSpace_equals(const SkColorSpace* a, const SkColorSpace* b);

/* ImageInfo: heap-allocated because SkImageInfo holds a non-trivial
 * sk_sp<SkColorSpace> member, so returning it by value from a bindgen-called
 * method hits the same by-value ABI hazard as SkPath. Owned by the caller;
 * free with skialin_bridge_ImageInfo_delete. colorSpace may be null. */
SkImageInfo* skialin_bridge_ImageInfo_make(int32_t width, int32_t height, SkColorType colorType, SkAlphaType alphaType, SkColorSpace* colorSpace);
void skialin_bridge_ImageInfo_delete(SkImageInfo* info);
SkImageInfo* skialin_bridge_ImageInfo_makeWH(const SkImageInfo* info, int32_t width, int32_t height);
SkImageInfo* skialin_bridge_ImageInfo_makeColorType(const SkImageInfo* info, SkColorType colorType);
SkImageInfo* skialin_bridge_ImageInfo_makeAlphaType(const SkImageInfo* info, SkAlphaType alphaType);
SkImageInfo* skialin_bridge_ImageInfo_makeColorSpace(const SkImageInfo* info, SkColorSpace* colorSpace);
/* Borrowed; null if this ImageInfo has no color space. */
SkColorSpace* skialin_bridge_ImageInfo_colorSpace(const SkImageInfo* info);
/* Ref-owned by the caller; null if this ImageInfo has no color space. */
SkColorSpace* skialin_bridge_ImageInfo_refColorSpace(const SkImageInfo* info);
bool skialin_bridge_ImageInfo_equals(const SkImageInfo* a, const SkImageInfo* b);

/* Pixmap: heap-allocated because SkPixmap holds a non-trivial SkImageInfo
 * member by value; same by-value ABI hazard as SkPath/SkImageInfo. Owned by
 * the caller; free with skialin_bridge_Pixmap_delete. Never owns the pixel
 * memory itself: addr must outlive the Pixmap. */
SkPixmap* skialin_bridge_Pixmap_make(const SkImageInfo* info, const void* addr, size_t rowBytes);
void skialin_bridge_Pixmap_delete(SkPixmap* pixmap);
/* Ref-owned by the caller; null if this Pixmap has no color space. */
SkColorSpace* skialin_bridge_Pixmap_refColorSpace(const SkPixmap* pixmap);
/* Null if the intersection of pixmap and area is empty. */
SkPixmap* skialin_bridge_Pixmap_extractSubset(const SkPixmap* pixmap, int32_t left, int32_t top, int32_t right, int32_t bottom);

/* Shader: ref-owned by the caller. Free with skialin_bridge_Shader_unref.
 * Routed entirely through the bridge, not direct bindgen calls: bindgen
 * doesn't generate instance methods for SkShader (its SkFlattenable base
 * defeats its vtable-layout inference). */
void skialin_bridge_Shader_unref(SkShader* shader);
SkShader* skialin_bridge_Shader_makeEmpty(void);
SkShader* skialin_bridge_Shader_makeColor(uint32_t argb);
SkShader* skialin_bridge_Shader_makeWithLocalMatrix(const SkShader* shader, const SkMatrix* localMatrix);
bool skialin_bridge_Shader_isOpaque(const SkShader* shader);

/* Attaches shader to paint; shader may be null to clear it. */
void skialin_bridge_Paint_setShader(SkPaint* paint, SkShader* shader);

}  // extern "C"
