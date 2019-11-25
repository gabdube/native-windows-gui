/*!
    Describes the caps, miter limit, line join, and dash information for a stroke.

    As with most COM objects, a stroke style if ref counted internally by Direct2D.
    Cloning will increase the ref count and dropping the stroke style resource will decrease it.
*/
use winapi::um::d2d1::{ID2D1StrokeStyle};
use winapi::shared::basetsd::UINT32;
use crate::win32::canvas;
use std::ops::Deref;
use std::{fmt, ptr};



#[derive(Copy, Clone, Debug)]
#[repr(u8)]
/// Describes the shape at the end of a line or segment.
pub enum CapStyle {
    Flat,
    Square,
    Round,
    Triangle,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
/// Describes the shape that joins two lines or segments.
pub enum LineJoin {
    Miter,
    Bevel,
    Round,
    MiterOrBevel,
}


#[derive(Copy, Clone, Debug)]
#[repr(u8)]
/// Describes the sequence of dashes and gaps in a stroke.
pub enum DashStyle {
    Solid,
    Dash,
    Dot,
    DashDot,
    DashDotDot,
    Custom,
}

/// Describes the stroke that outlines a shape.
#[derive(Copy, Clone, Debug)]
pub struct StrokeStyleProperties {
    pub start_cap: CapStyle,
    pub end_cap: CapStyle,
    pub dash_cap: CapStyle,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub dash_style: DashStyle,
    pub dash_offset: f32
}


/// Describes the caps, miter limit, line join, and dash information for a stroke.
/// See module level documentation
pub struct StrokeStyle {
    pub(crate) handle: *mut ID2D1StrokeStyle
}

impl StrokeStyle {

    /// Create a new solid brsuh with the specified color and the specified properties
    ///
    /// Arguments:
    ///   - prop: A structure that describes the stroke's line cap, dash offset, and other details of a stroke
    ///   - dashes: An array whose elements are set to the length of each dash and space in the dash pattern.
    ///             The first element sets the length of a dash, the second element sets the length of a space,
    ///             the third element sets the length of a dash, and so on. The length of each dash and space in the dash pattern is
    ///             the product of the element value in the array and the stroke width.
    ///
    pub fn new<T>(canvas: &T, prop: &StrokeStyleProperties, dashes: &[f32]) -> StrokeStyle 
        where T: Deref<Target = canvas::CanvasRenderer>
    {
        use winapi::um::d2d1::{D2D1_STROKE_STYLE_PROPERTIES, D2D1_CAP_STYLE_FLAT, D2D1_CAP_STYLE_SQUARE,
            D2D1_CAP_STYLE_ROUND, D2D1_CAP_STYLE_TRIANGLE, D2D1_LINE_JOIN_MITER, D2D1_LINE_JOIN_BEVEL, 
            D2D1_LINE_JOIN_ROUND, D2D1_LINE_JOIN_MITER_OR_BEVEL, D2D1_DASH_STYLE_SOLID, D2D1_DASH_STYLE_DASH,
            D2D1_DASH_STYLE_DASH_DOT, D2D1_DASH_STYLE_DASH_DOT_DOT, D2D1_DASH_STYLE_CUSTOM, D2D1_DASH_STYLE_DOT};


        let renderer = &canvas;
        let handle = unsafe {
            let target = &mut *renderer.renderer;
            let mut out: *mut ID2D1StrokeStyle = ptr::null_mut();
            
            // I know, this is an abomination
            let properties = D2D1_STROKE_STYLE_PROPERTIES {
                miterLimit: prop.miter_limit,
                dashOffset: prop.dash_offset,

                startCap: match prop.start_cap {
                    CapStyle::Flat => D2D1_CAP_STYLE_FLAT,
                    CapStyle::Square => D2D1_CAP_STYLE_SQUARE,
                    CapStyle::Round => D2D1_CAP_STYLE_ROUND,
                    CapStyle::Triangle => D2D1_CAP_STYLE_TRIANGLE,
                },

                endCap: match prop.start_cap {
                    CapStyle::Flat => D2D1_CAP_STYLE_FLAT,
                    CapStyle::Square => D2D1_CAP_STYLE_SQUARE,
                    CapStyle::Round => D2D1_CAP_STYLE_ROUND,
                    CapStyle::Triangle => D2D1_CAP_STYLE_TRIANGLE,
                },
                
                dashCap: match prop.start_cap {
                    CapStyle::Flat => D2D1_CAP_STYLE_FLAT,
                    CapStyle::Square => D2D1_CAP_STYLE_SQUARE,
                    CapStyle::Round => D2D1_CAP_STYLE_ROUND,
                    CapStyle::Triangle => D2D1_CAP_STYLE_TRIANGLE,
                },

                lineJoin: match prop.line_join {
                    LineJoin::Miter => D2D1_LINE_JOIN_MITER,
                    LineJoin::Bevel => D2D1_LINE_JOIN_BEVEL,
                    LineJoin::Round => D2D1_LINE_JOIN_ROUND,
                    LineJoin::MiterOrBevel => D2D1_LINE_JOIN_MITER_OR_BEVEL,
                },

                dashStyle: match prop.dash_style {
                    DashStyle::Solid => D2D1_DASH_STYLE_SOLID,
                    DashStyle::Dash => D2D1_DASH_STYLE_DASH,
                    DashStyle::Dot => D2D1_DASH_STYLE_DOT,
                    DashStyle::DashDot => D2D1_DASH_STYLE_DASH_DOT,
                    DashStyle::DashDotDot => D2D1_DASH_STYLE_DASH_DOT_DOT,
                    DashStyle::Custom => D2D1_DASH_STYLE_CUSTOM,
                }
            };
            
            target.CreateStrokeStyle(
                &properties,
                dashes.as_ptr(),
                dashes.len() as UINT32,
                (&mut out) as *mut *mut ID2D1StrokeStyle
            );

            out
        };

        StrokeStyle {
            handle
        }
    }

    /// Shortcut to create a stroke style only from the `DashStyle` parameter
    pub fn from_style<T>(canvas: &T, style: DashStyle) -> StrokeStyle 
        where T: Deref<Target = canvas::CanvasRenderer>
    {
        let prop = StrokeStyleProperties {
            start_cap: CapStyle::Flat,
            end_cap: CapStyle::Flat,
            dash_cap: CapStyle::Flat,
            line_join: LineJoin::Miter,
            miter_limit: 1.0,
            dash_style: style,
            dash_offset: 1.0
        };
        
        StrokeStyle::new(canvas, &prop, &[])
    }

    /// Check if the brush is initialized
    pub fn is_null(&self) -> bool { self.handle.is_null() }
}

impl Default for StrokeStyle {

    fn default() -> StrokeStyle {
        StrokeStyle{  handle: ptr::null_mut() }
    }

}

impl fmt::Debug for StrokeStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_null() {
            return write!(f, "StrokeStyle {{ Unbound }}");
        }

        write!(f, 
            "StrokeStyle {{ }}",
        )
    }
}

impl Clone for StrokeStyle {

    fn clone(&self) -> StrokeStyle {
        match self.is_null() {
            true => StrokeStyle{ handle: ptr::null_mut() },
            false => unsafe {
                (&*self.handle).AddRef();
                StrokeStyle{  handle: self.handle }
            }
        }
    }

}

impl Drop for StrokeStyle {

    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { (&*self.handle).Release(); }
            self.handle = ptr::null_mut();
        }
    }

}

