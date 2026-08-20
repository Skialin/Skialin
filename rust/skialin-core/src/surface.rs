use crate::{
    sys, BackendRenderTarget, BackendTexture, Canvas, ColorSpace, ColorType, DirectContext, GraphiteBackendTexture, GraphiteRecorder, Image,
    ImageInfo, SurfaceOrigin, SurfaceProps,
};
use std::marker::PhantomData;

pub struct Surface(pub(crate) *mut sys::SkSurface);

impl Surface {
    pub fn new_raster_n32_premul(width: i32, height: i32) -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_Surface_MakeRasterN32Premul(width, height) };
        (!ptr.is_null()).then_some(Surface(ptr))
    }

    pub fn new_raster(info: &ImageInfo) -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_Surface_MakeRaster(info.0) };
        (!ptr.is_null()).then_some(Surface(ptr))
    }

    /// Must be called on the thread `context`'s GL context is current on.
    #[allow(clippy::too_many_arguments)]
    pub fn new_render_target(
        context: &mut DirectContext,
        budgeted: bool,
        info: &ImageInfo,
        sample_count: i32,
        surface_origin: SurfaceOrigin,
        surface_props: Option<&SurfaceProps>,
        should_create_with_mips: bool,
        is_protected: bool,
    ) -> Option<Self> {
        let props_ptr = surface_props.map_or(std::ptr::null(), |props| props.0 as *const _);
        let ptr = unsafe {
            sys::skialin_bridge_Surface_MakeRenderTarget(
                context.0,
                budgeted,
                info.0,
                sample_count,
                surface_origin.into(),
                props_ptr,
                should_create_with_mips,
                is_protected,
            )
        };
        (!ptr.is_null()).then_some(Surface(ptr))
    }

    /// Wraps an existing GPU texture (e.g. `BackendTexture::new_vk`) as a
    /// render target instead of allocating a new one. Must be called on
    /// the thread `context`'s GL context is current on. `backend_texture`
    /// must outlive the returned Surface.
    #[allow(clippy::too_many_arguments)]
    pub fn wrap_backend_texture(
        context: &mut DirectContext,
        backend_texture: &BackendTexture,
        origin: SurfaceOrigin,
        sample_count: i32,
        color_type: ColorType,
        color_space: Option<&ColorSpace>,
        surface_props: Option<&SurfaceProps>,
    ) -> Option<Self> {
        let color_space_ptr = color_space.map_or(std::ptr::null_mut(), |cs| cs.0);
        let props_ptr = surface_props.map_or(std::ptr::null(), |props| props.0 as *const _);
        let ptr = unsafe {
            sys::skialin_bridge_Surface_WrapBackendTexture(
                context.0,
                backend_texture.0,
                origin.into(),
                sample_count,
                color_type.into(),
                color_space_ptr,
                props_ptr,
                None,
                std::ptr::null_mut(),
            )
        };
        (!ptr.is_null()).then_some(Surface(ptr))
    }

    /// Wraps an existing GPU render target (e.g. `BackendRenderTarget::new_gl`,
    /// for the window-system framebuffer or a renderbuffer that isn't a
    /// sampleable texture) as a Surface instead of allocating a new one.
    /// Must be called on the thread `context`'s GL context is current on.
    /// `backend_render_target` must outlive the returned Surface.
    pub fn wrap_backend_render_target(
        context: &mut DirectContext,
        backend_render_target: &BackendRenderTarget,
        origin: SurfaceOrigin,
        color_type: ColorType,
        color_space: Option<&ColorSpace>,
        surface_props: Option<&SurfaceProps>,
    ) -> Option<Self> {
        let color_space_ptr = color_space.map_or(std::ptr::null_mut(), |cs| cs.0);
        let props_ptr = surface_props.map_or(std::ptr::null(), |props| props.0 as *const _);
        let ptr = unsafe {
            sys::skialin_bridge_Surface_WrapBackendRenderTarget(
                context.0,
                backend_render_target.0,
                origin.into(),
                color_type.into(),
                color_space_ptr,
                props_ptr,
                None,
                std::ptr::null_mut(),
            )
        };
        (!ptr.is_null()).then_some(Surface(ptr))
    }

    pub fn new_graphite_render_target(recorder: &mut GraphiteRecorder, info: &ImageInfo, mipmapped: bool, surface_props: Option<&SurfaceProps>) -> Option<Self> {
        let props_ptr = surface_props.map_or(std::ptr::null(), |props| props.0 as *const _);
        let ptr = unsafe { sys::skialin_bridge_GraphiteSurface_MakeRenderTarget(recorder.0, info.0, mipmapped, props_ptr) };
        (!ptr.is_null()).then_some(Surface(ptr))
    }

    /// `backend_texture` must outlive the returned Surface.
    pub fn wrap_graphite_backend_texture(
        recorder: &mut GraphiteRecorder,
        backend_texture: &GraphiteBackendTexture,
        color_type: ColorType,
        color_space: Option<&ColorSpace>,
        surface_props: Option<&SurfaceProps>,
    ) -> Option<Self> {
        let color_space_ptr = color_space.map_or(std::ptr::null_mut(), |cs| cs.0);
        let props_ptr = surface_props.map_or(std::ptr::null(), |props| props.0 as *const _);
        let ptr = unsafe {
            sys::skialin_bridge_GraphiteSurface_WrapBackendTexture(recorder.0, backend_texture.0, color_type.into(), color_space_ptr, props_ptr)
        };
        (!ptr.is_null()).then_some(Surface(ptr))
    }

    pub fn canvas(&mut self) -> Canvas<'_> {
        let ptr = unsafe { sys::skialin_bridge_Surface_getCanvas(self.0) };
        Canvas { ptr, _marker: PhantomData }
    }

    pub fn image_snapshot(&mut self) -> Option<Image> {
        let ptr = unsafe { sys::skialin_bridge_Surface_makeImageSnapshot(self.0) };
        (!ptr.is_null()).then(|| Image(ptr))
    }

    pub fn image_snapshot_area(&mut self, bounds: crate::IRect) -> Option<Image> {
        let sk_bounds: sys::SkIRect = bounds.into();
        let ptr = unsafe { sys::skialin_bridge_Surface_makeImageSnapshotArea(self.0, &sk_bounds) };
        (!ptr.is_null()).then(|| Image(ptr))
    }

    pub fn width(&mut self) -> i32 {
        self.image_info().width()
    }

    pub fn height(&mut self) -> i32 {
        self.image_info().height()
    }

    pub fn image_info(&mut self) -> ImageInfo {
        unsafe { ImageInfo::from_raw(sys::skialin_bridge_Surface_imageInfo(self.0)) }
    }

    pub fn notify_content_will_change(&mut self, mode: i32) {
        unsafe { sys::skialin_bridge_Surface_notifyContentWillChange(self.0, mode) };
    }

    pub fn flush(&mut self) {
        unsafe { sys::skialin_bridge_Surface_flush(self.0) };
    }

    pub fn draw(&mut self, canvas: &mut Canvas, x: f32, y: f32, paint: Option<&crate::Paint>) {
        let paint_ptr = paint.map_or(std::ptr::null(), |p| &*p.0 as *const sys::SkPaint);
        unsafe { sys::skialin_bridge_Surface_draw(self.0, canvas.ptr, x, y, paint_ptr) };
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Surface_unref(self.0) };
    }
}
