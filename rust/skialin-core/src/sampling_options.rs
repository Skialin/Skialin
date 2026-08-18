use crate::{FilterMode, MipmapMode};

/// Mirrors Skia's `SkSamplingOptions`. Passed by value to shader/draw calls
/// that need it; never stored as a native pointer since it's a plain POD
/// struct with no `sk_sp` members.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SamplingOptions {
    pub max_aniso: i32,
    pub cubic: Option<(f32, f32)>,
    pub filter: FilterMode,
    pub mipmap: MipmapMode,
}

impl SamplingOptions {
    pub const fn new(filter: FilterMode, mipmap: MipmapMode) -> Self {
        SamplingOptions { max_aniso: 0, cubic: None, filter, mipmap }
    }

    pub const fn nearest() -> Self {
        Self::new(FilterMode::Nearest, MipmapMode::None)
    }

    pub const fn linear() -> Self {
        Self::new(FilterMode::Linear, MipmapMode::None)
    }

    pub const fn cubic(b: f32, c: f32) -> Self {
        SamplingOptions { max_aniso: 0, cubic: Some((b, c)), filter: FilterMode::Nearest, mipmap: MipmapMode::None }
    }

    pub const fn mitchell() -> Self {
        Self::cubic(1.0 / 3.0, 1.0 / 3.0)
    }

    pub const fn catmull_rom() -> Self {
        Self::cubic(0.0, 1.0 / 2.0)
    }

    pub fn aniso(max_aniso: i32) -> Self {
        SamplingOptions { max_aniso: max_aniso.max(1), ..Self::nearest() }
    }

    pub fn is_aniso(&self) -> bool {
        self.max_aniso != 0
    }
}

impl Default for SamplingOptions {
    fn default() -> Self {
        Self::nearest()
    }
}
