/*!
 Base color type over the D2D type
*/
use winapi::shared::d3d9types::D3DCOLORVALUE;

/// A solid color
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {

    /// Create a color from a [r, g, b, a] float array
    pub const fn rgba(v: [f32; 4]) -> Color {
        Color { r: v[0], g: v[1], b: v[2], a: v[3] }
    }

    /// Create a color from a [r, g, b] float array. Alpha is set to `1.0`.
    pub const fn rgb(v: [f32; 3]) -> Color {
        Color { r: v[0], g: v[1], b: v[2], a: 1.0 }
    }

}

impl From<D3DCOLORVALUE> for Color {
    fn from(c: D3DCOLORVALUE) -> Color {
        Color {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a
        }
    }
}

impl Into<D3DCOLORVALUE> for Color {
    fn into(self) -> D3DCOLORVALUE {
        D3DCOLORVALUE {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a
        }
    }
}
