use crate::{color, Canvas, Color, Paint, Path, Picture, PictureRecorder, RRect, Rect, M44};
use std::cell::RefCell;
use std::rc::Rc;

use crate::canvas::ClipOp;

const NON_ZERO_EPSILON: f32 = 0.001;

fn is_zero(value: f32) -> bool {
    value.abs() <= NON_ZERO_EPSILON
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightGeometry {
    /// (x, y, z)
    pub center: (f32, f32, f32),
    pub radius: f32,
}

impl Default for LightGeometry {
    fn default() -> Self {
        LightGeometry { center: (0.0, 0.0, 0.0), radius: 0.0 }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LightInfo {
    pub ambient_shadow_alpha: f32,
    pub spot_shadow_alpha: f32,
}

struct RenderNodeContextState {
    light_geometry: LightGeometry,
    light_info: LightInfo,
}

pub struct RenderNodeContext {
    pub measure_draw_bounds: bool,
    pub snapshot_cache: bool,
    state: RefCell<RenderNodeContextState>,
}

impl RenderNodeContext {
    pub fn new(measure_draw_bounds: bool, snapshot_cache: bool) -> Rc<Self> {
        Rc::new(RenderNodeContext {
            measure_draw_bounds,
            snapshot_cache,
            state: RefCell::new(RenderNodeContextState { light_geometry: LightGeometry::default(), light_info: LightInfo::default() }),
        })
    }

    pub fn light_geometry(&self) -> LightGeometry {
        self.state.borrow().light_geometry
    }

    pub fn light_info(&self) -> LightInfo {
        self.state.borrow().light_info
    }

    pub fn set_lighting_info(&self, light_geometry: LightGeometry, light_info: LightInfo) {
        let mut state = self.state.borrow_mut();
        state.light_geometry = light_geometry;
        state.light_info = light_info;
    }
}

enum ClipShape {
    Rect(Rect),
    RRect(RRect),
    Path(Path),
}

pub struct RenderNode {
    #[allow(dead_code)]
    context: Rc<RenderNodeContext>,
    recorder: PictureRecorder,
    content: Option<Picture>,

    layer_paint: Option<Paint>,
    bounds: Rect,
    pivot: Option<(f32, f32)>,
    alpha: f32,
    scale_x: f32,
    scale_y: f32,
    translation_x: f32,
    translation_y: f32,
    shadow_elevation: f32,
    ambient_shadow_color: Color,
    spot_shadow_color: Color,
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
    camera_distance: f32,

    clip_shape: Option<ClipShape>,
    clip_op: ClipOp,
    clip_antialias: bool,
    clip: bool,
}

impl RenderNode {
    pub fn new(context: Rc<RenderNodeContext>) -> Self {
        RenderNode {
            context,
            recorder: PictureRecorder::new(),
            content: None,
            layer_paint: None,
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            pivot: None,
            alpha: 1.0,
            scale_x: 1.0,
            scale_y: 1.0,
            translation_x: 0.0,
            translation_y: 0.0,
            shadow_elevation: 0.0,
            ambient_shadow_color: color::BLACK,
            spot_shadow_color: color::BLACK,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            camera_distance: 8.0,
            clip_shape: None,
            clip_op: ClipOp::Intersect,
            clip_antialias: false,
            clip: false,
        }
    }

    pub fn layer_paint(&self) -> Option<&Paint> {
        self.layer_paint.as_ref()
    }

    pub fn set_layer_paint(&mut self, layer_paint: Option<Paint>) {
        self.layer_paint = layer_paint;
    }

    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    pub fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    pub fn pivot(&self) -> Option<(f32, f32)> {
        self.pivot
    }

    pub fn set_pivot(&mut self, pivot: Option<(f32, f32)>) {
        self.pivot = pivot;
    }

    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
    }

    pub fn scale_x(&self) -> f32 {
        self.scale_x
    }
    pub fn set_scale_x(&mut self, v: f32) {
        self.scale_x = v;
    }
    pub fn scale_y(&self) -> f32 {
        self.scale_y
    }
    pub fn set_scale_y(&mut self, v: f32) {
        self.scale_y = v;
    }
    pub fn translation_x(&self) -> f32 {
        self.translation_x
    }
    pub fn set_translation_x(&mut self, v: f32) {
        self.translation_x = v;
    }
    pub fn translation_y(&self) -> f32 {
        self.translation_y
    }
    pub fn set_translation_y(&mut self, v: f32) {
        self.translation_y = v;
    }
    pub fn rotation_x(&self) -> f32 {
        self.rotation_x
    }
    pub fn set_rotation_x(&mut self, v: f32) {
        self.rotation_x = v;
    }
    pub fn rotation_y(&self) -> f32 {
        self.rotation_y
    }
    pub fn set_rotation_y(&mut self, v: f32) {
        self.rotation_y = v;
    }
    pub fn rotation_z(&self) -> f32 {
        self.rotation_z
    }
    pub fn set_rotation_z(&mut self, v: f32) {
        self.rotation_z = v;
    }
    pub fn camera_distance(&self) -> f32 {
        self.camera_distance
    }
    pub fn set_camera_distance(&mut self, v: f32) {
        self.camera_distance = v;
    }

    pub fn shadow_elevation(&self) -> f32 {
        self.shadow_elevation
    }
    pub fn set_shadow_elevation(&mut self, v: f32) {
        self.shadow_elevation = v;
    }
    pub fn ambient_shadow_color(&self) -> Color {
        self.ambient_shadow_color
    }
    pub fn set_ambient_shadow_color(&mut self, v: Color) {
        self.ambient_shadow_color = v;
    }
    pub fn spot_shadow_color(&self) -> Color {
        self.spot_shadow_color
    }
    pub fn set_spot_shadow_color(&mut self, v: Color) {
        self.spot_shadow_color = v;
    }

    pub fn set_clip_rect(&mut self, rect: Option<Rect>, op: ClipOp, antialias: bool) {
        self.clip_shape = rect.map(ClipShape::Rect);
        self.clip_op = op;
        self.clip_antialias = antialias;
    }

    pub fn set_clip_rrect(&mut self, rrect: Option<RRect>, op: ClipOp, antialias: bool) {
        self.clip_shape = rrect.map(ClipShape::RRect);
        self.clip_op = op;
        self.clip_antialias = antialias;
    }

    pub fn set_clip_path(&mut self, path: Option<Path>, op: ClipOp, antialias: bool) {
        self.clip_shape = path.map(ClipShape::Path);
        self.clip_op = op;
        self.clip_antialias = antialias;
    }

    pub fn clip(&self) -> bool {
        self.clip
    }
    pub fn set_clip(&mut self, clip: bool) {
        self.clip = clip;
    }

    pub fn begin_recording(&mut self) -> Canvas<'_> {
        self.recorder.begin_recording(Rect::new(0.0, 0.0, self.bounds.width(), self.bounds.height()))
    }

    pub fn end_recording(&mut self) {
        self.content = self.recorder.finish_recording_as_picture();
    }

    fn pivot_or_center(&self) -> (f32, f32) {
        self.pivot.unwrap_or((self.bounds.width() / 2.0, self.bounds.height() / 2.0))
    }

    pub fn matrix(&self) -> M44 {
        let (px, py) = self.pivot_or_center();
        let to_pivot = M44::translate(px, py, 0.0);
        let from_pivot = M44::translate(-px, -py, 0.0);
        let translate = M44::translate(self.translation_x, self.translation_y, 0.0);
        let scale = M44::scale(self.scale_x, self.scale_y, 1.0);

        if is_zero(self.rotation_x) && is_zero(self.rotation_y) {
            let rot = M44::rotate((0.0, 0.0, 1.0), self.rotation_z.to_radians());
            M44::concat(&translate, &M44::concat(&to_pivot, &M44::concat(&rot, &M44::concat(&scale, &from_pivot))))
        } else {
            let rot_x = M44::rotate((1.0, 0.0, 0.0), self.rotation_x.to_radians());
            let rot_y = M44::rotate((0.0, 1.0, 0.0), self.rotation_y.to_radians());
            let rot_z = M44::rotate((0.0, 0.0, 1.0), self.rotation_z.to_radians());
            let rotation = M44::concat(&rot_z, &M44::concat(&rot_y, &rot_x));

            let camera_pt = self.camera_distance.max(0.001) * 72.0;
            let mut persp = M44::identity().to_row_major();
            persp[14] = -1.0 / camera_pt;
            let persp = M44::from_row_major(&persp);

            M44::concat(
                &translate,
                &M44::concat(&to_pivot, &M44::concat(&persp, &M44::concat(&rotation, &M44::concat(&scale, &from_pivot)))),
            )
        }
    }

    fn draw_shadow(&self, canvas: &mut Canvas) {
        let light_geometry = self.context.light_geometry();
        let light_info = self.context.light_info();

        let path = match &self.clip_shape {
            Some(ClipShape::Rect(r)) => {
                let mut b = crate::PathBuilder::new();
                b.add_rect(*r, crate::PathDirection::Clockwise);
                Some(b.detach())
            }
            Some(ClipShape::RRect(rr)) => {
                let mut b = crate::PathBuilder::new();
                b.add_rect(rr.rect(), crate::PathDirection::Clockwise);
                Some(b.detach())
            }
            Some(ClipShape::Path(_)) => None,
            None => return,
        };
        let path_ref = match (&path, &self.clip_shape) {
            (Some(p), _) => p,
            (None, Some(ClipShape::Path(p))) => p,
            _ => return,
        };

        let ambient_alpha = light_info.ambient_shadow_alpha * self.alpha;
        let spot_alpha = light_info.spot_shadow_alpha * self.alpha;
        let ambient_color = multiply_alpha(self.ambient_shadow_color, ambient_alpha);
        let spot_color = multiply_alpha(self.spot_shadow_color, spot_alpha);

        canvas.draw_shadow(
            path_ref,
            (0.0, 0.0, self.shadow_elevation),
            light_geometry.center,
            light_geometry.radius,
            ambient_color,
            spot_color,
            if self.alpha < 1.0 { 1 } else { 0 },
        );
    }

    pub fn draw_into(&mut self, canvas: &mut Canvas) {
        canvas.save();

        canvas.translate(self.bounds.left, self.bounds.top);
        canvas.concat_44(&self.matrix());

        if self.shadow_elevation > 0.0 {
            self.draw_shadow(canvas);
        }

        if self.clip {
            canvas.save();
            match &self.clip_shape {
                Some(ClipShape::Rect(r)) => canvas.clip_rect(*r, self.clip_op, self.clip_antialias),
                Some(ClipShape::RRect(rr)) => canvas.clip_rrect(rr, self.clip_op, self.clip_antialias),
                Some(ClipShape::Path(p)) => canvas.clip_path(p, self.clip_op, self.clip_antialias),
                None => canvas.clip_rect(Rect::new(0.0, 0.0, self.bounds.width(), self.bounds.height()), self.clip_op, self.clip_antialias),
            }
        }

        if let Some(layer_paint) = &self.layer_paint {
            let rect = Rect::new(0.0, 0.0, self.bounds.width(), self.bounds.height());
            canvas.save_layer(Some(rect), Some(layer_paint));
        } else if self.alpha < 1.0 {
            let mut paint = Paint::new();
            paint.set_alphaf(self.alpha);
            canvas.save_layer(None, Some(&paint));
        } else {
            canvas.save();
        }

        if let Some(picture) = &self.content {
            canvas.draw_picture(picture);
        }

        canvas.restore();
        if self.clip {
            canvas.restore();
        }
        canvas.restore();
    }
}

fn multiply_alpha(color: Color, alpha: f32) -> Color {
    let a = ((color >> 24) & 0xff) as f32;
    let new_a = (a * alpha).round().clamp(0.0, 255.0) as u32;
    (color & 0x00ff_ffff) | (new_a << 24)
}
