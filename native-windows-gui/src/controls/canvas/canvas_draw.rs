/*!
    Common drawing utilities shared across canvas type.
    Instance of `CanvasDraw` are done using `canvas.begin_draw()`.
*/
use winapi::shared::winerror::S_OK;
use crate::win32::{canvas, base_helper};
use super::{CanvasError, Rect, Ellipse, Color, Matrix3x2F, BaseBrush, StrokeStyle, DrawTextOptions, MeasuringMode, WriteTextFormat};
use std::convert::TryInto;


pub struct CanvasDraw<'a> {
    base: &'a canvas::CanvasRenderer
}

impl<'a> CanvasDraw<'a> {

    pub fn new(renderer: &'a canvas::CanvasRenderer) -> CanvasDraw {
        unsafe {
            let target = &*renderer.render_target;
            target.BeginDraw();
        }

        CanvasDraw {
            base: renderer
        }
    }

    /// Finish drawing
    pub fn end_draw(self) -> Result<(), CanvasError> {
        use winapi::shared::winerror::{D2DERR_RECREATE_TARGET, D2DERR_WRONG_FACTORY};

        let mut tag1 = 0;
        let mut tag2 = 0;

        unsafe {
            let target = &*self.base.render_target;
            match target.EndDraw(&mut tag1, &mut tag2) {
                S_OK => Ok(()),
                D2DERR_RECREATE_TARGET => Err(CanvasError::RecreateTarget),
                D2DERR_WRONG_FACTORY => Err(CanvasError::WrongFactory),
                e => Err(CanvasError::Other(e))
            }
        }
    }

    /// Return the size of the canvas. The canvas size may be different from the window size.
    pub fn size(&self) -> (f32, f32) {
        unsafe {
            let target = &*self.base.render_target;
            let size = target.GetSize();
            (size.width, size.height)
        }
    }

    /// Sets the default transform for the draw commands. 
    /// If `transform` is None, use a identity matrix
    pub fn transform(&self, transform: Option<&Matrix3x2F>) {
        unsafe {
            let target = &*self.base.render_target;
            match transform {
                None => {
                    let t = Matrix3x2F{ matrix: [[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]]  };
                    target.SetTransform(&t);
                },
                Some(t) => { target.SetTransform(t); }
            }
        }
    }

    /// Clear the canvas area using the specified color
    pub fn clear(&self, color: Color) {
        unsafe {
            let target = &*self.base.render_target;
            let color = color.into();
            target.Clear(&color);
        }
    }

    /// Draws the outline of a rectangle that has the specified dimensions and stroke style.
    pub fn draw_rectangle<B: TryInto<BaseBrush>>(&self, rect: &Rect, brush: B, stroke_width: f32, stroke_style: &StrokeStyle) {
        let base = match brush.try_into() {
            Ok(b) => b,
            Err(_) => panic!("Brush is invalid")
        };

        unsafe {
            let target = &*self.base.render_target;
            target.DrawRectangle(rect, base.0, stroke_width, stroke_style.handle);
        }
    }

    /// Draws the outline of a ellipse that has the specified dimensions and stroke style.
    pub fn draw_ellipse<B: TryInto<BaseBrush>>(&self, ell: &Ellipse, brush: B, stroke_width: f32, stroke_style: &StrokeStyle) {
        let base = match brush.try_into() {
            Ok(b) => b,
            Err(_) => panic!("Brush is invalid")
        };

        unsafe {
            let target = &*self.base.render_target;
            target.DrawEllipse(ell, base.0, stroke_width, stroke_style.handle);
        }
    }

    /// Uses a brush to fill the interior of a rectangle.
    /// Panics if the brush is not bound to the renderer
    pub fn fill_rectangle<B: TryInto<BaseBrush>>(&self, rect: &Rect, brush: B) {
        let base = match brush.try_into() {
            Ok(b) => b,
            Err(_) => panic!("Brush is invalid")
        };

        unsafe {
            let target = &*self.base.render_target;
            target.FillRectangle(rect, base.0);
        }
    }

    /// Uses a brush to fill the interior of a ellipse.
    /// Panics if the brush is not bound to the renderer
    pub fn fill_ellipse<B: TryInto<BaseBrush>>(&self, ell: &Ellipse, brush: B) {
        let base = match brush.try_into() {
            Ok(b) => b,
            Err(_) => panic!("Brush is invalid")
        };

        unsafe {
            let target = &*self.base.render_target;
            target.FillEllipse(ell, base.0);
        }
    }

    /// Draws the specified text onto the canvas
    /// You might want to use `draw_simple_text` for a simplified interface over this function
    ///
    /// Arguments:
    ///  - text: The string of text to draw
    ///  - fmt: An object that describes formatting details of the text to draw, such as the font, the font size, and flow direction.
    ///  - area: The size and position of the area in which the text is drawn.
    ///  - brush: The brush used to paint the text.
    ///  - options: A value that indicates whether the text should be snapped to pixel boundaries and whether the text should be clipped to the layout rectangle. 
    ///  - measure: A value that indicates how glyph metrics are used to measure text when it is formatted.
    pub fn draw_text<'b, B: TryInto<BaseBrush>>(&self, text: &'b str, fmt: &WriteTextFormat, area: &Rect, brush: B, options: DrawTextOptions, measure: MeasuringMode) {
        use winapi::um::d2d1::D2D1_DRAW_TEXT_OPTIONS;
        use winapi::um::dcommon::DWRITE_MEASURING_MODE;
        
        unsafe {
            let target = &*self.base.render_target;
            let text = base_helper::to_utf16(text);
            let text_length = text.len();

            let base = match brush.try_into() {
                Ok(b) => b,
                Err(_) => panic!("Brush is invalid")
            };

            target.DrawText(
                text.as_ptr(),
                text_length as u32,
                fmt.handle,
                area,
                base.0,
                options.bits() as D2D1_DRAW_TEXT_OPTIONS,
                measure as DWRITE_MEASURING_MODE   
            );
        }
    }

    /// Draws the specified text onto the canvas
    /// Even though it might not look like it, this is a simplified interface over `draw_text` 
    ///
    // Arguments:
    ///  - text: The string of text to draw
    ///  - fmt: An object that describes formatting details of the text to draw, such as the font, the font size, and flow direction.
    ///  - pos: The position of the text
    ///  - brush: The brush used to paint the text.
    pub fn draw_simple_text<'b, B: TryInto<BaseBrush>>(&self, text: &'a str, fmt: &WriteTextFormat, pos: (f32, f32), size: (f32, f32), brush: B) {
        let area = Rect {
            left: pos.0,
            top: pos.1,
            right: size.0,
            bottom: size.1,
        };
        
        self.draw_text(
            text,
            fmt,
            &area,
            brush,
            DrawTextOptions::OPTIONS_NONE,
            MeasuringMode::Natural
        )
    }

}
