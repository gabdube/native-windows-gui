/*!
    A brush that can be used to paint an area with a linear gradient.

    As with most COM objects, this brush is ref counted internally by Direct2D.
    Cloning will increase the ref count and dropping the brush resource will decrease it.

    Winapi documentation: https://docs.microsoft.com/en-us/windows/win32/direct2d/direct2d-brushes-overview

    ## Example
    ```
    use native_windows_gui as nwg;

    fn init(canvas: &nwg::Canvas, header_gradient: &nwg::GradientStopCollection) {
        let linear_props = nwg::LinearBrushProperties { startPoint: nwg::Point2F {x:150.0, y:30.0}, endPoint: nwg::Point2F {x:150.0, y:0.0} };
        let header_inner_brush = nwg::LinearGradientBrush::from_linear_gradient(canvas, &linear_props, header_gradient);
    }
    ```
*/
use winapi::um::d2d1::{ID2D1LinearGradientBrush};
use crate::win32::canvas;
use super::{GradientStopCollection, LinearBrushProperties, BrushProperties, Matrix3x2F, Point2F};
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

    /// Retrieves the ending coordinates of the linear gradient.
    pub fn end_point(&self) -> Point2F {
        if self.is_null() { panic!("Resources is not bound to a render target") }
        unsafe { (&*self.handle).GetEndPoint() }
    }

    /// Sets the ending coordinates of the linear gradient in the brush's coordinate space.
    pub fn set_end_point(&self, point: &Point2F) {
        if self.is_null() { panic!("Resources is not bound to a render target") }
        unsafe { (&*self.handle).SetEndPoint( Point2F { x: point.x, y: point.y } ); }
    }
    
    /// Retrieves the starting coordinates of the linear gradient.
    pub fn start_point(&self) -> Point2F {
        if self.is_null() { panic!("Resources is not bound to a render target") }
        unsafe { (&*self.handle).GetStartPoint() }
    }

    /// Sets the ending coordinates of the linear gradient in the brush's coordinate space.
    pub fn set_start_point(&self, point: &Point2F) {
        if self.is_null() { panic!("Resources is not bound to a render target") }
        unsafe { (&*self.handle).SetStartPoint( Point2F { x: point.x, y: point.y } ); }
    }

    /// Retrieves the ID2D1GradientStopCollection associated with this linear gradient brush.
    pub fn gradient_stop_collection(&self) -> GradientStopCollection {
        if self.is_null() { panic!("Resources is not bound to a render target") }

        let mut collection = GradientStopCollection::default();
        unsafe { (&*self.handle).GetGradientStopCollection(&mut collection.handle); }
        collection
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

        let sp = self.start_point();
        let ep = self.end_point();

        write!(f, 
            "LinearGradientBrush {{ start_point: {:?}, end_point: {:?} }}",
            (sp.x, sp.y), (ep.x, ep.y)
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
