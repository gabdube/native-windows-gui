/*!
Canvas are areas that can be freely painted to using NWG canvas API

Canvas use Direct2D. Direct2D is a hardware-accelerated, immediate-mode, 2-D graphics API that provides
high performance and high-quality rendering for 2-D geometry, bitmaps, and text. The Direct2D API is designed
to interoperate well with GDI, GDI+, and Direct3D.
*/
mod resources;
mod canvas_window;

pub use resources::*;
pub use canvas_window::{CanvasWindow, CanvasWindowFlags};
