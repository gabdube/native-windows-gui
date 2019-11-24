/*!
    All the canvas resources
*/
use winapi::um::d2d1::{D2D1_BRUSH_PROPERTIES, D2D1_COLOR_F};
use winapi::um::d2d1::{ID2D1SolidColorBrush};
use winapi::um::d2dbasetypes::D2D_MATRIX_3X2_F;

use crate::win32::canvas;
use std::ops::Deref;
use std::{ptr, fmt};


/// A solid color
pub type Color = D2D1_COLOR_F;

/// Common brush properties
pub type BrushProperties = D2D1_BRUSH_PROPERTIES;


/// A brush that can be used to paint areas with a solid color
pub struct SolidBrush {
    handle: *mut ID2D1SolidColorBrush,
    color: Color,
    properties: BrushProperties
}

impl SolidBrush {

    pub fn new<T>(canvas: &T, color: Color, properties: BrushProperties) -> SolidBrush 
        where T: Deref<Target = canvas::CanvasRenderer>
    {
        let renderer = &canvas;
        let handle = unsafe {
            let target = &mut *renderer.render_target;
            let mut out: *mut ID2D1SolidColorBrush = ptr::null_mut();
            target.CreateSolidColorBrush(
                &color as *const Color,
                &properties as *const BrushProperties,
                (&mut out) as *mut *mut ID2D1SolidColorBrush
            );

            out
        };

        SolidBrush {
            handle,
            color,
            properties
        }
    }

    pub fn color(&self) -> Color { self.color.clone() }
    pub fn properties(&self) -> BrushProperties { self.properties.clone() }
}

impl Default for SolidBrush {

    fn default() -> SolidBrush {
        SolidBrush{ 
            handle: ptr::null_mut(),
            color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            properties: BrushProperties {
                opacity: 1.0, 
                transform: D2D_MATRIX_3X2_F{ matrix: [[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]] }
            }
        }
    }

}

impl fmt::Debug for SolidBrush {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = &self.color;
        let p = &self.properties;
        write!(f, 
            "SolidBrush {{ color: [{}, {}, {} ,{}], properties: BrushProperties{{ opacity: {}, transform: {:?} }} }}",
            c.r, c.g, c.b, c.a, p.opacity, p.transform.matrix
        )
    }
}
