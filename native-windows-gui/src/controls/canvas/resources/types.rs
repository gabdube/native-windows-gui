use winapi::um::d2d1::{D2D1_BRUSH_PROPERTIES, D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, D2D1_RECT_F, D2D1_POINT_2F,
    D2D1_GAMMA_2_2, D2D1_GAMMA_1_0, D2D1_EXTEND_MODE_CLAMP, D2D1_EXTEND_MODE_WRAP, D2D1_EXTEND_MODE_MIRROR};
use winapi::um::d2dbasetypes::D2D_MATRIX_3X2_F;
use winapi::shared::ntdef::HRESULT;
use super::Color;


/// Common brush properties
pub type BrushProperties = D2D1_BRUSH_PROPERTIES;

/// Linear gradient brush properties
pub type LinearBrushProperties = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES;

/// A simple 3x2 matrix
pub type Matrix3x2F = D2D_MATRIX_3X2_F;

/// Represents a rectangle
pub type Rect = D2D1_RECT_F;

// A two dimensional point
pub type Point2F = D2D1_POINT_2F;

/// Specifies which gamma is used for interpolation.
/// See https://docs.microsoft.com/en-us/windows/win32/api/d2d1/ne-d2d1-d2d1_gamma
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Gamma {
    _2_2 = D2D1_GAMMA_2_2,
    _1_0 = D2D1_GAMMA_1_0
}

/// Specifies how a brush paints areas outside of its normal content area.
/// See https://docs.microsoft.com/en-us/windows/win32/api/d2d1/ne-d2d1-d2d1_extend_mode
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ExtendMode {
    Clamp = D2D1_EXTEND_MODE_CLAMP,
    Wrap = D2D1_EXTEND_MODE_WRAP,
    Mirror = D2D1_EXTEND_MODE_MIRROR,
}

/// Contains the position and color of a gradient stop.
/// See https://docs.microsoft.com/en-us/windows/win32/api/d2d1/ns-d2d1-d2d1_gradient_stop
#[derive(Debug)]
pub struct GradientStop {
    pub position: f32,
    pub color: Color,
}

/// Errors that can be returned when drawing to a canvas
#[derive(Copy, Clone, Debug)]
pub enum CanvasError {
    RecreateTarget,
    Other(HRESULT)
}
