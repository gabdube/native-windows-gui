/*!
    Brush resources for a canvas control
*/
use std::hash::Hash;
use std::any::TypeId;

use super::defs::{BrushType};
use controls::{AnyHandle, ControlType};
use resources::{ResourceT, Resource};
use error::Error;
use ui::Ui;

use winapi::{ID2D1SolidColorBrush};

/**
    A template that creates a brush used by a Canvas control. The resource is unpacked with the canvas.

    Params:  
    • `canvas`: The canvas control that will use the resource
    • `btype`: The type of the brush to create. 
*/
pub struct BrushT<ID: Hash+Clone> {
    canvas: ID,
    btype: BrushType
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

        Err(Error::Unimplemented)
    }
}


/**
    A brush resource
*/
pub struct Brush {
    handle: CanvasHandle
}

#[derive(Clone, Copy)]
enum CanvasHandle {
    SolidBrush(*mut ID2D1SolidColorBrush)
}

impl Resource for Brush {

    /**
        Should return the underlying handle to the object
    */
    fn handle(&self) -> AnyHandle {
        let handle = match self.handle {
            CanvasHandle::SolidBrush(h) => h as usize
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

fn create_solid_brush() {
    
}