/*!
    Wrapper types over some Direct2D enums and types
*/

use winapi::um::d2d1::{D2D1_BRUSH_PROPERTIES, D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, D2D1_RECT_F, D2D1_POINT_2F,
    D2D1_GAMMA_2_2, D2D1_GAMMA_1_0, D2D1_EXTEND_MODE_CLAMP, D2D1_EXTEND_MODE_WRAP, D2D1_EXTEND_MODE_MIRROR, D2D1_ELLIPSE};
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

/// Represents a ellispe
pub type Ellipse = D2D1_ELLIPSE;

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
    /// A presentation error has occurred that may be recoverable. The caller needs to re-create the render target then attempt to render the frame again.
    RecreateTarget,

    /// Objects used together were not all created from the same factory instance.
    WrongFactory,

    /// Other errors. See https://docs.microsoft.com/en-us/windows/win32/direct2d/direct2d-error-codes
    Other(HRESULT)
}


use winapi::um::dcommon::{DWRITE_MEASURING_MODE_GDI_NATURAL, DWRITE_MEASURING_MODE_GDI_CLASSIC, DWRITE_MEASURING_MODE_NATURAL};
use winapi::um::d2d1::{D2D1_DRAW_TEXT_OPTIONS_NO_SNAP, D2D1_DRAW_TEXT_OPTIONS_CLIP, D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT, D2D1_DRAW_TEXT_OPTIONS_NONE};

bitflags! {
    /// Specifies whether text snapping is suppressed or clipping to the layout rectangle is enabled.
    pub struct DrawTextOptions: u32 {
        /// Text is not vertically snapped to pixel boundaries. This setting is recommended for text that is being animated.
        const NO_SNAP = D2D1_DRAW_TEXT_OPTIONS_NO_SNAP;

        /// Text is clipped to the layout rectangle.
        const CLIP = D2D1_DRAW_TEXT_OPTIONS_CLIP;

        /// In Windows 8.1 and later, text is rendered using color versions of glyphs, if defined by the font.
        const ENABLE_COLOR_FONT = D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT;

        /// Bitmap origins of color glyph bitmaps are not snapped. (Not exported by winapi-rs ????)
        /// const DISABLE_COLOR_BITMAP_SNAPPING = D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING;

        /// Text is vertically snapped to pixel boundaries and is not clipped to the layout rectangle.
        const OPTIONS_NONE = D2D1_DRAW_TEXT_OPTIONS_NONE;
    }
}

#[repr(u32)]
pub enum MeasuringMode {
    /// Specifies that text is measured using glyph ideal metrics whose values are independent to the current display resolution.
    Natural = DWRITE_MEASURING_MODE_NATURAL,

    /// Specifies that text is measured using glyph display-compatible metrics whose values tuned for the current display resolution.
    GdiClassic = DWRITE_MEASURING_MODE_GDI_CLASSIC,

    /// Specifies that text is measured using the same glyph display metrics as text measured by GDI using a font created with CLEARTYPE_NATURAL_QUALITY.
    GdiNatural = DWRITE_MEASURING_MODE_GDI_NATURAL
}
