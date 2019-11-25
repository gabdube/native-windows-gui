/*~
    Base types to pass to drawing functions
*/
use winapi::um::d2d1::{ID2D1Brush};
use super::{SolidBrush, LinearGradientBrush};
use std::convert::TryFrom;
use std::mem;


/// Common brush type to pass any kind of brush to drawing functions
pub struct BaseBrush( pub(crate) *mut ID2D1Brush );

impl TryFrom<&SolidBrush> for BaseBrush {
    type Error = ();

    fn try_from(brush: &SolidBrush) -> Result<Self, Self::Error> {
        if brush.is_null() {
            Err(())
        } else {
            let brush = unsafe { BaseBrush( mem::transmute(brush.handle) ) };
            Ok(brush)
        }
    }
}

impl TryFrom<&LinearGradientBrush> for BaseBrush {
    type Error = ();

    fn try_from(brush: &LinearGradientBrush) -> Result<Self, Self::Error> {
        if brush.is_null() {
            Err(())
        } else {
            let brush = unsafe { BaseBrush( mem::transmute(brush.handle) ) };
            Ok(brush)
        }
    }
}
