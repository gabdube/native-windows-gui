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
use std::ops::{Deref, DerefMut};

use winapi::ID2D1SolidColorBrush;

use error::Error;
use super::{Canvas, CanvasProtected};

/**
    D2d resources held by a canvas
*/
pub enum CanvasResources {
    SolidBrush(*mut ID2D1SolidColorBrush)
}

/**
    Object that offers a light wrapper over the D2D1 api.
    For now most of the functions are unsafe
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
            use winapi::D2D1_MATRIX_3X2_F;

            if canvas.get_must_recreate_target() {
                if let Err(e) = canvas.rebuild() {
                    return Err(Error::System(e));
                }
            }

            let identity = D2D1_MATRIX_3X2_F {
                matrix: [[0.0, 0.0],
                         [1.0, 0.0],
                         [1.0, 1.0]]
            };

            canvas.BeginDraw(); 
            canvas.SetTransform(&identity);
        }

        Ok( CanvasRenderer { canvas: canvas } )
    }

}