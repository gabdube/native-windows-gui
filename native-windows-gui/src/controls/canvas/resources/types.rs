/*!
Sane Typedef for the D2D types
*/
use winapi::um::d2d1::{D2D1_BRUSH_PROPERTIES, D2D1_COLOR_F, D2D1_RECT_F};
use winapi::um::d2dbasetypes::D2D_MATRIX_3X2_F;
use winapi::shared::ntdef::HRESULT;


/// A solid color
pub type Color = D2D1_COLOR_F;

/// Common brush properties
pub type BrushProperties = D2D1_BRUSH_PROPERTIES;

/// A simple 3x2 matrix
pub type Matrix3x2F = D2D_MATRIX_3X2_F;

/// Represents a rectangle
pub type Rect = D2D1_RECT_F;


/// Errors that can be returned when drawing to a canvas
#[derive(Copy, Clone, Debug)]
pub enum CanvasError {
    Other(HRESULT)
}
