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
