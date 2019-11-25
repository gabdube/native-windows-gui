/*!
    A brush that paints an area with a solid color.

    As with most COM objects, a solid brush if ref counted internally by Direct2D.
    Cloning will increase the ref count and dropping the brush resource will decrease it.
*/
use winapi::um::d2d1::{ID2D1SolidColorBrush};
use super::{Color, BrushProperties, Matrix3x2F};
use crate::win32::canvas;
use std::ops::Deref;
use std::{mem, ptr, fmt};


/// A brush that can be used to paint areas with a solid color
/// See module level documentation
pub struct SolidBrush {
    pub(crate) handle: *mut ID2D1SolidColorBrush
}

impl SolidBrush {

    /// Create a new solid brsuh with the specified color and the specified properties
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
            handle
        }
    }

    /// Create a solid brush from a color. Use the default brush properties
    pub fn from_color<T>(canvas: &T, color: Color) -> SolidBrush 
        where T: Deref<Target = canvas::CanvasRenderer>
    {
        SolidBrush::new(
            canvas,
            color,
            BrushProperties { opacity: 1.0, transform: Matrix3x2F{ matrix: [[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]]  }  }
        )
    }

    /// Check if the brush is initialized
    pub fn is_null(&self) -> bool { self.handle.is_null() }

    /// Return the color of the brush. Panic if the resource is not bound.
    pub fn color(&self) -> Color {
        if self.is_null() { panic!("Resources is not bound to a render target") }
        unsafe { (&*self.handle).GetColor() }
    }

    /// Sets the color of the brush. Panic if the resource is not bound.
    pub fn set_color(&self, color: Color) {
        if self.is_null() { panic!("Resources is not bound to a render target") }
        unsafe { (&*self.handle).SetColor(&color); }
    }

    /// Return the opacity of the brush. Panic if the resource is not bound.
    pub fn opacity(&self) -> f32 {
        if self.is_null() { panic!("Resources is not bound to a render target") }
        unsafe { (&*self.handle).GetOpacity() }
    }

    /// Sets the opacity of the brush. Panic if the resource is not bound.
    pub fn set_opacity(&self, op: f32) {
        if self.is_null() { panic!("Resources is not bound to a render target") }
        unsafe { (&*self.handle).SetOpacity(op); }
    }

    /// Return the transform of the brush. Panic if the resource is not bound.
    pub fn transform(&self) -> Matrix3x2F {
        if self.is_null() { panic!("Resources is not bound to a render target") }

        unsafe { 
            let mut transform = mem::zeroed();
            (&*self.handle).GetTransform(&mut transform);
            transform
        }
    }

    /// Sets the transform of the brush. Panic if the resource is not bound.
    pub fn set_transform(&self, mat: Matrix3x2F) {
        if self.is_null() { panic!("Resources is not bound to a render target") }
        unsafe { (&*self.handle).SetTransform(&mat); }
    }

}

impl Default for SolidBrush {

    fn default() -> SolidBrush {
        SolidBrush{  handle: ptr::null_mut() }
    }

}

impl fmt::Debug for SolidBrush {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_null() {
            return write!(f, "SolidBrush {{ Unbound }}");
        }

        let c = self.color();
        let op = self.opacity();
        let transform = self.transform();
        write!(f, 
            "SolidBrush {{ color: [{}, {}, {}, {}], properties: BrushProperties{{ opacity: {}, transform: {:?} }} }}",
            c.r, c.g, c.b, c.a, op, transform.matrix
        )
    }
}

impl Clone for SolidBrush {

    fn clone(&self) -> SolidBrush {
        match self.is_null() {
            true => SolidBrush{ handle: ptr::null_mut() },
            false => unsafe {
                (&*self.handle).AddRef();
                SolidBrush{  handle: self.handle }
            }
        }
    }

}

impl Drop for SolidBrush {

    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { (&*self.handle).Release(); }
            self.handle = ptr::null_mut();
        }
    }

}
