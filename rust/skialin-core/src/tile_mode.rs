#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TileMode {
    /// Replicates the edge color if the shader draws outside of its original bounds.
    Clamp,
    /// Repeats the shader's image horizontally and vertically.
    Repeat,
    /// Repeats the shader's image, alternating mirror images so adjacent images always seam.
    Mirror,
    /// Only draws within the original domain; transparent-black everywhere else.
    Decal,
}
