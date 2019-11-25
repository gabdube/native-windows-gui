/*!
    A brush that can be used to paint an area with a linear gradient.

    As with most COM objects, this brush is ref counted internally by Direct2D.
    Cloning will increase the ref count and dropping the brush resource will decrease it.
*/
use winapi::um::d2d1::{ID2D1LinearGradientBrush};
use crate::win32::canvas;
use super::{GradientStopCollection, LinearBrushProperties, BrushProperties, Matrix3x2F};
use std::ops::Deref;
use std::{mem, ptr, fmt};


/// A brush that can be used to paint an area with a linear gradient.
/// See module level documentation
pub struct LinearGradientBrush {
    pub(crate) handle: *mut ID2D1LinearGradientBrush
}

impl LinearGradientBrush {

    /// Create a new linear gradient brush with the specified colors and the specified properties
    pub fn new<T>(canvas: &T, lin_properties: &LinearBrushProperties, stop_collection: &GradientStopCollection, properties: &BrushProperties) -> LinearGradientBrush 
        where T: Deref<Target = canvas::CanvasRenderer>
    {
        let renderer = &canvas;
        let handle = unsafe {
            let target = &mut *renderer.render_target;
            let mut out: *mut ID2D1LinearGradientBrush = ptr::null_mut();
            target.CreateLinearGradientBrush(
                lin_properties,
                properties,
                stop_collection.handle,
                (&mut out) as *mut *mut ID2D1LinearGradientBrush
            );

            out
        };

        LinearGradientBrush {
            handle
        }
    }

    /// Create a linear gradient. Use the default brush properties
    pub fn from_linear_gradient<T>(canvas: &T, lin_properties: &LinearBrushProperties, stop_collections: &GradientStopCollection) -> LinearGradientBrush
        where T: Deref<Target = canvas::CanvasRenderer>
    {
        LinearGradientBrush::new(
            canvas,
            lin_properties,
            stop_collections,
            &BrushProperties { opacity: 1.0, transform: Matrix3x2F{ matrix: [[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]]  }  }
        )
    }

    /// Check if the brush is initialized
    pub fn is_null(&self) -> bool { self.handle.is_null() }

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

impl Default for LinearGradientBrush {

    fn default() -> LinearGradientBrush {
        LinearGradientBrush{ handle: ptr::null_mut() }
    }

}

impl fmt::Debug for LinearGradientBrush {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_null() {
            return write!(f, "LinearGradientBrush {{ Unbound }}");
        }

        write!(f, 
            "LinearGradientBrush {{ }}",
        )
    }
}

impl Clone for LinearGradientBrush {

    fn clone(&self) -> LinearGradientBrush {
        match self.is_null() {
            true => LinearGradientBrush{ handle: ptr::null_mut() },
            false => unsafe {
                (&*self.handle).AddRef();
                LinearGradientBrush{  handle: self.handle }
            }
        }
    }

}

impl Drop for LinearGradientBrush {

    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { (&*self.handle).Release(); }
            self.handle = ptr::null_mut();
        }
    }

}
