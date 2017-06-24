/*!
    Pen resources for a canvas control
*/
use std::hash::Hash;
use std::any::TypeId;

use super::defs::{CapStyle, LineJoin, DashStyle};
use controls::{Canvas, AnyHandle, ControlType};
use resources::{ResourceT, Resource};
use error::{Error, SystemError};
use ui::Ui;

use winapi::{ID2D1StrokeStyle};

/**
    A template that creates a brush used by a Canvas control. The resource is unpacked with the canvas.

    Params:  
    • `canvas`: The canvas control that will use the resource
    • `btype`: The type of the brush to create. 
*/
#[derive(Clone)]
pub struct PenT<ID: Hash+Clone> {
    pub canvas: ID,
    pub start_cap: CapStyle,
    pub end_cap: CapStyle,
    pub dash_cap: CapStyle,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub dash_style: DashStyle,
    pub dash_offset: f32
}

impl<ID: Hash+Clone> ResourceT<ID> for PenT<ID> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Pen>()
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Resource>, Error> {

        match ui.type_of_control(&self.canvas) {
            Ok(ControlType::Canvas) => {/* All good */},
            Ok(t) => { return Err(Error::BadParent( format!("A Pen resource canvas must be a Canvas control. Got {:?}", t) )); }
            Err(e) => { return Err(e); }
        }

        let factory = match ui.get::<Canvas<ID>>(&self.canvas) {
            Ok(c) => c.get_factory(),
            Err(_) => { unreachable!(); } // ui.type_of_control already check this
        };

        let factory = unsafe{&mut * factory};
        match create_pen(self, factory) {
            Ok(h) => { 
                Ok(Box::new( Pen{ handle: h } ))
            },
            Err(e) => { Err(e) }
        }
    }
}


/**
    A brush resource
*/
pub struct Pen {
    handle: *mut ID2D1StrokeStyle
}

impl Resource for Pen {

    /**
        Should return the underlying handle to the object
    */
    fn handle(&self) -> AnyHandle {
        AnyHandle::Custom(TypeId::of::<Pen>(), self.handle as usize)
    }

    /**
        If specified, should free any ressource allocated in the template `build` function.
    */
    fn free(&mut self) {
        // TODO
    }

}


// Private functions
use winapi::{ID2D1Factory, S_OK};
use std::ptr;

fn create_pen<ID: Hash+Clone>(pen: &PenT<ID>, factory: &mut ID2D1Factory) -> Result<*mut ID2D1StrokeStyle, Error> {
    use winapi::{D2D1_STROKE_STYLE_PROPERTIES, D2D1_CAP_STYLE, D2D1_LINE_JOIN, D2D1_DASH_STYLE};

    let pen = pen.clone();
    let start_cap = D2D1_CAP_STYLE(pen.start_cap as u32);
    let end_cap = D2D1_CAP_STYLE(pen.end_cap as u32);
    let dash_cap = D2D1_CAP_STYLE(pen.dash_cap as u32);
    let line_join = D2D1_LINE_JOIN(pen.line_join as u32);
    let dash_style = D2D1_DASH_STYLE(pen.dash_style as u32);
    let stroke_style_prop = D2D1_STROKE_STYLE_PROPERTIES {
        startCap: start_cap,
        endCap: end_cap,
        dashCap: dash_cap,
        lineJoin: line_join,
        miterLimit: pen.miter_limit,
        dashStyle: dash_style,
        dashOffset: pen.dash_offset
    };

    let mut stroke_style: *mut ID2D1StrokeStyle = ptr::null_mut();
    let result = unsafe{ factory.CreateStrokeStyle(&stroke_style_prop, ptr::null(), 0, &mut stroke_style) };

    if result == S_OK {
        Ok(stroke_style)
    } else {
        Err(Error::System(SystemError::ComError("Failed to import brush".to_string())))
    }
}