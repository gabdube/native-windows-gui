/*!
    A canvas control where the user can draw stuff.
    This defines the canvas renderer
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::hash::Hash;
use std::ptr;
use std::mem;
use std::ops::{Deref, DerefMut};

use winapi::{FLOAT, D2D1_RECT_F, D2D1_ROUNDED_RECT, D2D1_ELLIPSE, D2D1_POINT_2F, 
  D2D1_MATRIX_3X2_F};

use error::Error;
use defs::{Rectangle, Ellipse};
use super::{Canvas, CanvasProtected, CanvasResources};

/**
    Object that offers a light wrapper over the D2D1 api.
*/
pub struct CanvasRenderer<'a, ID: Clone+Hash+'a> {
    canvas: &'a mut Canvas<ID>
}

impl<'a, ID: Clone+Hash> CanvasRenderer<'a, ID> {
    /// Clears the drawing area to the specified color. 
    pub fn clear(&mut self, r:f32, g:f32, b: f32, a: f32) {
        use winapi::D2D1_COLOR_F;
        let color = D2D1_COLOR_F{r:r, g:g, b:b, a:a};
        unsafe{ self.Clear(&color); }
    }

    /// Return the render target size
    pub fn get_render_size(&mut self) -> (f32, f32) {
        use winapi::D2D1_SIZE_F;
        let mut render_size = D2D1_SIZE_F{width: 0.0, height: 0.0};
        unsafe{ self.GetSize(&mut render_size); }
        
        (render_size.width as f32, render_size.height as f32)
    }

    /// Set the transformation matrix of the renderer
    pub fn set_transform(&mut self, m: [[FLOAT; 2]; 3]) {
        unsafe{ self.SetTransform( &D2D1_MATRIX_3X2_F{ matrix: m } ); }
    }

    /// Set the transformation matrix of the renderer
    pub fn get_transform(&mut self) -> [[FLOAT; 2]; 3] {
        unsafe{ 
            let mut m: D2D1_MATRIX_3X2_F = mem::uninitialized();
            self.GetTransform( &mut m );
            m.matrix
        }
    }

    /// Fill a rectangle in the render target with the brush identified by `brush`
    pub fn fill_rectangle(&mut self, brush: &ID, r: &Rectangle) -> Result<(), Error> {
        let rect = D2D1_RECT_F{left: r.left, top: r.top, bottom: r.bottom, right: r.right};
        let brush = match self.get_resource(brush) {
            Ok(b) => b,
            Err(e) => { return Err(e); }
        };

        match brush {
            CanvasResources::SolidBrush(b) => { unsafe{ self.FillRectangle(&rect, mem::transmute(b) ); } }
        }

        Ok(())
    }

    /// Fill a rounded rectangle in the render target with the brush identified by `brush`
    pub fn fill_rounded_rectangle(&mut self, brush: &ID, r: &Rectangle, radius: (f32, f32)) -> Result<(), Error> {
        let rect = D2D1_RECT_F{left: r.left, top: r.top, bottom: r.bottom, right: r.right};
        let rect = D2D1_ROUNDED_RECT{ rect: rect, radiusX: radius.0, radiusY: radius.1 };
        let brush = match self.get_resource(brush) {
            Ok(b) => b,
            Err(e) => { return Err(e); }
        };

        match brush {
            CanvasResources::SolidBrush(b) => { unsafe{ self.FillRoundedRectangle(&rect, mem::transmute(b) ); } }
        }

        Ok(())
    }

    /// Fill an ellipse in the render target with the brush identifier by `brush`
    pub fn fill_ellipse(&mut self, brush: &ID, e: &Ellipse) -> Result<(), Error> {
        let ellipse = D2D1_ELLIPSE{point: D2D1_POINT_2F{ x: e.center.0, y: e.center.1 } , radiusX: e.radius.0, radiusY: e.radius.1};
        let brush = match self.get_resource(brush) {
            Ok(b) => b,
            Err(e) => { return Err(e); }
        };

        match brush {
            CanvasResources::SolidBrush(b) => { unsafe{ self.FillEllipse(&ellipse, mem::transmute(b) ); } }
        }

        Ok(())
    }

    /// Draw the outlines of a rectangle in the render target with the brush identified by `brush`
    pub fn draw_rectangle(&mut self, brush: &ID, r: &Rectangle, width: f32, style: Option<()>) -> Result<(), Error> {
        let rect = D2D1_RECT_F{left: r.left, top: r.top, bottom: r.bottom, right: r.right};
        let brush = match self.get_resource(brush) {
            Ok(b) => b,
            Err(e) => { return Err(e); }
        };

        match brush {
            CanvasResources::SolidBrush(b) => { unsafe{ self.DrawRectangle(&rect, mem::transmute(b), width, ptr::null_mut()); } }
        }

        Ok(())
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
    fn prepare(canvas: &'a mut Canvas<ID>) -> Result<CanvasRenderer<'a, ID>, Error>;
}

impl<'a, ID: Clone+Hash> RendererProtected<'a, ID> for CanvasRenderer<'a, ID> {

    fn prepare(canvas: &'a mut Canvas<ID>) -> Result<CanvasRenderer<'a, ID>, Error> {
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

        Ok( CanvasRenderer { canvas: canvas } )
    }

}