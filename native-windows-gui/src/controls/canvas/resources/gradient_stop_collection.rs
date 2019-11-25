/*!
    Represents an collection of GradientStop objects for linear and radial gradient brushes.

    Winapi documetation: https://docs.microsoft.com/en-us/windows/win32/api/d2d1/nf-d2d1-id2d1rendertarget-creategradientstopcollection(constd2d1_gradient_stop_uint32_d2d1_gamma_d2d1_extend_mode_id2d1gradientstopcollection)
*/

use winapi::um::d2d1::{ID2D1GradientStopCollection};
use crate::win32::canvas;
use super::{GradientStop, Gamma, ExtendMode};
use std::ops::Deref;
use std::{ptr, fmt};


/// Represents an collection of GradientStop objects for linear and radial gradient brushes.
/// See module level documentation
pub struct GradientStopCollection {
    pub(crate) handle: *mut ID2D1GradientStopCollection
}

impl GradientStopCollection {

    pub fn new<T>(canvas: &T, stops: &[GradientStop], gamma: Gamma, extend_mode: ExtendMode) -> GradientStopCollection
        where T: Deref<Target = canvas::CanvasRenderer>
    {
        use winapi::um::d2d1::{D2D1_GAMMA, D2D1_EXTEND_MODE, D2D1_GRADIENT_STOP};

        let renderer = &canvas;
        let handle = unsafe {
            let target = &mut *renderer.render_target;
            let mut out: *mut ID2D1GradientStopCollection = ptr::null_mut();

            let mut gradients: Vec<D2D1_GRADIENT_STOP> = Vec::with_capacity(stops.len());
            for stop in stops {
                gradients.push( D2D1_GRADIENT_STOP { position: stop.position, color: stop.color.into() } );
            }

            target.CreateGradientStopCollection(
                gradients.as_ptr(),
                stops.len() as u32,
                gamma as D2D1_GAMMA,
                extend_mode as D2D1_EXTEND_MODE,
                (&mut out) as *mut *mut ID2D1GradientStopCollection
            );

            out
        };

        GradientStopCollection {
            handle
        }
    }

    /// Creates an GradientStopCollection from the specified gradient stops that uses the GAMMA_2_2 color interpolation gamma and the clamp extend mode.
    pub fn from_stops<T>(canvas: &T, stops: &[GradientStop]) -> GradientStopCollection 
        where T: Deref<Target = canvas::CanvasRenderer>
    {
        GradientStopCollection::new(
            canvas,
            stops,
            Gamma::_2_2,
            ExtendMode::Clamp
        )
    }

    /// Check if the collection is initialized
    pub fn is_null(&self) -> bool { self.handle.is_null() }

}


impl Default for GradientStopCollection {

    fn default() -> GradientStopCollection {
        GradientStopCollection{ handle: ptr::null_mut() }
    }

}

impl fmt::Debug for GradientStopCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_null() {
            return write!(f, "GradientStopCollection {{ Unbound }}");
        }

        write!(f, 
            "GradientStopCollection {{ }}",
        )
    }
}

impl Clone for GradientStopCollection {

    fn clone(&self) -> GradientStopCollection {
        match self.is_null() {
            true => GradientStopCollection{ handle: ptr::null_mut() },
            false => unsafe {
                (&*self.handle).AddRef();
                GradientStopCollection{  handle: self.handle }
            }
        }
    }

}

impl Drop for GradientStopCollection {

    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { (&*self.handle).Release(); }
            self.handle = ptr::null_mut();
        }
    }

}
