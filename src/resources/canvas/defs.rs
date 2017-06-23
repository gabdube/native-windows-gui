/*!
    Canvas resource constants
*/

/**
    Define a rectangle shape that can be used with canvases
*/
#[derive(Clone)]
pub struct Rectangle {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32
}

/**
    Define a ellipse shape that can be used with canvases
*/
#[derive(Clone)]
pub struct Ellipse {
    pub center: (f32, f32),
    pub radius: (f32, f32),
}

/**
    A enumeration of the different brush type that can be created using a `BrushT` resource.
*/
#[derive(Clone)]
pub enum BrushType {
    SolidBrush(SolidBrush)
}

/**
    A brush using a single solid color. Used when painting in a canvas

    Members:  
    • `color`: The brush color (red, green, blue, alpha)
*/
#[derive(Clone)]
pub struct SolidBrush {
    pub color: (f32, f32, f32, f32)
}

/**
    Cap style used when creating a Pen
*/
#[derive(Clone, Debug)]
pub enum CapStyle {
    Flat = 0,
    Square = 1,
    Round = 2,
    Triangle = 3
}

/**
    Line join type used when creating a Pen
*/
#[derive(Clone, Debug)]
pub enum LineJoin {
    Miter = 0,
    Bevel = 1,
    Round = 2,
    MiterOrBevel = 3
}

/**
    Dash style used when creating a Pen
*/
#[derive(Clone, Debug)]
pub enum DashStyle {
    Solid = 0,
    Dash = 1,
    Dot = 2,
    DashDot = 3,
    DashDotDot = 4,
}

/**
    Describe how lines should be painted. Used when painting in a canvas
    
    Members:  
    • `start_cap`: The cap applied to the start of all the open figures in a stroked geometry. 
    • `end_cap`: The cap applied to the end of all the open figures in a stroked geometry.
    • `dash_cap`: The shape at either end of each dash segment.
    • `line_join`: A value that describes how segments are joined. This value is ignored for a vertex if the segment flags specify that the segment should have a smooth join. 
    • `miter_limit`: The limit of the thickness of the join on a mitered corner. This value is always treated as though it is greater than or equal to 1.0f. 
    • `dash_style`: A value that specifies whether the stroke has a dash pattern and, if so, the dash style. 
    • `dash_offset`: A value that specifies an offset in the dash sequence. A positive dash offset value shifts the dash pattern, in units of stroke width,  
       toward the start of the stroked geometry. A negative dash offset value shifts the dash pattern, in units of stroke width, toward the end of the stroked geometry.
*/
#[derive(Clone, Debug)]
pub struct Pen {
    pub start_cap: CapStyle,
    pub end_cap: CapStyle,
    pub dash_cap: CapStyle,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub dash_style: DashStyle,
    pub dash_offset: f32,
}
