/*!
    Common drawing utilities shared across canvas type.
    Instance of `CanvasDraw` are done using `canvas.begin_draw()`.
*/
use winapi::shared::winerror::S_OK;
use crate::win32::canvas;
use super::{CanvasError, Rect, Color, Matrix3x2F, BaseBrush, StrokeStyle};
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
        use winapi::shared::winerror::D2DERR_RECREATE_TARGET;

        let mut tag1 = 0;
        let mut tag2 = 0;

        unsafe {
            let target = &*self.base.render_target;
            match target.EndDraw(&mut tag1, &mut tag2) {
                S_OK => Ok(()),
                D2DERR_RECREATE_TARGET => Err(CanvasError::RecreateTarget),
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

}
