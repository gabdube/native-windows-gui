/*!
Sane Typedef for the D2D types
*/
use winapi::um::d2d1::{D2D1_BRUSH_PROPERTIES, D2D1_COLOR_F};
use winapi::um::d2dbasetypes::D2D_MATRIX_3X2_F;


/// A solid color
pub type Color = D2D1_COLOR_F;

/// Common brush properties
pub type BrushProperties = D2D1_BRUSH_PROPERTIES;

/// A simple 3x2 matrix
pub type Matrix3x2F = D2D_MATRIX_3X2_F;
