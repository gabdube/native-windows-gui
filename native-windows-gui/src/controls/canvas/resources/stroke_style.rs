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

impl CapStyle {
    
    fn from_d2d(value: u32) -> CapStyle {
        use winapi::um::d2d1::{D2D1_CAP_STYLE_FLAT, D2D1_CAP_STYLE_SQUARE, D2D1_CAP_STYLE_ROUND, D2D1_CAP_STYLE_TRIANGLE};
        match value {
            D2D1_CAP_STYLE_FLAT => CapStyle::Flat,
            D2D1_CAP_STYLE_SQUARE => CapStyle::Square,
            D2D1_CAP_STYLE_ROUND => CapStyle::Round,
            D2D1_CAP_STYLE_TRIANGLE => CapStyle::Triangle,
            _ => CapStyle::Flat
        }
    }

    fn into_d2d(self) -> u32 {
        use winapi::um::d2d1::{D2D1_CAP_STYLE_FLAT, D2D1_CAP_STYLE_SQUARE, D2D1_CAP_STYLE_ROUND, D2D1_CAP_STYLE_TRIANGLE};
        match self {
            CapStyle::Flat => D2D1_CAP_STYLE_FLAT,
            CapStyle::Square => D2D1_CAP_STYLE_SQUARE,
            CapStyle::Round => D2D1_CAP_STYLE_ROUND,
            CapStyle::Triangle => D2D1_CAP_STYLE_TRIANGLE,
        }
    }
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
        use winapi::um::d2d1::{D2D1_STROKE_STYLE_PROPERTIES, D2D1_LINE_JOIN_MITER, D2D1_LINE_JOIN_BEVEL, 
            D2D1_LINE_JOIN_ROUND, D2D1_LINE_JOIN_MITER_OR_BEVEL, D2D1_DASH_STYLE_SOLID, D2D1_DASH_STYLE_DASH,
            D2D1_DASH_STYLE_DASH_DOT, D2D1_DASH_STYLE_DASH_DOT_DOT, D2D1_DASH_STYLE_CUSTOM, D2D1_DASH_STYLE_DOT};


        let renderer = &canvas;
        let handle = unsafe {
            let target = &mut *renderer.renderer;
            let mut out: *mut ID2D1StrokeStyle = ptr::null_mut();
            
            let properties = D2D1_STROKE_STYLE_PROPERTIES {
                miterLimit: prop.miter_limit,
                dashOffset: prop.dash_offset,

                startCap: prop.start_cap.into_d2d(),
                endCap: prop.end_cap.into_d2d(),
                dashCap: prop.dash_cap.into_d2d(),

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

            let (dash_ptr, dash_count) = if properties.dashStyle == D2D1_DASH_STYLE_CUSTOM {
                (dashes.as_ptr(), dashes.len() as UINT32)
            } else {
                (ptr::null(), 0)
            };
            
            target.CreateStrokeStyle(
                &properties,
                dash_ptr,
                dash_count,
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
            dash_offset: 0.0
        };
        
        StrokeStyle::new(canvas, &prop, &[])
    }

    /// Gets a value that specifies how the ends of each dash are drawn.
    pub fn dash_cap(&self) -> CapStyle {
        let cap = unsafe { (&*self.handle).GetDashCap() };
        CapStyle::from_d2d(cap)
    }   

    /// Retrieves the type of shape used at the end of a stroke.
    pub fn end_cap(&self) -> CapStyle {
        let cap = unsafe { (&*self.handle).GetEndCap() };
        CapStyle::from_d2d(cap)
    }

    /// Retrieves the type of shape used at the beginning of a stroke.
    pub fn start_cap(&self) -> CapStyle {
        let cap = unsafe { (&*self.handle).GetStartCap() };
        CapStyle::from_d2d(cap)
    }

    /// Gets a value that describes the stroke's dash pattern.
    pub fn line_join(&self) -> LineJoin {
        use winapi::um::d2d1::{D2D1_LINE_JOIN_MITER, D2D1_LINE_JOIN_BEVEL, D2D1_LINE_JOIN_ROUND, D2D1_LINE_JOIN_MITER_OR_BEVEL};

        let style = unsafe { (&*self.handle).GetDashStyle() };
        match style {
            D2D1_LINE_JOIN_MITER => LineJoin::Miter,
            D2D1_LINE_JOIN_BEVEL => LineJoin::Bevel,
            D2D1_LINE_JOIN_ROUND => LineJoin::Round,
            D2D1_LINE_JOIN_MITER_OR_BEVEL => LineJoin::MiterOrBevel,
            _ => LineJoin::Miter,
        }
    }

    /// Retrieves the type of joint used at the vertices of a shape's outline.
    pub fn dash_style(&self) -> DashStyle {
        use winapi::um::d2d1::{D2D1_DASH_STYLE_SOLID, D2D1_DASH_STYLE_DASH, D2D1_DASH_STYLE_DASH_DOT,
            D2D1_DASH_STYLE_DASH_DOT_DOT, D2D1_DASH_STYLE_CUSTOM, D2D1_DASH_STYLE_DOT};

        let style = unsafe { (&*self.handle).GetLineJoin() };
        match style {
            D2D1_DASH_STYLE_SOLID => DashStyle::Solid,
            D2D1_DASH_STYLE_DASH => DashStyle::Dash,
            D2D1_DASH_STYLE_DASH_DOT => DashStyle::DashDot,
            D2D1_DASH_STYLE_DASH_DOT_DOT => DashStyle::DashDotDot,
            D2D1_DASH_STYLE_CUSTOM => DashStyle::Custom,
            D2D1_DASH_STYLE_DOT => DashStyle::Dot,
            _ => DashStyle::Solid,
        }
    }

    /// Copies the dash pattern to the specified array.
    pub fn dashes(&self) -> Vec<f32> {
        unsafe {
            let handle = &*self.handle;
            let dash_count = handle.GetDashesCount() as usize;
            let mut dashes = Vec::with_capacity(dash_count);
            dashes.set_len(dash_count);

            handle.GetDashes(dashes.as_mut_ptr(), dash_count as u32);

            dashes
        }
    }

    /// Retrieves a value that specifies how far in the dash sequence the stroke will start.
    pub fn dash_offset(&self) -> f32 {
        unsafe { (&*self.handle).GetDashOffset() }
    }

    /// Retrieves the limit on the ratio of the miter length to half the stroke's thickness.
    pub fn miter_limit(&self) -> f32 {
        unsafe { (&*self.handle).GetMiterLimit() }
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

        let sc = self.start_cap();
        let ec = self.end_cap();
        let dc = self.dash_cap();
        let lj = self.line_join();
        let ds = self.dash_style();
        let doff = self.dash_offset();
        let ml = self.miter_limit();
        let dashes = self.dashes();

        write!(f, 
            "StrokeStyle {{ start_cap: {:?}, end_cap: {:?}, dash_cap: {:?}, line_join: {:?}, dash_style: {:?}, dash_offset: {:?}, miter_limit: {:?}, dashes: {:?} }}",
            sc, ec, dc, lj, ds, doff, ml, dashes
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

