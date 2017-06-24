/*!
    A canvas control where the user can draw stuff.
    This defines the canvas control
*/

use std::hash::Hash;
use std::ptr;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use winapi::{HWND, ID2D1Factory, ID2D1HwndRenderTarget};

use controls::{Control, ControlType, AnyHandle};
use error::{Error, SystemError};
use super::{CanvasRenderer, RendererProtected, build_render_target, CANVAS_CLASS_NAME};
use ui::Ui;

/**
    A blank control that can be painted to
*/
pub struct Canvas<ID: Clone+Hash> {
    handle: HWND,
    factory: *mut ID2D1Factory,
    render_target: *mut ID2D1HwndRenderTarget,
    must_recreate_target: bool,
    p: PhantomData<ID>
}

impl<ID: Clone+Hash> Canvas<ID> {
    /**
        Make the canvas "paint ready" and return an object to paint to it.
        In very very **very** rare case, the renderer creation can fail.
    */
    pub fn renderer<'a>(&'a mut self, ui: &'a Ui<ID>) -> Result<CanvasRenderer<'a, ID>, Error> {
        CanvasRenderer::prepare(self, ui)
    }

    /**
        Redraw the canvas
    */
    pub fn redraw(&self) {
        use user32::RedrawWindow;
        use winapi::{RDW_ERASE, RDW_INVALIDATE};
        unsafe { 
            RedrawWindow(self.handle, ptr::null(), ptr::null_mut(), RDW_ERASE|RDW_INVALIDATE);
        }
    }
    
    /**
        Set the render target resolution.  
        If the control size do not match the render target size, the result will be upscaled or downscaled
    */
    pub fn set_render_size(&mut self, w: u32, h: u32) {
        use winapi::D2D_SIZE_U;
        let render_size = D2D_SIZE_U{width: w, height: h};
        unsafe{ self.Resize(&render_size); }
    }

    /**
        Return the render target's dots per inch (DPI).
    */
    pub fn get_dpi(&mut self) -> (f32, f32) {
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        unsafe { self.GetDpi(&mut x, &mut y); }
        (x, y)
    }

    /**
        Sets the dots per inch (DPI) of the render target.   

        Arguments:  
        • `dpix`:  A value greater than or equal to zero that specifies the horizontal DPI of the render target.  
        • `dpiy`:  A value greater than or equal to zero that specifies the vertical DPI of the render target.  
    */
    pub fn set_dpi(&mut self, dpix: f32, fpiy: f32) {
        unsafe { self.SetDpi(dpix, fpiy); }
    }

    /**
        Get the inner render target of the canvas. This is used when building canvas resources.
    */
    pub fn get_render_target(&self) -> *mut ID2D1HwndRenderTarget { self.render_target }

    /**
        Get the inner factory of the canvas. This is used when building canvas resources.
    */
    pub fn get_factory(&self) -> *mut ID2D1Factory { self.factory }

    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, true); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }
    pub fn update(&self) { unsafe{ ::low::window_helper::update(self.handle); } }
    pub fn focus(&self) { unsafe{ ::user32::SetFocus(self.handle); } }
}

impl<ID: Clone+Hash> Control for Canvas<ID> {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::Canvas 
    }

    fn free(&mut self) {
        unsafe{
            use user32::{DestroyWindow, UnregisterClassW};
            use kernel32::GetModuleHandleW;
            use low::other_helper::to_utf16;

            let factory = &mut *self.factory;
            let render_target = &mut *self.render_target;

            render_target.Release();
            factory.Release();
            DestroyWindow(self.handle);

            let cls = to_utf16(CANVAS_CLASS_NAME);
            let hmod = GetModuleHandleW(ptr::null_mut());
            UnregisterClassW(cls.as_ptr(), hmod);
        };
    }

}

impl<'a, ID: Clone+Hash> Deref for Canvas<ID> {
    type Target = ID2D1HwndRenderTarget;

    #[inline(always)]
    fn deref(&self) -> &ID2D1HwndRenderTarget {
        unsafe{ & *self.render_target }
    }

}

impl<'a, ID: Clone+Hash> DerefMut for Canvas<ID> {

    #[inline(always)]
    fn deref_mut(&mut self) -> &mut ID2D1HwndRenderTarget {
        unsafe{ &mut *self.render_target }
    }

}

/**
    Protected renderer method (only available in the canvas control module)
*/
pub trait CanvasProtected<ID: Clone+Hash>  {
    fn get_must_recreate_target(&mut self) -> bool;
    fn set_must_recreate_target(&mut self, recreate: bool);
    fn create(h: HWND, f: *mut ID2D1Factory, r: *mut ID2D1HwndRenderTarget) -> Canvas<ID>;
    fn rebuild(&mut self) -> Result<(), SystemError>;
}

impl<ID: Clone+Hash> CanvasProtected<ID> for Canvas<ID> {

    fn get_must_recreate_target(&mut self) -> bool {
        self.must_recreate_target
    }

    fn set_must_recreate_target(&mut self, recreate: bool) {
        self.must_recreate_target = recreate;
    }

    fn create(h: HWND, f: *mut ID2D1Factory, r: *mut ID2D1HwndRenderTarget) -> Canvas<ID> {
         Canvas::<ID>{
            handle: h,
            factory: f,
            render_target: r,
            must_recreate_target: false,
            p: PhantomData
        }
    }

    /// Rebuild the canvas renderer
    fn rebuild(&mut self) -> Result<(), SystemError> {
        let result = unsafe{ build_render_target(self.handle,  &mut *self.factory) };
        match result {
            Ok(render_target) => {
                self.render_target = render_target;
                self.must_recreate_target = false;
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

}