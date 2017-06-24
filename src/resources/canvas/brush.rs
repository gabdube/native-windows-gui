/*!
    Brush resources for a canvas control
*/
use std::hash::Hash;
use std::any::TypeId;

use super::defs::{BrushType, SolidBrush};
use controls::{Canvas, AnyHandle, ControlType};
use resources::{ResourceT, Resource};
use error::{Error, SystemError};
use ui::Ui;

use winapi::{ID2D1SolidColorBrush};

/**
    A template that creates a brush used by a Canvas control. The resource is unpacked with the canvas.

    Params:  
    • `canvas`: The canvas control that will use the resource
    • `btype`: The type of the brush to create. 
*/
#[derive(Clone)]
pub struct BrushT<ID: Hash+Clone> {
    pub canvas: ID,
    pub btype: BrushType
}

impl<ID: Hash+Clone> ResourceT<ID> for BrushT<ID> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Brush>()
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Resource>, Error> {

        match ui.type_of_control(&self.canvas) {
            Ok(ControlType::Canvas) => {/* All good */},
            Ok(t) => { return Err(Error::BadParent( format!("A brush resource canvas must be a Canvas control. Got {:?}", t) )); }
            Err(e) => { return Err(e); }
        }

        let rt = match ui.get::<Canvas<ID>>(&self.canvas) {
            Ok(c) => c.get_render_target(),
            Err(_) => { unreachable!(); } // ui.type_of_control already check this
        };

        let rt = unsafe{&mut * rt};
        let handle = match &self.btype {
            &BrushType::SolidBrush(ref c) => create_solid_brush(rt, c)
        };

        match handle {
            Ok(h) => { 
                Ok(Box::new( Brush{ handle: h } ))
            },
            Err(e) => { Err(e) }
        }
    }
}


/**
    A brush resource
*/
pub struct Brush {
    handle: BrushHandle
}

#[derive(Clone, Copy)]
enum BrushHandle {
    SolidBrush(*mut ID2D1SolidColorBrush)
}

impl Resource for Brush {

    /**
        Should return the underlying handle to the object
    */
    fn handle(&self) -> AnyHandle {
        let handle = match self.handle {
            BrushHandle::SolidBrush(h) => h as usize
        };

        AnyHandle::Custom(TypeId::of::<Brush>(), handle)
    }

    /**
        If specified, should free any ressource allocated in the template `build` function.
    */
    fn free(&mut self) {
        // TODO
    }

}


// Private functions
use winapi::{ID2D1HwndRenderTarget, D2D1_MATRIX_3X2_F, S_OK};
use std::ptr;

fn create_solid_brush(rt: &mut ID2D1HwndRenderTarget, b: &SolidBrush) -> Result<BrushHandle, Error> {
    use winapi::{D2D1_COLOR_F, D2D1_BRUSH_PROPERTIES};

    let c = &b.color;
    let color = D2D1_COLOR_F{r: c.0, g: c.1, b: c.2, a: c.3};
    let identity = D2D1_MATRIX_3X2_F {matrix: [[1.0, 0.0],[0.0, 1.0],[0.0, 0.0]]};
    let property = D2D1_BRUSH_PROPERTIES { opacity: 1.0, transform: identity};
    let mut brush: *mut ID2D1SolidColorBrush = ptr::null_mut();
    let result = unsafe{ rt.CreateSolidColorBrush(&color, &property, &mut brush) };

    if result == S_OK {
        Ok(BrushHandle::SolidBrush(brush))
    } else {
        Err(Error::System(SystemError::ComError("Failed to import brush".to_string())))
    }
}