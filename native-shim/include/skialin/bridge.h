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
class SkTypeface;
class SkFontMgr;
class SkFont;
class SkTextBlob;
class SkColorFilter;
class SkImageFilter;
class SkMaskFilter;
class SkRuntimeEffect;
class SkRRect;
class SkPathEffect;
class SkPathMeasure;
enum class SkBlendMode;
enum class SkClipOp;
enum SkBlurStyle : int;

namespace skia {
namespace textlayout {
class TextStyle;
struct ParagraphStyle;
class FontCollection;
class ParagraphBuilder;
class Paragraph;
}  // namespace textlayout
}  // namespace skia

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
void skialin_bridge_Canvas_getTotalMatrix(const SkCanvas* canvas, SkMatrix* outMatrix);

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
/* Default-constructed: no pixels, kUnknown_SkColorType, zero size. Useful as
 * an out-parameter target, e.g. for SkImage::peekPixels. */
SkPixmap* skialin_bridge_Pixmap_makeEmpty(void);
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

/* Gradients: ref-owned by the caller. Free with skialin_bridge_Shader_unref.
 * colors is `count` packed ARGB uint32s; positions may be null (colors
 * distributed evenly) or `count` strictly-increasing floats in [0, 1].
 * colorSpace is always sRGB and interpolation always the default
 * (destination color space, not premultiplied) for this pass; localMatrix
 * may be null. Null result if the inputs are invalid (e.g. count < 2,
 * non-positive radius). */
SkShader* skialin_bridge_Shader_makeLinearGradient(
    const SkPoint* pts /* [2] */, const uint32_t* colors, const float* positions, size_t count, SkTileMode tileMode, const SkMatrix* localMatrix);
SkShader* skialin_bridge_Shader_makeRadialGradient(
    SkPoint center, float radius, const uint32_t* colors, const float* positions, size_t count, SkTileMode tileMode, const SkMatrix* localMatrix);
SkShader* skialin_bridge_Shader_makeTwoPointConicalGradient(
    SkPoint start, float startRadius, SkPoint end, float endRadius,
    const uint32_t* colors, const float* positions, size_t count, SkTileMode tileMode, const SkMatrix* localMatrix);
SkShader* skialin_bridge_Shader_makeSweepGradient(
    SkPoint center, float startAngle, float endAngle,
    const uint32_t* colors, const float* positions, size_t count, SkTileMode tileMode, const SkMatrix* localMatrix);

/* RuntimeEffect: ref-owned by the caller. Free with
 * skialin_bridge_RuntimeEffect_unref. Scoped to shader and color-filter
 * effects (the two common cases); MakeForBlender/makeBlender are not yet
 * bound. Uniforms are passed as a raw packed byte buffer matching the
 * SkSL uniform block's layout (offsets from SkRuntimeEffect::uniforms(),
 * not yet exposed here either -- callers must know their own layout).
 * children are borrowed for the duration of the call (ref'd internally,
 * not consumed). On failure, *outError is set to a UTF-8, non-NUL-terminated
 * SkData describing the compile error (ref-owned by the caller, free with
 * skialin_bridge_Data_unref); on success *outError is left untouched by
 * the caller's responsibility to have zero-initialized it first. */
SkRuntimeEffect* skialin_bridge_RuntimeEffect_MakeForShader(const char* sksl, size_t length, SkData** outError);
SkRuntimeEffect* skialin_bridge_RuntimeEffect_MakeForColorFilter(const char* sksl, size_t length, SkData** outError);
void skialin_bridge_RuntimeEffect_unref(SkRuntimeEffect* effect);
SkShader* skialin_bridge_RuntimeEffect_makeShader(
    const SkRuntimeEffect* effect, const uint8_t* uniforms, size_t uniformsLength,
    SkShader* const* children, size_t childCount, const SkMatrix* localMatrix);
SkColorFilter* skialin_bridge_RuntimeEffect_makeColorFilter(
    const SkRuntimeEffect* effect, const uint8_t* uniforms, size_t uniformsLength,
    SkColorFilter* const* children, size_t childCount);

/* RRect: heap-allocated with `new`/`delete`, same rationale as ImageInfo/
 * Pixmap (a plain value class, but rect()/radii() etc. return non-trivial-
 * sized values by value, which hits the gotcha #1 ABI hazard; routed
 * entirely through the bridge for consistency since it's never visible to
 * bindgen at all -- forward-declared only). radii8, where present, is 4
 * corners * (x, y) in Corner enum order: upper-left, upper-right,
 * lower-right, lower-left (matches SkRRect::Corner). type: 0 empty,
 * 1 rect, 2 oval, 3 simple, 4 nine-patch, 5 complex (matches SkRRect::Type). */
SkRRect* skialin_bridge_RRect_MakeRect(const SkRect* rect);
SkRRect* skialin_bridge_RRect_MakeOval(const SkRect* oval);
SkRRect* skialin_bridge_RRect_MakeRectXY(const SkRect* rect, float xRad, float yRad);
SkRRect* skialin_bridge_RRect_MakeRectRadii(const SkRect* rect, const float* radii8);
void skialin_bridge_RRect_delete(SkRRect* rrect);
SkRRect* skialin_bridge_RRect_clone(const SkRRect* rrect);
void skialin_bridge_RRect_rect(const SkRRect* rrect, SkRect* outRect);
void skialin_bridge_RRect_radii(const SkRRect* rrect, float* outRadii8);
int32_t skialin_bridge_RRect_type(const SkRRect* rrect);
bool skialin_bridge_RRect_containsPoint(const SkRRect* rrect, SkPoint point);
bool skialin_bridge_RRect_containsRect(const SkRRect* rrect, const SkRect* rect);
bool skialin_bridge_RRect_isValid(const SkRRect* rrect);
SkRRect* skialin_bridge_RRect_inset(const SkRRect* rrect, float dx, float dy);
SkRRect* skialin_bridge_RRect_outset(const SkRRect* rrect, float dx, float dy);
/* Null if the matrix doesn't preserve the rounded-rect shape (e.g. a skew). */
SkRRect* skialin_bridge_RRect_transform(const SkRRect* rrect, const SkMatrix* matrix);

/* Canvas draw/clip calls that need an SkRRect. paint may be null for clipRRect. */
void skialin_bridge_Canvas_drawRRect(SkCanvas* canvas, const SkRRect* rrect, const SkPaint* paint);
void skialin_bridge_Canvas_drawDRRect(SkCanvas* canvas, const SkRRect* outer, const SkRRect* inner, const SkPaint* paint);
void skialin_bridge_Canvas_clipRRect(SkCanvas* canvas, const SkRRect* rrect, SkClipOp op);

/* PathEffect: ref-owned by the caller. Free with skialin_bridge_PathEffect_unref.
 * Routed entirely through the bridge: SkPathEffect's SkFlattenable base
 * defeats bindgen's vtable-layout inference, same as SkShader/SkColorFilter.
 * Scoped to Dash/Corner/Discrete/Trim plus Compose/Sum. 1D/2D path effects
 * (path-along-path, line/tile-based) are not yet bound. */
void skialin_bridge_PathEffect_unref(SkPathEffect* effect);
/* Null if intervals is empty, has an odd count, or any interval is negative. */
SkPathEffect* skialin_bridge_PathEffect_MakeDash(const float* intervals, size_t count, float phase);
SkPathEffect* skialin_bridge_PathEffect_MakeCorner(float radius);
SkPathEffect* skialin_bridge_PathEffect_MakeDiscrete(float segLength, float deviation, uint32_t seedAssist);
/* mode: 0 = normal (keep [startT, stopT]), 1 = inverted (keep the complement). */
SkPathEffect* skialin_bridge_PathEffect_MakeTrim(float startT, float stopT, int32_t mode);
SkPathEffect* skialin_bridge_PathEffect_MakeCompose(SkPathEffect* outer, SkPathEffect* inner);
SkPathEffect* skialin_bridge_PathEffect_MakeSum(SkPathEffect* first, SkPathEffect* second);

/* Attaches a path effect to paint; the effect may be null to clear it. */
void skialin_bridge_Paint_setPathEffect(SkPaint* paint, SkPathEffect* effect);

/* PathMeasure: owned by the caller. Free with skialin_bridge_PathMeasure_delete.
 * Routed entirely through the bridge, not direct bindgen calls: SkPathMeasure
 * holds a non-trivial sk_sp<SkContourMeasure> member, the same category as
 * SkFont, and is never given to bindgen (forward-declared only) so there
 * are no raw symbols to call anyway. flags for getMatrix: 0x1 = position,
 * 0x2 = tangent, 0x3 = both (matches SkPathMeasure::MatrixFlags). */
SkPathMeasure* skialin_bridge_PathMeasure_new(const SkPath* path, bool forceClosed, float resScale);
void skialin_bridge_PathMeasure_delete(SkPathMeasure* measure);
/* path may be null to clear it. */
void skialin_bridge_PathMeasure_setPath(SkPathMeasure* measure, const SkPath* path, bool forceClosed);
float skialin_bridge_PathMeasure_getLength(SkPathMeasure* measure);
/* False (position/tangent unchanged) if there is no path or it's zero-length. */
bool skialin_bridge_PathMeasure_getPosTan(SkPathMeasure* measure, float distance, SkPoint* outPosition, SkPoint* outTangent);
bool skialin_bridge_PathMeasure_getMatrix(SkPathMeasure* measure, float distance, SkMatrix* outMatrix, int32_t flags);
/* False (dst untouched) if the segment is zero-length or startD > stopD. */
bool skialin_bridge_PathMeasure_getSegment(SkPathMeasure* measure, float startD, float stopD, SkPathBuilder* dst, bool startWithMoveTo);
bool skialin_bridge_PathMeasure_isClosed(SkPathMeasure* measure);
bool skialin_bridge_PathMeasure_nextContour(SkPathMeasure* measure);

/* Typeface: ref-owned by the caller. Free with skialin_bridge_Typeface_unref.
 * SkTypeface has pure virtual methods (onGetFamilyName etc.), which defeats
 * bindgen's vtable-layout inference the same way SkImage does, so every
 * accessor is routed through the bridge. SkFontStyle is passed as 3 flat
 * ints (weight, width, slant) rather than the packed struct itself. */
void skialin_bridge_Typeface_unref(SkTypeface* typeface);
SkTypeface* skialin_bridge_Typeface_MakeEmpty(void);
uint32_t skialin_bridge_Typeface_uniqueID(const SkTypeface* typeface);
bool skialin_bridge_Typeface_isBold(const SkTypeface* typeface);
bool skialin_bridge_Typeface_isItalic(const SkTypeface* typeface);
bool skialin_bridge_Typeface_isFixedPitch(const SkTypeface* typeface);
int32_t skialin_bridge_Typeface_countGlyphs(const SkTypeface* typeface);
int32_t skialin_bridge_Typeface_getUnitsPerEm(const SkTypeface* typeface);
uint16_t skialin_bridge_Typeface_unicharToGlyph(const SkTypeface* typeface, int32_t unichar);
void skialin_bridge_Typeface_fontStyle(const SkTypeface* typeface, int32_t* weight, int32_t* width, int32_t* slant);
/* UTF-8 bytes, no NUL terminator. Ref-owned by the caller; free with skialin_bridge_Data_unref. */
SkData* skialin_bridge_Typeface_familyName(const SkTypeface* typeface);

/* FontMgr: ref-owned by the caller. Free with skialin_bridge_FontMgr_unref.
 * SkFontMgr is abstract (pure virtual methods), so it's routed entirely
 * through the bridge, same as SkTypeface. RefSystem is the platform default
 * (DirectWrite on Windows, the only platform this shim currently builds for). */
void skialin_bridge_FontMgr_unref(SkFontMgr* mgr);
SkFontMgr* skialin_bridge_FontMgr_RefSystem(void);
SkFontMgr* skialin_bridge_FontMgr_RefEmpty(void);
int32_t skialin_bridge_FontMgr_countFamilies(const SkFontMgr* mgr);
/* UTF-8 bytes, no NUL terminator. Ref-owned by the caller. */
SkData* skialin_bridge_FontMgr_familyName(const SkFontMgr* mgr, int32_t index);
/* familyName may be null to request the default system family. Null result
 * if no match is found. */
SkTypeface* skialin_bridge_FontMgr_matchFamilyStyle(const SkFontMgr* mgr, const char* familyName, int32_t weight, int32_t width, int32_t slant);
/* data is ref'd by the bridge, not consumed: it stays independently valid
 * and closeable afterward. Null if the data isn't a recognized font format. */
SkTypeface* skialin_bridge_FontMgr_makeFromData(const SkFontMgr* mgr, SkData* data, int32_t ttcIndex);
/* Null if the file isn't found or isn't a recognized font format. */
SkTypeface* skialin_bridge_FontMgr_makeFromFile(const SkFontMgr* mgr, const char* path, int32_t ttcIndex);

/* Font: heap-allocated with `new`/`delete`, not ref-counted itself (it holds
 * a strong sk_sp ref to its Typeface internally). SkFont carries
 * `sk_is_trivially_relocatable = std::true_type`, so unlike SkPathBuilder
 * (gotcha #2) there's no in-place-construction requirement; a plain `new`
 * is safe. Only the two seams that cross the sk_sp ownership boundary
 * (construction with a typeface, and refTypeface/setTypeface) are bridged;
 * every other accessor's raw bindgen symbol (SkFont_getSize, SkFont_setSize,
 * etc.) is called directly from skialin-core since bindgen exposes them as
 * free functions even though it doesn't generate an `impl SkFont` block
 * (SkFont's non-trivial sk_sp<SkTypeface> member trips the same inference
 * gap that produces gotcha #2/#4, but only for the impl-block sugar; the
 * plain extern symbols are still emitted and are safe to call: their
 * signatures are all scalar/void returns with by-value SkSpan/SkPoint
 * parameters, which is only unsafe for by-value *returns* per gotcha #1). */
SkFont* skialin_bridge_Font_MakeDefault(void);
SkFont* skialin_bridge_Font_MakeWithTypeface(SkTypeface* typeface, float size);
void skialin_bridge_Font_delete(SkFont* font);
/* Ref-owned by the caller; null if this Font has no typeface. Free with skialin_bridge_Typeface_unref. */
SkTypeface* skialin_bridge_Font_refTypeface(const SkFont* font);
/* typeface may be null to clear it; ref'd by the bridge, not consumed. */
void skialin_bridge_Font_setTypeface(SkFont* font, SkTypeface* typeface);

/* TextBlob: ref-owned by the caller. Free with skialin_bridge_TextBlob_unref.
 * Factories return sk_sp<SkTextBlob> by value in the real API, which hits
 * the same by-value ABI hazard as SkPath (gotcha #1), so they're bridged.
 * text is encoded per `encoding` (SkTextEncoding::kUTF8 = 0, kUTF16 = 1,
 * kUTF32 = 2, kGlyphID = 3); byteLength is the byte length of text.
 * Null if byteLength is zero. */
void skialin_bridge_TextBlob_unref(SkTextBlob* blob);
SkTextBlob* skialin_bridge_TextBlob_MakeFromText(const void* text, size_t byteLength, const SkFont* font, int32_t encoding);
/* xpos.length must equal the glyph/character count implied by text/byteLength/encoding. */
SkTextBlob* skialin_bridge_TextBlob_MakeFromPosTextH(const void* text, size_t byteLength, const float* xpos, size_t xposLength, float constY, const SkFont* font, int32_t encoding);
/* pos.length must equal the glyph/character count implied by text/byteLength/encoding. */
SkTextBlob* skialin_bridge_TextBlob_MakeFromPosText(const void* text, size_t byteLength, const SkPoint* pos, size_t posLength, const SkFont* font, int32_t encoding);

/* TextStyle (skia::textlayout::TextStyle): heap-allocated with `new`/`delete`.
 * A plain value class with non-trivial members (std::vector<SkString>,
 * sk_sp<SkTypeface>, a paint-or-id variant), so like SkFont it's routed
 * entirely through the bridge rather than relying on bindgen's per-method
 * symbols. Scoped to the common styling surface for this pass: color, font
 * family/size/style, decoration, spacing, height, typeface, locale.
 * Foreground/background paint, shadows, font features, font arguments, and
 * placeholders are not yet bound. TextDecoration/-Mode/-Style are passed as
 * plain ints matching the C++ enum's integer values (see TextStyle.h). */
skia::textlayout::TextStyle* skialin_bridge_TextStyle_new(void);
skia::textlayout::TextStyle* skialin_bridge_TextStyle_clone(const skia::textlayout::TextStyle* style);
void skialin_bridge_TextStyle_delete(skia::textlayout::TextStyle* style);

uint32_t skialin_bridge_TextStyle_getColor(const skia::textlayout::TextStyle* style);
void skialin_bridge_TextStyle_setColor(skia::textlayout::TextStyle* style, uint32_t color);

/* families is `count` C strings with matching byte lengths in `lengths` (not necessarily NUL-terminated). */
void skialin_bridge_TextStyle_setFontFamilies(skia::textlayout::TextStyle* style, const char* const* families, const size_t* lengths, size_t count);
size_t skialin_bridge_TextStyle_countFontFamilies(const skia::textlayout::TextStyle* style);
/* UTF-8 bytes, no NUL terminator. Ref-owned by the caller; free with skialin_bridge_Data_unref. */
SkData* skialin_bridge_TextStyle_fontFamily(const skia::textlayout::TextStyle* style, size_t index);

float skialin_bridge_TextStyle_getFontSize(const skia::textlayout::TextStyle* style);
void skialin_bridge_TextStyle_setFontSize(skia::textlayout::TextStyle* style, float size);

void skialin_bridge_TextStyle_getFontStyle(const skia::textlayout::TextStyle* style, int32_t* weight, int32_t* width, int32_t* slant);
void skialin_bridge_TextStyle_setFontStyle(skia::textlayout::TextStyle* style, int32_t weight, int32_t width, int32_t slant);

void skialin_bridge_TextStyle_getDecoration(const skia::textlayout::TextStyle* style, int32_t* type, int32_t* mode, uint32_t* color, int32_t* decorationStyle, float* thicknessMultiplier);
void skialin_bridge_TextStyle_setDecoration(skia::textlayout::TextStyle* style, int32_t type);
void skialin_bridge_TextStyle_setDecorationMode(skia::textlayout::TextStyle* style, int32_t mode);
void skialin_bridge_TextStyle_setDecorationColor(skia::textlayout::TextStyle* style, uint32_t color);
void skialin_bridge_TextStyle_setDecorationStyle(skia::textlayout::TextStyle* style, int32_t decorationStyle);
void skialin_bridge_TextStyle_setDecorationThicknessMultiplier(skia::textlayout::TextStyle* style, float multiplier);

float skialin_bridge_TextStyle_getLetterSpacing(const skia::textlayout::TextStyle* style);
void skialin_bridge_TextStyle_setLetterSpacing(skia::textlayout::TextStyle* style, float letterSpacing);
float skialin_bridge_TextStyle_getWordSpacing(const skia::textlayout::TextStyle* style);
void skialin_bridge_TextStyle_setWordSpacing(skia::textlayout::TextStyle* style, float wordSpacing);

float skialin_bridge_TextStyle_getHeight(const skia::textlayout::TextStyle* style);
void skialin_bridge_TextStyle_setHeight(skia::textlayout::TextStyle* style, float height);
bool skialin_bridge_TextStyle_getHeightOverride(const skia::textlayout::TextStyle* style);
void skialin_bridge_TextStyle_setHeightOverride(skia::textlayout::TextStyle* style, bool heightOverride);

/* Ref-owned by the caller; null if this TextStyle has no typeface. Free with skialin_bridge_Typeface_unref. */
SkTypeface* skialin_bridge_TextStyle_refTypeface(const skia::textlayout::TextStyle* style);
/* typeface may be null to clear it; ref'd by the bridge, not consumed. */
void skialin_bridge_TextStyle_setTypeface(skia::textlayout::TextStyle* style, SkTypeface* typeface);

/* UTF-8 bytes, no NUL terminator. Ref-owned by the caller; free with skialin_bridge_Data_unref. */
SkData* skialin_bridge_TextStyle_getLocale(const skia::textlayout::TextStyle* style);
void skialin_bridge_TextStyle_setLocale(skia::textlayout::TextStyle* style, const char* locale, size_t length);

/* ParagraphStyle (skia::textlayout::ParagraphStyle): heap-allocated with
 * `new`/`delete`, same rationale as TextStyle. TextDirection is the key
 * knob for RTL/bidi layout (skparagraph resolves character-level bidi via
 * ICU internally once this is set to kRtl); TextAlign/TextDirection ints
 * match the C++ enums in DartTypes.h. StrutStyle is not yet bound. */
skia::textlayout::ParagraphStyle* skialin_bridge_ParagraphStyle_new(void);
void skialin_bridge_ParagraphStyle_delete(skia::textlayout::ParagraphStyle* style);

int32_t skialin_bridge_ParagraphStyle_getTextDirection(const skia::textlayout::ParagraphStyle* style);
void skialin_bridge_ParagraphStyle_setTextDirection(skia::textlayout::ParagraphStyle* style, int32_t direction);
int32_t skialin_bridge_ParagraphStyle_getTextAlign(const skia::textlayout::ParagraphStyle* style);
void skialin_bridge_ParagraphStyle_setTextAlign(skia::textlayout::ParagraphStyle* style, int32_t align);

size_t skialin_bridge_ParagraphStyle_getMaxLines(const skia::textlayout::ParagraphStyle* style);
void skialin_bridge_ParagraphStyle_setMaxLines(skia::textlayout::ParagraphStyle* style, size_t maxLines);

/* UTF-8 bytes, no NUL terminator. Ref-owned by the caller; free with skialin_bridge_Data_unref. */
SkData* skialin_bridge_ParagraphStyle_getEllipsis(const skia::textlayout::ParagraphStyle* style);
void skialin_bridge_ParagraphStyle_setEllipsis(skia::textlayout::ParagraphStyle* style, const char* ellipsis, size_t length);

float skialin_bridge_ParagraphStyle_getHeight(const skia::textlayout::ParagraphStyle* style);
void skialin_bridge_ParagraphStyle_setHeight(skia::textlayout::ParagraphStyle* style, float height);
int32_t skialin_bridge_ParagraphStyle_getTextHeightBehavior(const skia::textlayout::ParagraphStyle* style);
void skialin_bridge_ParagraphStyle_setTextHeightBehavior(skia::textlayout::ParagraphStyle* style, int32_t behavior);

/* Owned by the caller; free with skialin_bridge_TextStyle_delete. */
skia::textlayout::TextStyle* skialin_bridge_ParagraphStyle_getTextStyle(const skia::textlayout::ParagraphStyle* style);
/* style is copied, not consumed; the caller retains ownership of it. */
void skialin_bridge_ParagraphStyle_setTextStyle(skia::textlayout::ParagraphStyle* paragraphStyle, const skia::textlayout::TextStyle* style);

/* FontCollection: ref-owned by the caller. Free with
 * skialin_bridge_FontCollection_unref. Resolves families named in TextStyle
 * to a Typeface during layout; setDefaultFontManager is the minimum needed
 * to get real glyphs (usually FontMgr::system()). setAssetFontManager/
 * setDynamicFontManager/setTestFontManager are not yet bound. */
skia::textlayout::FontCollection* skialin_bridge_FontCollection_new(void);
void skialin_bridge_FontCollection_unref(skia::textlayout::FontCollection* collection);
/* fontManager is ref'd by the bridge, not consumed. */
void skialin_bridge_FontCollection_setDefaultFontManager(skia::textlayout::FontCollection* collection, SkFontMgr* fontManager);

/* ParagraphBuilder: owned by the caller. Free with
 * skialin_bridge_ParagraphBuilder_delete. Both ParagraphBuilder and
 * Paragraph are abstract (pure virtual layout()/paint()/etc.), which
 * defeats bindgen's vtable-layout inference the same way SkImage does, so
 * every method is routed through the bridge. The bridge owns picking the
 * SkUnicode implementation (ICU, via SkUnicodes::ICU::Make(), matching the
 * icu/harfbuzz build enabled for this shim) so callers never need to touch
 * SkUnicode directly. text is UTF-8. */
skia::textlayout::ParagraphBuilder* skialin_bridge_ParagraphBuilder_make(const skia::textlayout::ParagraphStyle* style, skia::textlayout::FontCollection* fontCollection);
void skialin_bridge_ParagraphBuilder_delete(skia::textlayout::ParagraphBuilder* builder);
/* style is copied, not consumed; the caller retains ownership of it. */
void skialin_bridge_ParagraphBuilder_pushStyle(skia::textlayout::ParagraphBuilder* builder, const skia::textlayout::TextStyle* style);
void skialin_bridge_ParagraphBuilder_pop(skia::textlayout::ParagraphBuilder* builder);
void skialin_bridge_ParagraphBuilder_addText(skia::textlayout::ParagraphBuilder* builder, const char* text, size_t length);
/* Owned by the caller. Free with skialin_bridge_Paragraph_delete. Consumes the builder's accumulated state but not the builder object itself, which remains usable (matching the real API). */
skia::textlayout::Paragraph* skialin_bridge_ParagraphBuilder_build(skia::textlayout::ParagraphBuilder* builder);

/* Paragraph: owned by the caller. Free with skialin_bridge_Paragraph_delete. */
void skialin_bridge_Paragraph_delete(skia::textlayout::Paragraph* paragraph);
void skialin_bridge_Paragraph_layout(skia::textlayout::Paragraph* paragraph, float width);
void skialin_bridge_Paragraph_paint(skia::textlayout::Paragraph* paragraph, SkCanvas* canvas, float x, float y);
float skialin_bridge_Paragraph_getMaxWidth(const skia::textlayout::Paragraph* paragraph);
float skialin_bridge_Paragraph_getHeight(const skia::textlayout::Paragraph* paragraph);
float skialin_bridge_Paragraph_getMinIntrinsicWidth(const skia::textlayout::Paragraph* paragraph);
float skialin_bridge_Paragraph_getMaxIntrinsicWidth(const skia::textlayout::Paragraph* paragraph);
float skialin_bridge_Paragraph_getAlphabeticBaseline(const skia::textlayout::Paragraph* paragraph);
float skialin_bridge_Paragraph_getIdeographicBaseline(const skia::textlayout::Paragraph* paragraph);
float skialin_bridge_Paragraph_getLongestLine(const skia::textlayout::Paragraph* paragraph);
bool skialin_bridge_Paragraph_didExceedMaxLines(const skia::textlayout::Paragraph* paragraph);
size_t skialin_bridge_Paragraph_lineNumber(skia::textlayout::Paragraph* paragraph);
/* -1 if not applicable (not shaped yet). */
int32_t skialin_bridge_Paragraph_unresolvedGlyphs(skia::textlayout::Paragraph* paragraph);

/* affinity: 0 = upstream, 1 = downstream. */
int32_t skialin_bridge_Paragraph_getGlyphPositionAtCoordinate(skia::textlayout::Paragraph* paragraph, float dx, float dy, int32_t* affinity);
/* [start, end) of the word containing the glyph at offset. */
void skialin_bridge_Paragraph_getWordBoundary(skia::textlayout::Paragraph* paragraph, uint32_t offset, size_t* start, size_t* end);

/* Line metrics, matching skia::textlayout::LineMetrics (Metrics.h); the
 * per-run fLineMetrics map is not exposed. hardBreak: 0/1. Returns false
 * (leaving outputs unset) if lineNumber is out of range. */
bool skialin_bridge_Paragraph_getLineMetricsAt(
    skia::textlayout::Paragraph* paragraph, int32_t lineNumber,
    size_t* startIndex, size_t* endIndex, size_t* endExcludingWhitespaces, size_t* endIncludingNewline, int32_t* hardBreak,
    double* ascent, double* descent, double* unscaledAscent, double* height, double* width, double* left, double* baseline);

/* ColorFilter: ref-owned by the caller. Free with skialin_bridge_ColorFilter_unref.
 * Routed entirely through the bridge, not direct bindgen calls: bindgen
 * doesn't generate instance methods for SkColorFilter (its SkFlattenable
 * base defeats its vtable-layout inference, same as SkShader). Scoped to
 * the common factories: Blend/Matrix/Compose/Lerp. HSLAMatrix, gamma,
 * table, and lighting filters are not yet bound. */
void skialin_bridge_ColorFilter_unref(SkColorFilter* filter);
SkColorFilter* skialin_bridge_ColorFilter_Blend(uint32_t argb, SkBlendMode mode);
/* rowMajor20 is a 4x5 row-major matrix (20 floats). */
SkColorFilter* skialin_bridge_ColorFilter_Matrix(const float* rowMajor20, bool clamp);
SkColorFilter* skialin_bridge_ColorFilter_Compose(SkColorFilter* outer, SkColorFilter* inner);
SkColorFilter* skialin_bridge_ColorFilter_Lerp(float t, SkColorFilter* dst, SkColorFilter* src);

/* ImageFilter: ref-owned by the caller. Free with skialin_bridge_ImageFilter_unref.
 * Routed entirely through the bridge for the same reason as SkColorFilter
 * (SkFlattenable base). Scoped to the common factories from
 * include/effects/SkImageFilters.h; Arithmetic, Crop, DisplacementMap,
 * Image, Magnifier, MatrixConvolution, Merge, Picture, RuntimeShader,
 * Shader, Tile, and the lighting filters are not yet bound. `input` may be
 * null to use the source bitmap. */
void skialin_bridge_ImageFilter_unref(SkImageFilter* filter);
SkImageFilter* skialin_bridge_ImageFilter_Blur(float sigmaX, float sigmaY, SkTileMode tileMode, SkImageFilter* input);
SkImageFilter* skialin_bridge_ImageFilter_DropShadow(float dx, float dy, float sigmaX, float sigmaY, uint32_t color, SkImageFilter* input);
SkImageFilter* skialin_bridge_ImageFilter_DropShadowOnly(float dx, float dy, float sigmaX, float sigmaY, uint32_t color, SkImageFilter* input);
SkImageFilter* skialin_bridge_ImageFilter_Offset(float dx, float dy, SkImageFilter* input);
SkImageFilter* skialin_bridge_ImageFilter_ColorFilter(SkColorFilter* cf, SkImageFilter* input);
SkImageFilter* skialin_bridge_ImageFilter_Compose(SkImageFilter* outer, SkImageFilter* inner);
SkImageFilter* skialin_bridge_ImageFilter_MatrixTransform(
    const SkMatrix* matrix, int32_t maxAniso, bool useCubic, float cubicB, float cubicC, SkFilterMode filter, SkMipmapMode mipmap,
    SkImageFilter* input);
SkImageFilter* skialin_bridge_ImageFilter_Dilate(float radiusX, float radiusY, SkImageFilter* input);
SkImageFilter* skialin_bridge_ImageFilter_Erode(float radiusX, float radiusY, SkImageFilter* input);

/* MaskFilter: ref-owned by the caller. Free with skialin_bridge_MaskFilter_unref.
 * Routed entirely through the bridge for the same reason as SkColorFilter
 * (SkFlattenable base). style: 0 = normal, 1 = solid, 2 = outer, 3 = inner
 * (matches SkBlurStyle's declaration order). */
void skialin_bridge_MaskFilter_unref(SkMaskFilter* filter);
SkMaskFilter* skialin_bridge_MaskFilter_MakeBlur(int32_t style, float sigma, bool respectCTM);

/* Attaches a filter to paint; the filter may be null to clear it. */
void skialin_bridge_Paint_setColorFilter(SkPaint* paint, SkColorFilter* filter);
void skialin_bridge_Paint_setImageFilter(SkPaint* paint, SkImageFilter* filter);
void skialin_bridge_Paint_setMaskFilter(SkPaint* paint, SkMaskFilter* filter);

/* Ref-owned by the caller; null if paint has none attached. Free with the
 * matching type's unref (skialin_bridge_Shader_unref etc.). refShader/
 * refColorFilter/refImageFilter/refMaskFilter return sk_sp<T> by value in
 * the real API, which needs the sk_sp ownership seam bridged. */
SkShader* skialin_bridge_Paint_refShader(const SkPaint* paint);
SkColorFilter* skialin_bridge_Paint_refColorFilter(const SkPaint* paint);
SkImageFilter* skialin_bridge_Paint_refImageFilter(const SkPaint* paint);
SkMaskFilter* skialin_bridge_Paint_refMaskFilter(const SkPaint* paint);

}  // extern "C"
