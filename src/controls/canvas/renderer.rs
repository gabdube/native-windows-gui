/*!
    A canvas control where the user can draw stuff.
    This defines the canvas renderer
*/

use std::hash::Hash;
use std::ptr;
use std::mem;
use std::any::TypeId;
use std::ops::{Deref, DerefMut};

use winapi::{FLOAT, D2D1_RECT_F, D2D1_ROUNDED_RECT, D2D1_ELLIPSE, D2D1_POINT_2F, 
  D2D1_MATRIX_3X2_F, ID2D1Brush, ID2D1StrokeStyle};

use ui::Ui;
use error::Error;
use resources::{Brush};
use controls::AnyHandle;
use defs::canvas::{Rectangle, Ellipse};
use super::{Canvas, CanvasProtected};

/**
    Object that offers a light wrapper over the D2D1 api.
*/
pub struct CanvasRenderer<'a, ID: Clone+Hash+'static> {
    pub ui: &'a Ui<ID>,
    canvas: &'a mut Canvas<ID>
}

impl<'a, ID: Clone+Hash> CanvasRenderer<'a, ID> {
    
    /**
        Clears the drawing area to the specified color.  
    
        Arguments:  
        • `r`: Red component. 0.0 - 1.0  
        • `g`: Green component. 0.0 - 1.0  
        • `b`: Blue component. 0.0 - 1.0  
        • `a`: Alpha component. 0.0 - 1.0  
    */
    pub fn clear(&mut self, r:f32, g:f32, b: f32, a: f32) {
        use winapi::D2D1_COLOR_F;
        let color = D2D1_COLOR_F{r:r, g:g, b:b, a:a};
        unsafe{ self.Clear(&color); }
    }

    /**
        Return the render target size in a tuple of (`width`, `height`)
    */
    pub fn get_render_size(&mut self) -> (f32, f32) {
        use winapi::D2D1_SIZE_F;
        let mut render_size = D2D1_SIZE_F{width: 0.0, height: 0.0};
        unsafe{ self.GetSize(&mut render_size); }
        
        (render_size.width as f32, render_size.height as f32)
    }

    /**
        Set the transformation matrix of the renderer

        Arguments:  
        • `m`: The 3x2 matrix.
    */
    pub fn set_transform(&mut self, m: &[[FLOAT; 2]; 3]) {
        unsafe{ self.SetTransform( &D2D1_MATRIX_3X2_F{ matrix: *m } ); }
    }

    /**
        Return the transformation matrix of the renderer
    */
    pub fn get_transform(&mut self) -> [[FLOAT; 2]; 3] {
        unsafe{ 
            let mut m: D2D1_MATRIX_3X2_F = mem::uninitialized();
            self.GetTransform( &mut m );
            m.matrix
        }
    }

    /**
        Fill a rectangle shape defined by `r` using the brush identified by `brush`  

        Arguments:  
        • `brush`: Id of the brush saved in the canvas  
        • `r`: The rectangle shape to draw  
    */
    pub fn fill_rectangle(&mut self, brush: &ID, r: &Rectangle) -> Result<(), Error> {
        let rect = D2D1_RECT_F{left: r.left, top: r.top, bottom: r.bottom, right: r.right};
        let brush = match self.fill_setup(brush) {
            Ok(d) => d,
            Err(e) => { return Err(e); }
        };

        unsafe{ self.FillRectangle(&rect, mem::transmute(brush) ) };
        Ok(())
    }

    /**
        Fill a rounded rectangle shape defined by `r` using the brush identified by `brush`  

        Arguments:  
        • `brush`: Id of the brush saved in the canvas  
        • `r`: The rectangle shape to draw  
        • `radius`: Amount of rounding on the rectangle border. (`width`, `height`)
    */
    pub fn fill_rounded_rectangle(&mut self, brush: &ID, r: &Rectangle, radius: (f32, f32)) -> Result<(), Error> {
        let rect = D2D1_RECT_F{left: r.left, top: r.top, bottom: r.bottom, right: r.right};
        let rect = D2D1_ROUNDED_RECT{ rect: rect, radiusX: radius.0, radiusY: radius.1 };
        let brush = match self.fill_setup(brush) {
            Ok(d) => d,
            Err(e) => { return Err(e); }
        };

        unsafe{ self.FillRoundedRectangle(&rect, mem::transmute(brush)); }
        Ok(())
    }

    /**
        Fill an ellipse shape defined by `e` using the brush identified by `brush`  

        Arguments:  
        • `brush`: Id of the brush saved in the canvas  
        • `e`: The ellipse shape to draw  
    */
    pub fn fill_ellipse(&mut self, brush: &ID, e: &Ellipse) -> Result<(), Error> {
        let ellipse = D2D1_ELLIPSE{point: D2D1_POINT_2F{ x: e.center.0, y: e.center.1 } , radiusX: e.radius.0, radiusY: e.radius.1};
        let brush = match self.fill_setup(brush) {
            Ok(d) => d,
            Err(e) => { return Err(e); }
        };

        unsafe{ self.FillEllipse(&ellipse, mem::transmute(brush)); }
        Ok(())
    }

    /**
        Draw the outline of a rectangle shape defined by `r` using the brush identified by `brush` and the pen identifed
        by `pen`.

        Arguments:  
        • `brush`: Id of the brush saved in the canvas  
        • `pen`: Id of the pen saved in the canvas  
        • `r`: The rectangle shape to draw  
        • `width`: Width of the outline to draw  
    */
    pub fn draw_rectangle(&mut self, brush: &ID, pen: Option<&ID>, r: &Rectangle, width: f32) -> Result<(), Error> {
        let rect = D2D1_RECT_F{left: r.left, top: r.top, bottom: r.bottom, right: r.right};
        let (brush, pen) = match self.draw_setup(brush, pen) {
            Ok(d) => d,
            Err(e) => { return Err(e); }
        };

        unsafe{ self.DrawRectangle(&rect, mem::transmute(brush), width, mem::transmute(pen)); }

        Ok(())
    }

    /**
        Draw the outline of a rounded rectangle shape defined by `r` using the brush identified by `brush` and the pen identifed
        by `pen`.

        Arguments:  
        • `brush`: Id of the brush saved in the canvas  
        • `pen`: Id of the pen saved in the canvas  
        • `r`: The rectangle shape to draw  
        • `width`: Width of the outline to draw  
        • `radius`: Amount of rounding on the rectangle border. (`width`, `height`)  
    */
    pub fn draw_rounded_rectangle(&mut self, brush: &ID, pen: Option<&ID>, r: &Rectangle, width: f32, radius: (f32, f32)) -> Result<(), Error> {
        let rect = D2D1_RECT_F{left: r.left, top: r.top, bottom: r.bottom, right: r.right};
        let rect = D2D1_ROUNDED_RECT{ rect: rect, radiusX: radius.0, radiusY: radius.1 };
        let (brush, pen) = match self.draw_setup(brush, pen) {
            Ok(d) => d,
            Err(e) => { return Err(e); }
        };

        unsafe{ self.DrawRoundedRectangle(&rect, mem::transmute(brush), width, mem::transmute(pen)); }

        Ok(())
    }

    /**
        Draw the outline of an ellipse shape defined by `e` using the brush identified by `brush` and the pen identifed
        by `pen`.

        Arguments:  
        • `brush`: Id of the brush saved in the canvas  
        • `pen`: Id of the pen saved in the canvas  
        • `e`: The ellipse shape to draw  
        • `width`: Width of the outline to draw  
    */
    pub fn draw_ellipse(&mut self, brush: &ID, pen: Option<&ID>, e: &Ellipse, width: f32) -> Result<(), Error> {
        let ellipse = D2D1_ELLIPSE{point: D2D1_POINT_2F{ x: e.center.0, y: e.center.1 } , radiusX: e.radius.0, radiusY: e.radius.1};
        let (brush, pen) = match self.draw_setup(brush, pen) {
            Ok(d) => d,
            Err(e) => { return Err(e); }
        };

        unsafe{ self.DrawEllipse(&ellipse, mem::transmute(brush), width, mem::transmute(pen)); }

        Ok(())
    }

    fn fill_setup(&mut self, brush: &ID) -> Result<*mut ID2D1Brush, Error> {
        match self.ui.handle_of(brush) {
            Ok(AnyHandle::Custom(t, h)) => {
                if t == TypeId::of::<Brush>() { Ok(h as *mut ID2D1Brush) }
                else { Err(Error::BadResource( format!("A brush resource required. Got a custom handle of another type") )) }
            },
            Ok(h) => Err(Error::BadResource( format!("A brush resource required. Got {}", h.human_name()) )),
            Err(e) => Err(e)
        }
    }

    fn draw_setup(&mut self, brush: &ID, pen: Option<&ID>) -> Result<(*mut ID2D1Brush, *mut ID2D1StrokeStyle), Error> {
        Err(Error::Unimplemented)
    }

}

impl<'a, ID: Clone+Hash> Deref for CanvasRenderer<'a, ID> {
    type Target = Canvas<ID>;
    #[inline(always)] fn deref(&self) -> &Canvas<ID> { &self.canvas }
}

impl<'a, ID: Clone+Hash> DerefMut for CanvasRenderer<'a, ID> {
    #[inline(always)]fn deref_mut(&mut self) -> &mut Canvas<ID> { &mut self.canvas }
}

impl<'a, ID: Clone+Hash> Drop for CanvasRenderer<'a, ID> {
    fn drop(&mut self) {
        unsafe{ 
            use winapi::D2DERR_RECREATE_TARGET;
            let recreate = self.EndDraw(ptr::null_mut(), ptr::null_mut()) == D2DERR_RECREATE_TARGET;
            self.canvas.set_must_recreate_target(recreate); 
        }
    }
}

/**
    Protected renderer method (only available in the canvas control module)
*/
pub trait RendererProtected<'a, ID: Clone+Hash>  {
    fn prepare(canvas: &'a mut Canvas<ID>, ui: &'a Ui<ID>) -> Result<CanvasRenderer<'a, ID>, Error>;
}

impl<'a, ID: Clone+Hash> RendererProtected<'a, ID> for CanvasRenderer<'a, ID> {

    fn prepare(canvas: &'a mut Canvas<ID>, ui: &'a Ui<ID>) -> Result<CanvasRenderer<'a, ID>, Error> {
        unsafe{ 
            if canvas.get_must_recreate_target() {
                if let Err(e) = canvas.rebuild() {
                    return Err(Error::System(e));
                }
            }

            let identity = D2D1_MATRIX_3X2_F {
                matrix: [[1.0, 0.0],
                         [0.0, 1.0],
                         [0.0, 0.0]]
            };

            canvas.BeginDraw(); 
            canvas.SetTransform(&identity);
        }

        Ok( CanvasRenderer { canvas: canvas, ui: ui } )
    }

}