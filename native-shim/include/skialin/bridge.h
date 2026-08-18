#pragma once

/* Bindgen binds Skia's C++ classes (SkPaint, SkPath, SkMatrix, SkCanvas, ...)
 * directly from Skia's own headers. This bridge exists only for the seams
 * bindgen cannot cross on its own: factory statics and methods that return
 * or consume sk_sp<T>, since sk_sp is a template and its instantiations
 * are not directly callable through a generated binding. Every function
 * here takes/returns a raw, already-ref-managed pointer so the Rust side
 * owns a plain pointer with clear, explicit lifetime rules. */

#include "include/core/SkImageInfo.h"

class SkSurface;
class SkCanvas;
class SkImage;
class SkData;
class SkBitmap;

extern "C" {

/* Surface: ref-owned by the caller. Free with skialin_bridge_Surface_unref. */
SkSurface* skialin_bridge_Surface_MakeRasterN32Premul(int32_t width, int32_t height);
SkSurface* skialin_bridge_Surface_MakeRaster(const SkImageInfo* info);
void skialin_bridge_Surface_unref(SkSurface* surface);

/* Borrowed for the lifetime of the surface, never freed independently. */
SkCanvas* skialin_bridge_Surface_getCanvas(SkSurface* surface);

/* Ref-owned by the caller. Free with skialin_bridge_Image_unref. */
SkImage* skialin_bridge_Surface_makeImageSnapshot(SkSurface* surface);

/* Image: ref-owned by the caller. Free with skialin_bridge_Image_unref. */
void skialin_bridge_Image_unref(SkImage* image);
SkImage* skialin_bridge_Image_MakeFromEncoded(const uint8_t* bytes, size_t length);
SkData* skialin_bridge_Image_encodeToData(const SkImage* image);

/* Bitmap -> Image conversion; result is ref-owned by the caller. */
SkImage* skialin_bridge_Bitmap_asImage(const SkBitmap* bitmap);

/* Data: ref-owned by the caller. Free with skialin_bridge_Data_unref. */
void skialin_bridge_Data_unref(SkData* data);

}  // extern "C"
