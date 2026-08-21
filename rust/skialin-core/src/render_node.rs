use crate::{sys, Canvas, ClipOp, Paint, Path, RRect, Rect};

/// A thin wrapper around the C++ `skialin::node::RenderNodeContext` (native-shim/src/node/).
/// See `RenderNode` for why the actual recording/drawing logic lives in C++ rather than here.
pub struct RenderNodeContext(pub(crate) *mut sys::skialin::node::RenderNodeContext);

impl RenderNodeContext {
    pub fn new(measure_draw_bounds: bool, snapshot_cache: bool) -> Self {
        let ptr = unsafe { sys::skialin_bridge_RenderNodeContext_Make(measure_draw_bounds, snapshot_cache) };
        RenderNodeContext(ptr)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_lighting_info(
        &mut self,
        center_x: f32,
        center_y: f32,
        center_z: f32,
        radius: f32,
        ambient_shadow_alpha: f32,
        spot_shadow_alpha: f32,
    ) {
        unsafe {
            sys::skialin_bridge_RenderNodeContext_setLightingInfo(
                self.0,
                center_x,
                center_y,
                center_z,
                radius,
                ambient_shadow_alpha,
                spot_shadow_alpha,
            );
        }
    }
}

impl Drop for RenderNodeContext {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_RenderNodeContext_unref(self.0) };
    }
}

/// A thin wrapper around the C++ `skialin::node::RenderNode` (native-shim/src/node/RenderNode.cpp),
/// itself a port of skiko's `org.jetbrains.skiko.node.RenderNode`. This has to be a real
/// `SkDrawable` subclass living in C++ rather than a plain Rust struct recording an `SkPicture`:
/// when one RenderNode's recording embeds another (a nested Compose GraphicsLayer), the embedded
/// node must stay a *live* reference -- `SkCanvas::drawDrawable` replays the referenced
/// `SkDrawable::draw()` fresh on every playback -- rather than a frozen snapshot of whatever it
/// looked like at record time. A plain `SkPicture`-of-a-`SkPicture` would bake the child's content
/// in as of record time, so a parent that stops re-recording (the whole point of the cache) would
/// never show that child's later updates. See RenderNode.h in native-shim for the longer version.
pub struct RenderNode(pub(crate) *mut sys::skialin::node::RenderNode);

impl RenderNode {
    pub fn new(context: &RenderNodeContext) -> Self {
        let ptr = unsafe { sys::skialin_bridge_RenderNode_Make(context.0) };
        RenderNode(ptr)
    }

    pub fn layer_paint(&self) -> Option<Paint> {
        let mut paint = Paint::new();
        let has_value = unsafe { sys::skialin_bridge_RenderNode_getLayerPaint(self.0, &mut *paint.0) };
        has_value.then_some(paint)
    }

    pub fn set_layer_paint(&mut self, paint: Option<&Paint>) {
        unsafe {
            sys::skialin_bridge_RenderNode_setLayerPaint(
                self.0,
                paint.map_or(std::ptr::null(), |p| &*p.0 as *const sys::SkPaint),
            );
        }
    }

    pub fn bounds(&self) -> Rect {
        let mut sk_rect = sys::SkRect::default();
        unsafe { sys::skialin_bridge_RenderNode_getBounds(self.0, &mut sk_rect) };
        sk_rect.into()
    }

    pub fn set_bounds(&mut self, bounds: Rect) {
        let sk_rect: sys::SkRect = bounds.into();
        unsafe { sys::skialin_bridge_RenderNode_setBounds(self.0, &sk_rect) };
    }

    /// `None` mirrors an unset (NaN) pivot, i.e. "use the bounds' center".
    pub fn pivot(&self) -> Option<(f32, f32)> {
        let mut sk_point = sys::SkPoint::default();
        unsafe { sys::skialin_bridge_RenderNode_getPivot(self.0, &mut sk_point) };
        (!sk_point.fX.is_nan()).then_some((sk_point.fX, sk_point.fY))
    }

    pub fn set_pivot(&mut self, pivot: Option<(f32, f32)>) {
        let (x, y) = pivot.unwrap_or((f32::NAN, f32::NAN));
        unsafe { sys::skialin_bridge_RenderNode_setPivot(self.0, x, y) };
    }

    pub fn alpha(&self) -> f32 {
        unsafe { sys::skialin_bridge_RenderNode_getAlpha(self.0) }
    }
    pub fn set_alpha(&mut self, value: f32) {
        unsafe { sys::skialin_bridge_RenderNode_setAlpha(self.0, value) };
    }

    pub fn scale_x(&self) -> f32 {
        unsafe { sys::skialin_bridge_RenderNode_getScaleX(self.0) }
    }
    pub fn set_scale_x(&mut self, value: f32) {
        unsafe { sys::skialin_bridge_RenderNode_setScaleX(self.0, value) };
    }

    pub fn scale_y(&self) -> f32 {
        unsafe { sys::skialin_bridge_RenderNode_getScaleY(self.0) }
    }
    pub fn set_scale_y(&mut self, value: f32) {
        unsafe { sys::skialin_bridge_RenderNode_setScaleY(self.0, value) };
    }

    pub fn translation_x(&self) -> f32 {
        unsafe { sys::skialin_bridge_RenderNode_getTranslationX(self.0) }
    }
    pub fn set_translation_x(&mut self, value: f32) {
        unsafe { sys::skialin_bridge_RenderNode_setTranslationX(self.0, value) };
    }

    pub fn translation_y(&self) -> f32 {
        unsafe { sys::skialin_bridge_RenderNode_getTranslationY(self.0) }
    }
    pub fn set_translation_y(&mut self, value: f32) {
        unsafe { sys::skialin_bridge_RenderNode_setTranslationY(self.0, value) };
    }

    pub fn rotation_x(&self) -> f32 {
        unsafe { sys::skialin_bridge_RenderNode_getRotationX(self.0) }
    }
    pub fn set_rotation_x(&mut self, value: f32) {
        unsafe { sys::skialin_bridge_RenderNode_setRotationX(self.0, value) };
    }

    pub fn rotation_y(&self) -> f32 {
        unsafe { sys::skialin_bridge_RenderNode_getRotationY(self.0) }
    }
    pub fn set_rotation_y(&mut self, value: f32) {
        unsafe { sys::skialin_bridge_RenderNode_setRotationY(self.0, value) };
    }

    pub fn rotation_z(&self) -> f32 {
        unsafe { sys::skialin_bridge_RenderNode_getRotationZ(self.0) }
    }
    pub fn set_rotation_z(&mut self, value: f32) {
        unsafe { sys::skialin_bridge_RenderNode_setRotationZ(self.0, value) };
    }

    pub fn camera_distance(&self) -> f32 {
        unsafe { sys::skialin_bridge_RenderNode_getCameraDistance(self.0) }
    }
    pub fn set_camera_distance(&mut self, value: f32) {
        unsafe { sys::skialin_bridge_RenderNode_setCameraDistance(self.0, value) };
    }

    pub fn shadow_elevation(&self) -> f32 {
        unsafe { sys::skialin_bridge_RenderNode_getShadowElevation(self.0) }
    }
    pub fn set_shadow_elevation(&mut self, value: f32) {
        unsafe { sys::skialin_bridge_RenderNode_setShadowElevation(self.0, value) };
    }

    pub fn ambient_shadow_color(&self) -> u32 {
        unsafe { sys::skialin_bridge_RenderNode_getAmbientShadowColor(self.0) }
    }
    pub fn set_ambient_shadow_color(&mut self, value: u32) {
        unsafe { sys::skialin_bridge_RenderNode_setAmbientShadowColor(self.0, value) };
    }

    pub fn spot_shadow_color(&self) -> u32 {
        unsafe { sys::skialin_bridge_RenderNode_getSpotShadowColor(self.0) }
    }
    pub fn set_spot_shadow_color(&mut self, value: u32) {
        unsafe { sys::skialin_bridge_RenderNode_setSpotShadowColor(self.0, value) };
    }

    pub fn clip(&self) -> bool {
        unsafe { sys::skialin_bridge_RenderNode_getClip(self.0) }
    }
    pub fn set_clip(&mut self, clip: bool) {
        unsafe { sys::skialin_bridge_RenderNode_setClip(self.0, clip) };
    }

    pub fn set_clip_rect(&mut self, rect: Option<Rect>, op: ClipOp, antialias: bool) {
        let sk_rect: Option<sys::SkRect> = rect.map(Into::into);
        unsafe {
            sys::skialin_bridge_RenderNode_setClipRect(
                self.0,
                sk_rect.as_ref().map_or(std::ptr::null(), |r| r as *const _),
                op.into(),
                antialias,
            );
        }
    }

    pub fn set_clip_rrect(&mut self, rrect: Option<&RRect>, op: ClipOp, antialias: bool) {
        unsafe {
            sys::skialin_bridge_RenderNode_setClipRRect(
                self.0,
                rrect.map_or(std::ptr::null(), |r| r.0),
                op.into(),
                antialias,
            );
        }
    }

    pub fn set_clip_path(&mut self, path: Option<&Path>, op: ClipOp, antialias: bool) {
        unsafe {
            sys::skialin_bridge_RenderNode_setClipPath(
                self.0,
                path.map_or(std::ptr::null(), |p| p.0),
                op.into(),
                antialias,
            );
        }
    }

    /// Borrowed: valid only until the matching `end_recording`.
    pub fn begin_recording(&mut self) -> Canvas<'_> {
        let ptr = unsafe { sys::skialin_bridge_RenderNode_beginRecording(self.0) };
        unsafe { Canvas::from_raw(ptr) }
    }

    pub fn end_recording(&mut self) {
        unsafe { sys::skialin_bridge_RenderNode_endRecording(self.0) };
    }

    pub fn draw_into(&mut self, canvas: &mut Canvas) {
        unsafe { sys::skialin_bridge_RenderNode_drawInto(self.0, canvas.as_raw()) };
    }
}

impl Drop for RenderNode {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_RenderNode_unref(self.0) };
    }
}
