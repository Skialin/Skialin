use crate::{sys, AlphaType, Bitmap, ColorSpace, ColorType, Data, FilterMode, IRect, ISize, ImageInfo, Matrix, MipmapMode, Pixmap, SamplingOptions, Shader, TileMode};

impl From<TileMode> for sys::SkTileMode {
    fn from(mode: TileMode) -> Self {
        (match mode {
            TileMode::Clamp => sys::SkTileMode_kClamp,
            TileMode::Repeat => sys::SkTileMode_kRepeat,
            TileMode::Mirror => sys::SkTileMode_kMirror,
            TileMode::Decal => sys::SkTileMode_kDecal,
        }) as sys::SkTileMode
    }
}

impl From<FilterMode> for sys::SkFilterMode {
    fn from(mode: FilterMode) -> Self {
        (match mode {
            FilterMode::Nearest => sys::SkFilterMode_kNearest,
            FilterMode::Linear => sys::SkFilterMode_kLinear,
        }) as sys::SkFilterMode
    }
}

impl From<MipmapMode> for sys::SkMipmapMode {
    fn from(mode: MipmapMode) -> Self {
        (match mode {
            MipmapMode::None => sys::SkMipmapMode_kNone,
            MipmapMode::Nearest => sys::SkMipmapMode_kNearest,
            MipmapMode::Linear => sys::SkMipmapMode_kLinear,
        }) as sys::SkMipmapMode
    }
}

pub struct Image(pub(crate) *mut sys::SkImage);

impl Image {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkImage) -> Option<Self> {
        (!ptr.is_null()).then_some(Image(ptr))
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_Image_MakeFromEncoded(bytes.as_ptr(), bytes.len())) }
    }

    /// A deep copy of `pixmap`'s pixels.
    pub fn from_pixmap_copy(pixmap: &Pixmap) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_Image_RasterFromPixmapCopy(pixmap.as_raw())) }
    }

    /// `pixels`' bytes become this image's pixel storage; no copy is made.
    /// `pixels` is ref'd by the bridge (matching `sk_sp` semantics), not
    /// consumed: it stays independently valid and closeable afterward.
    pub fn from_data(info: &ImageInfo, pixels: &Data, row_bytes: usize) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_Image_RasterFromData(info.0, pixels.0, row_bytes)) }
    }

    pub fn width(&self) -> i32 {
        unsafe { sys::skialin_bridge_Image_width(self.0) }
    }

    pub fn height(&self) -> i32 {
        unsafe { sys::skialin_bridge_Image_height(self.0) }
    }

    pub fn dimensions(&self) -> ISize {
        ISize::new(self.width(), self.height())
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_wh(self.width(), self.height())
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { sys::skialin_bridge_Image_uniqueID(self.0) }
    }

    pub fn alpha_type(&self) -> AlphaType {
        unsafe { sys::skialin_bridge_Image_alphaType(self.0) }.into()
    }

    pub fn color_type(&self) -> ColorType {
        unsafe { sys::skialin_bridge_Image_colorType(self.0) }.into()
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        unsafe { ColorSpace::from_raw(sys::skialin_bridge_Image_refColorSpace(self.0)) }
    }

    pub fn image_info(&self) -> ImageInfo {
        unsafe { ImageInfo::from_raw(sys::skialin_bridge_Image_imageInfo(self.0)) }
    }

    pub fn is_alpha_only(&self) -> bool {
        unsafe { sys::skialin_bridge_Image_isAlphaOnly(self.0) }
    }

    pub fn is_opaque(&self) -> bool {
        unsafe { sys::skialin_bridge_Image_isOpaque(self.0) }
    }

    pub fn is_texture_backed(&self) -> bool {
        unsafe { sys::skialin_bridge_Image_isTextureBacked(self.0) }
    }

    pub fn is_lazy_generated(&self) -> bool {
        unsafe { sys::skialin_bridge_Image_isLazyGenerated(self.0) }
    }

    pub fn has_mipmaps(&self) -> bool {
        unsafe { sys::skialin_bridge_Image_hasMipmaps(self.0) }
    }

    pub fn is_protected(&self) -> bool {
        unsafe { sys::skialin_bridge_Image_isProtected(self.0) }
    }

    pub fn make_shader(&self, tile_x: TileMode, tile_y: TileMode, sampling: SamplingOptions, local_matrix: Option<&Matrix>) -> Option<Shader> {
        let matrix_ptr = local_matrix.map_or(std::ptr::null(), |m| &m.0);
        let (cubic_b, cubic_c) = sampling.cubic.unwrap_or((0.0, 0.0));
        unsafe {
            Shader::from_raw(sys::skialin_bridge_Image_makeShader(
                self.0,
                tile_x.into(),
                tile_y.into(),
                sampling.max_aniso,
                sampling.cubic.is_some(),
                cubic_b,
                cubic_c,
                sampling.filter.into(),
                sampling.mipmap.into(),
                matrix_ptr,
            ))
        }
    }

    pub fn make_raw_shader(&self, tile_x: TileMode, tile_y: TileMode, sampling: SamplingOptions, local_matrix: Option<&Matrix>) -> Option<Shader> {
        let matrix_ptr = local_matrix.map_or(std::ptr::null(), |m| &m.0);
        let (cubic_b, cubic_c) = sampling.cubic.unwrap_or((0.0, 0.0));
        unsafe {
            Shader::from_raw(sys::skialin_bridge_Image_makeRawShader(
                self.0,
                tile_x.into(),
                tile_y.into(),
                sampling.max_aniso,
                sampling.cubic.is_some(),
                cubic_b,
                cubic_c,
                sampling.filter.into(),
                sampling.mipmap.into(),
                matrix_ptr,
            ))
        }
    }

    /// A [`Pixmap`] view over this image's pixels, if it has direct CPU
    /// access to them.
    pub fn peek_pixels(&self) -> Option<Pixmap> {
        let pixmap = Pixmap::empty();
        let ok = unsafe { sys::skialin_bridge_Image_peekPixels(self.0, pixmap.as_raw()) };
        ok.then_some(pixmap)
    }

    /// # Safety
    /// `dst_pixels` must be valid for `dst_row_bytes * dst_info.height()` bytes.
    pub unsafe fn read_pixels(&self, dst_info: &ImageInfo, dst_pixels: *mut u8, dst_row_bytes: usize, src_x: i32, src_y: i32) -> bool {
        sys::skialin_bridge_Image_readPixels(self.0, dst_info.0, dst_pixels.cast(), dst_row_bytes, src_x, src_y)
    }

    pub fn scale_pixels(&self, dst: &mut Pixmap, sampling: SamplingOptions) -> bool {
        let (cubic_b, cubic_c) = sampling.cubic.unwrap_or((0.0, 0.0));
        unsafe {
            sys::skialin_bridge_Image_scalePixels(
                self.0,
                dst.as_raw(),
                sampling.max_aniso,
                sampling.cubic.is_some(),
                cubic_b,
                cubic_c,
                sampling.filter.into(),
                sampling.mipmap.into(),
            )
        }
    }

    pub fn make_scaled(&self, info: &ImageInfo, sampling: SamplingOptions) -> Option<Image> {
        let (cubic_b, cubic_c) = sampling.cubic.unwrap_or((0.0, 0.0));
        unsafe {
            Self::from_raw(sys::skialin_bridge_Image_makeScaled(
                self.0,
                info.0,
                sampling.max_aniso,
                sampling.cubic.is_some(),
                cubic_b,
                cubic_c,
                sampling.filter.into(),
                sampling.mipmap.into(),
            ))
        }
    }

    /// The original encoded bytes, if this image was created from an
    /// encoded stream (e.g. via [`Image::decode`]).
    pub fn ref_encoded_data(&self) -> Option<Data> {
        unsafe { Data::from_raw(sys::skialin_bridge_Image_refEncodedData(self.0)) }
    }

    pub fn encode_to_data(&self) -> Option<Data> {
        unsafe { Data::from_raw(sys::skialin_bridge_Image_encodeToData(self.0)) }
    }

    pub fn encode_to_png(&self) -> Option<Vec<u8>> {
        self.encode_to_data().map(|data| data.as_bytes().to_vec())
    }

    pub fn make_subset(&self, subset: IRect, mipmapped: bool) -> Option<Image> {
        unsafe { Self::from_raw(sys::skialin_bridge_Image_makeSubset(self.0, subset.left, subset.top, subset.right, subset.bottom, mipmapped)) }
    }

    pub fn with_default_mipmaps(&self) -> Option<Image> {
        unsafe { Self::from_raw(sys::skialin_bridge_Image_withDefaultMipmaps(self.0)) }
    }

    pub fn make_non_texture_image(&self) -> Option<Image> {
        unsafe { Self::from_raw(sys::skialin_bridge_Image_makeNonTextureImage(self.0)) }
    }

    pub fn make_raster_image(&self, allow_caching: bool) -> Option<Image> {
        unsafe { Self::from_raw(sys::skialin_bridge_Image_makeRasterImage(self.0, allow_caching)) }
    }

    pub fn as_legacy_bitmap(&self) -> Option<Bitmap> {
        let mut bitmap = Bitmap::new();
        let ok = unsafe { sys::skialin_bridge_Image_asLegacyBitmap(self.0, bitmap.as_raw_mut()) };
        ok.then_some(bitmap)
    }

    pub fn make_color_space(&self, target: &ColorSpace, mipmapped: bool) -> Option<Image> {
        unsafe { Self::from_raw(sys::skialin_bridge_Image_makeColorSpace(self.0, target.0, mipmapped)) }
    }

    pub fn make_color_type_and_color_space(&self, target_color_type: ColorType, target_color_space: &ColorSpace, mipmapped: bool) -> Option<Image> {
        unsafe { Self::from_raw(sys::skialin_bridge_Image_makeColorTypeAndColorSpace(self.0, target_color_type.into(), target_color_space.0, mipmapped)) }
    }

    pub fn reinterpret_color_space(&self, new_color_space: &ColorSpace) -> Option<Image> {
        unsafe { Self::from_raw(sys::skialin_bridge_Image_reinterpretColorSpace(self.0, new_color_space.0)) }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Image_unref(self.0) };
    }
}
