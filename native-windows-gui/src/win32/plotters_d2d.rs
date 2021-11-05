/*!
    Direct2D backend for the plotters control
*/
use winapi::shared::windef::HWND;
use winapi::um::d2d1::*;
use winapi::um::dwrite::{IDWriteFactory, IDWriteTextFormat, DWriteCreateFactory, DWRITE_FACTORY_TYPE_SHARED};
use winapi::shared::winerror::{S_OK, D2DERR_RECREATE_TARGET};

use super::base_helper::to_utf16;
use super::{high_dpi, window_helper};
use std::{cell::{Ref, RefCell, RefMut}, collections::HashMap, mem, ptr};

use plotters::prelude::DrawingBackend;
use plotters_backend::{DrawingErrorKind, BackendColor, BackendStyle, BackendTextStyle, BackendCoord};



#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl From<&BackendColor> for Color {
    fn from(c: &BackendColor) -> Self {
        let (r, g, b) = c.rgb;
        let a = (c.alpha.clamp(0.0, 1.0) * 255.0) as u8;
        Color { r, g, b, a }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct FontFormat {
    family: String,
    size: u32,
    style: u32,
    is_bold: bool
}

impl<T: BackendTextStyle> From<&T> for FontFormat {

    fn from(text: &T) -> Self {
        use plotters_backend::{FontFamily::*, FontStyle::*};
        use winapi::um::dwrite::{DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_STYLE_OBLIQUE, DWRITE_FONT_STYLE_ITALIC};

        let size = (text.size() * 100.0) as u32;

        let family = match text.family() {
            Serif => { "Georgia" },
            SansSerif => { "Arial" },
            Monospace => { "Courier New" },
            Name(name) => { name }
        };

        let mut is_bold = false;
        let style = match text.style() {
            Normal => DWRITE_FONT_STYLE_NORMAL,
            Oblique => DWRITE_FONT_STYLE_OBLIQUE,
            Italic => DWRITE_FONT_STYLE_ITALIC,
            Bold => {
                is_bold = true;
                DWRITE_FONT_STYLE_NORMAL
            },
        };

        FontFormat {
            family: family.to_owned(),
            size,
            style,
            is_bold
        }
    }
}


/**
    Errors that can be returned when using the Plotters control
*/
#[derive(Debug, Clone)]
pub enum PlottersError {
    RendererInit(String),
    Uninitialized,
    Unknown,
}

impl std::fmt::Display for PlottersError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PlottersError::*;
        match self {
            RendererInit(reason) => write!(f, "Plotters inner canvas creation failed: {}", reason),
            Uninitialized => write!(f, "The plotters canvas is not initialized"),
            Unknown => write!(f, "An unexpected error occured"),
        }
    }

}

impl std::error::Error for PlottersError {

}

struct PixelBitmap {
    memory: Vec<u8>,
    size: (u32, u32),
    bitmap: *mut ID2D1Bitmap
}

struct Target {
    render_target: *mut ID2D1HwndRenderTarget,
    brushes: HashMap<Color, *mut ID2D1SolidColorBrush>,

    // Target to draw pixels if draw_pixel is called
    pixel_bitmap: Option<PixelBitmap>,
    write_pixels: bool,

    size: (u32, u32),
    last_error: i32,
}

impl Target {
    fn fetch_brush(&mut self, color: Color) -> *mut ID2D1SolidColorBrush {
        let render_target = unsafe { &*self.render_target };
        let brush = self.brushes.entry(color)
            .or_insert_with(|| {
                let mut brush = ptr::null_mut();
                let props  = D2D1_BRUSH_PROPERTIES {
                    opacity: 1.0,
                    transform: D2D1_MATRIX_3X2_F {
                        matrix: [[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]]
                    }
                };

                let [r, g, b, a] = [
                    color.r as f32 / 255.0,
                    color.g as f32 / 255.0,
                    color.b as f32 / 255.0,
                    color.a as f32 / 255.0,
                ];

                unsafe {
                    render_target.CreateSolidColorBrush(
                        &D2D1_COLOR_F { r, g, b, a },
                        &props,
                        &mut brush
                    );
                }

                brush
            });

        *brush
    }

    pub(crate) fn allocate_pixel_bitmap(&mut self, renderer: *mut ID2D1Factory) {
        use winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
        use winapi::um::dcommon::{D2D1_PIXEL_FORMAT, D2D1_ALPHA_MODE_PREMULTIPLIED};

        if let Some(bitmap) = self.pixel_bitmap.as_ref() {
            let old_size = bitmap.size;
            let new_size = self.size;
            if old_size == new_size {
                return;
            }
        }

        let (width, height) = (self.size.0 as usize, self.size.1 as usize);
        let pixel_size = 4;

        let bitmap = unsafe {
            let mut bitmap = ptr::null_mut();

            let mut dpi_x = 0.0;
            let mut dpi_y = 0.0;
            (&*renderer).GetDesktopDpi(&mut dpi_x, &mut dpi_y);

            (&*self.render_target).CreateBitmap(
                D2D1_SIZE_U { width: width as _, height: height as _ },
                ptr::null(),
                0,
                &D2D1_BITMAP_PROPERTIES {
                    pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_R8G8B8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
                    dpiX: dpi_x,
                    dpiY: dpi_y,
                },
                &mut bitmap,
            );

            bitmap
        };

        self.pixel_bitmap = Some(PixelBitmap {
            memory: vec![0; width*height*pixel_size],
            size: self.size,
            bitmap
        });
    }

}

impl Drop for Target {

    fn drop(&mut self) {
        unsafe {
            for &brush in self.brushes.values() {
                (&*brush).Release();
            }

            if !self.render_target.is_null() {
                (&*self.render_target).Release();
            }
        }
    }

}

/**
    Direct2D backend for the plotters control
*/
pub struct PlottersBackend {
    renderer: *mut ID2D1Factory,
    write_factory: *mut IDWriteFactory,
    text_formats: RefCell<HashMap<FontFormat, *mut IDWriteTextFormat>>,
    target: RefCell<Target>,
    simple_stroke_style: *mut ID2D1StrokeStyle,
}

impl PlottersBackend {

    pub(crate) fn init(handle: HWND) -> Result<PlottersBackend, PlottersError> {
        unsafe {
            build_renderer(handle)
        }
    }

    pub(crate) fn begin_draw(&self) {
        unsafe {
            let target = self.target();
            (&*target.render_target).BeginDraw();
        }
    }

    pub(crate) fn end_draw(&self) {
        let result = unsafe {
            let mut target = self.target_mut();

            // Writes the pixel bitmap if needed
            if target.write_pixels {
                if let Some(bitmap) = target.pixel_bitmap.as_ref() {
                    let (width, height) = bitmap.size;

                    let copy_rect = D2D1_RECT_U {
                        top: 0, left: 0,
                        bottom: height, right: width,
                    };

                    let draw_rect = D2D1_RECT_F {
                        top: 0.0, left: 0.0,
                        bottom: height as f32, right: width as f32,
                    };

                    (&*bitmap.bitmap).CopyFromMemory(&copy_rect, bitmap.memory.as_ptr() as _, width*4);
                    (&*target.render_target).DrawBitmap(
                        bitmap.bitmap,
                        &draw_rect,
                        1.0,
                        D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
                        &draw_rect
                    );
                }

                target.write_pixels = false;
            }

            (&*target.render_target).EndDraw(ptr::null_mut(), ptr::null_mut())
        };

        match result {
            S_OK => { /* All good */ },
            e => {
                self.target_mut().last_error = e;
            }
        }
    }

    pub(crate) fn clear(&self) {
        unsafe {
            let target = self.target();
            (&*target.render_target).Clear(&D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 });
        }
    }

    /// Rebuilds the inner target if needed
    pub(crate) fn rebuild(&self, handle: HWND) -> Result<(), PlottersError> {
        let mut target = self.target_mut();
        let new_size = unsafe { client_size(handle) };
        if target.size != new_size || target.last_error == D2DERR_RECREATE_TARGET {
            *target = unsafe { build_render_target(handle, &mut *self.renderer)? };
        }


        target.allocate_pixel_bitmap(self.renderer);

        Ok(())
    }

    fn target(&self) -> Ref<Target> {
        self.target.borrow()
    }

    fn fetch_text_format(&self, mut fmt: FontFormat) -> *mut IDWriteTextFormat {
        use winapi::um::dwrite::{DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_WEIGHT_BOLD, DWRITE_FONT_STRETCH_NORMAL};

        let write_factory = unsafe { &*self.write_factory };

        // Setting a font with a size lesser than 100 will segfault direct2D
        fmt.size = fmt.size.max(100);

        let mut formats = self.text_formats.borrow_mut();
        let text_format = formats.entry(fmt.clone())
            .or_insert_with(move || {
                let mut text_format = ptr::null_mut();

                let font_size = (fmt.size as f32) / 100.0;
                let family_name = to_utf16(&fmt.family);
                let locale = unsafe { locale_name() };

                let weight = match fmt.is_bold {
                    true => DWRITE_FONT_WEIGHT_BOLD,
                    false => DWRITE_FONT_WEIGHT_NORMAL,
                };

                unsafe {
                    write_factory.CreateTextFormat(
                        family_name.as_ptr(),
                        ptr::null_mut(),
                        weight,
                        fmt.style,
                        DWRITE_FONT_STRETCH_NORMAL,
                        font_size,
                        locale.as_ptr(),
                        &mut text_format
                    );
                }

                text_format
            });

        *text_format
    }

    fn target_mut(&self) -> RefMut<Target> {
        self.target.borrow_mut()
    }

}

impl Drop for PlottersBackend {

    fn drop(&mut self) {
        unsafe {
            let formats = self.text_formats.borrow();
            for &fmt in formats.values() {
                (&*fmt).Release();
            }

            if !self.simple_stroke_style.is_null() {
                (&*self.simple_stroke_style).Release();
            }

            if !self.renderer.is_null() {
                (&*self.renderer).Release();
            }

            if !self.write_factory.is_null() {
                (&*self.write_factory).Release();
            }
        }
    }

}

impl<'a> DrawingBackend for &'a PlottersBackend {
    type ErrorType = PlottersError;

    fn get_size(&self) -> (u32, u32) {
        let (width, height) = self.target().size;
        let (width, height) = unsafe { high_dpi::physical_to_logical(width as i32, height as i32) };
        (width as u32, height as u32)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: (i32, i32),
        color: BackendColor
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut target = self.target.borrow_mut();

        // Allocate the pixel bitmap if it was not allocated before
        target.allocate_pixel_bitmap(self.renderer);

        // Tells the API to write the pixel bitmap on top of the canvas at the end of the drawing
        target.write_pixels = true;

        // Write the pixel to the bitmap
        let (x, y) = (point.0 as usize, point.1 as usize);
        let width = target.size.0 as usize;
        let pixels = target.pixel_bitmap.as_mut().unwrap();
        let pixel_offset = ((width * y * 4) + (x * 4)).max(0);
        pixels.memory[pixel_offset] = color.rgb.0;
        pixels.memory[pixel_offset+1] = color.rgb.1;
        pixels.memory[pixel_offset+2] = color.rgb.2;
        pixels.memory[pixel_offset+3] = 255;

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: (i32, i32),
        to: (i32, i32),
        style: &S
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut target = self.target_mut();
        let stroke_width = style.stroke_width() as f32;
        let brush = target.fetch_brush(Color::from(&style.color()));

        unsafe {
            let p0 = D2D1_POINT_2F { x: from.0 as f32, y: from.1 as f32 };
            let p1 = D2D1_POINT_2F { x: to.0 as f32, y: to.1 as f32 };
            (&*target.render_target).DrawLine(
                p0,
                p1,
                brush as _,
                stroke_width,
                self.simple_stroke_style,
            );
        }

        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: (i32, i32),
        bottom_right: (i32, i32),
        style: &S,
        fill: bool
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut target = self.target_mut();
        let stroke_width = style.stroke_width() as f32;
        let brush = target.fetch_brush(Color::from(&style.color()));

        let left = upper_left.0 as f32;
        let right = bottom_right.0 as f32;
        let bottom = bottom_right.1 as f32;
        let top = upper_left.1 as f32;

        unsafe {
            let rect = D2D1_RECT_F {
                left,
                top,
                right: right.max(left),
                bottom: bottom.max(top),
            };

            match fill {
                true => {
                    (&*target.render_target).FillRectangle(
                        &rect,
                        brush as _
                    );
                },
                false => {
                    (&*target.render_target).DrawRectangle(
                        &rect,
                        brush as _,
                        stroke_width,
                        self.simple_stroke_style
                    )
                }
            }

        }

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = (i32, i32)>>(
        &mut self,
        path: I,
        style: &S
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut target = self.target_mut();
        let stroke_width = style.stroke_width() as f32;
        let brush = target.fetch_brush(Color::from(&style.color()));

        let mut iter_path = path.into_iter();
        let (mut last_x, mut last_y) = iter_path.next().unwrap_or((0, 0));
        for (x, y) in iter_path {
            let p0 = D2D1_POINT_2F { x: last_x as f32, y: last_y as f32 };
            let p1 = D2D1_POINT_2F { x: x as f32, y: y as f32 };

            unsafe {
                (&*target.render_target).DrawLine(
                    p0,
                    p1,
                    brush as _,
                    stroke_width,
                    self.simple_stroke_style,
                );
            }

            last_x = x;
            last_y = y;
        }

        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: (i32, i32),
        radius: u32,
        style: &S,
        fill: bool
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut target = self.target_mut();
        let stroke_width = style.stroke_width() as f32;
        let brush = target.fetch_brush(Color::from(&style.color()));

        unsafe {
            let ellipse = D2D1_ELLIPSE {
                point: D2D1_POINT_2F { x: center.0 as f32, y: center.1 as f32 },
                radiusX: radius as f32,
                radiusY: radius as f32,
            };

            match fill {
                true => {
                    (&*target.render_target).FillEllipse(
                        &ellipse,
                        brush as _
                    );
                },
                false => {
                    (&*target.render_target).DrawEllipse(
                        &ellipse,
                        brush as _,
                        stroke_width,
                        self.simple_stroke_style
                    )
                }
            }

        }

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {

        let mut target = self.target_mut();
        let brush = target.fetch_brush(Color::from(&style.color()));

        unsafe {
            let fact = &*self.renderer;
            let mut path = ptr::null_mut();
            let mut sink = ptr::null_mut();


            fact.CreatePathGeometry(&mut path);
            (&*path).Open(&mut sink);

            let mut vertex_iter = vert.into_iter();
            let (px, py) = vertex_iter.next().unwrap_or((0, 0));
            (&*sink).BeginFigure(D2D1_POINT_2F { x: px as f32, y: py as f32 }, D2D1_FIGURE_BEGIN_FILLED);

            for (px, py) in vertex_iter {
                (&*sink).AddLine(D2D1_POINT_2F { x: px as f32, y: py as f32 });
            }

            (&*sink).EndFigure(D2D1_FIGURE_END_CLOSED);
            (&*sink).Close();


            (&*target.render_target).FillGeometry(
                path as _,
                brush as _,
                ptr::null_mut(),
            );


            (&*sink).Release();
            (&*path).Release();
        }

        Ok(())
    }

    fn draw_text<TStyle: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &TStyle,
        pos: (i32, i32)
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        use winapi::um::dcommon::DWRITE_MEASURING_MODE_NATURAL;
        use plotters::style::text_anchor::{HPos, VPos};

        let mut target = self.target_mut();
        let brush = target.fetch_brush(Color::from(&style.color()));

        let text_format = self.fetch_text_format(FontFormat::from(style));
        let raw_text = to_utf16(text);
        let (width, height) = target.size;

        let [min_x, min_y, max_x, max_y] = style
            .layout_box(text)
            .map(|((min_x, min_y), (max_x, max_y))| [min_x, min_y, max_x, max_y] )
            .unwrap_or([0, 0, 0, 0]);

        let text_width = (max_x - min_x) as i32;
        let text_height = (max_y - min_y) as i32;

        let dx = match style.anchor().h_pos {
            HPos::Left => 0,
            HPos::Right => -text_width,
            HPos::Center => -text_width / 2,
        };
        let dy = match style.anchor().v_pos {
            VPos::Top => 0,
            VPos::Center => -text_height / 2,
            VPos::Bottom => -text_height,
        };


        let (x, y) = (pos.0 + dx, pos.1 + dy);

        let layout_rect = D2D1_RECT_F {
            left: x as f32,
            top: y as f32,
            right: width as f32,
            bottom: height as f32,
        };

        unsafe {
            (&*target.render_target).DrawText(
                raw_text.as_ptr(),
                (raw_text.len() - 1) as _,
                text_format,
                &layout_rect,
                brush as _,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
        }

        Ok(())
    }

    fn estimate_text_size<TStyle: BackendTextStyle>(
        &self,
        text: &str,
        style: &TStyle
    ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
        use winapi::um::dwrite::DWRITE_TEXT_METRICS;

        let text_format = self.fetch_text_format(FontFormat::from(style));
        let text = to_utf16(text);

        let [width, height]: [u32; 2];

        unsafe {
            let write = &*self.write_factory;
            let mut layout = ptr::null_mut();

            write.CreateTextLayout (
                text.as_ptr(),
                text.len() as _,
                text_format,
                1000.0,
                1000.0,
                &mut layout,
            );

            let layout = &*layout;
            let mut metrics: DWRITE_TEXT_METRICS = mem::zeroed();
            layout.GetMetrics(&mut metrics);

            width = metrics.width as u32;
            height = metrics.height as u32;

            layout.Release();
        }


        Ok((width, height))
    }

    fn blit_bitmap<'b>(
        &mut self,
        _pos: (i32, i32),
        (_iw, _ih): (u32, u32),
        _src: &'b [u8]
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

}

unsafe fn build_render_target(hwnd: HWND, factory: &mut ID2D1Factory) -> Result<Target, PlottersError> {
    use winapi::shared::dxgiformat::{DXGI_FORMAT_B8G8R8A8_UNORM};
    use winapi::um::dcommon::{D2D_SIZE_U, D2D1_PIXEL_FORMAT, D2D1_ALPHA_MODE_PREMULTIPLIED};

    let (width, height) = client_size(hwnd);
    let size = D2D_SIZE_U { width, height };

    let pixel_format = D2D1_PIXEL_FORMAT {
        format: DXGI_FORMAT_B8G8R8A8_UNORM,
        alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED
    };

    let render_props = D2D1_RENDER_TARGET_PROPERTIES {
        _type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
        pixelFormat: pixel_format,
        dpiX: 0.0, dpiY: 0.0,
        usage: D2D1_RENDER_TARGET_USAGE_NONE,
        minLevel: D2D1_FEATURE_LEVEL_DEFAULT
    };

    let hwnd_render_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
        hwnd: hwnd,
        pixelSize: size,
        presentOptions: D2D1_PRESENT_OPTIONS_NONE
    };

    let mut render_target: *mut ID2D1HwndRenderTarget = ptr::null_mut();
    if factory.CreateHwndRenderTarget(&render_props, &hwnd_render_props, &mut render_target) != S_OK {
        factory.Release();
        Err(PlottersError::RendererInit("Failed to create the direct2D render target".into()))
    } else {
        Ok(Target {
            render_target,
            brushes: Default::default(),
            pixel_bitmap: None,
            write_pixels: false,
            size: (width, height),
            last_error: S_OK
        })
    }
}

unsafe fn build_static_resources(backend: &mut PlottersBackend) -> Result<(), PlottersError> {
    let f = &mut *backend.renderer;

    let props = D2D1_STROKE_STYLE_PROPERTIES {
        startCap: D2D1_CAP_STYLE_ROUND,
        endCap: D2D1_CAP_STYLE_ROUND,
        dashCap: D2D1_CAP_STYLE_ROUND,
        lineJoin: D2D1_LINE_JOIN_MITER,
        miterLimit: 0.0,
        dashStyle: D2D1_DASH_STYLE_SOLID,
        dashOffset: 0.0,
    };

    f.CreateStrokeStyle(
        &props,
        ptr::null(),
        0,
        &mut backend.simple_stroke_style
    );

    Ok(())
}

unsafe fn client_size(hwnd: HWND) -> (u32, u32) {
    window_helper::get_window_physical_size(hwnd)
}

unsafe fn locale_name() -> Vec<u16> {
    use winapi::um::winnls::GetUserDefaultLocaleName;
    use winapi::um::winnt::LOCALE_NAME_MAX_LENGTH;

    let mut name_buffer: Vec<u16> = vec![0; LOCALE_NAME_MAX_LENGTH];
    GetUserDefaultLocaleName(name_buffer.as_mut_ptr(), LOCALE_NAME_MAX_LENGTH as i32);

    name_buffer
}

unsafe fn build_renderer(handle: HWND) -> Result<PlottersBackend, PlottersError> {
    use winapi::ctypes::c_void;
    use winapi::Interface;

    // Build the write factory
    let mut write_factory: *mut IDWriteFactory = ptr::null_mut();
    let result = DWriteCreateFactory (
        DWRITE_FACTORY_TYPE_SHARED,
        &IDWriteFactory::uuidof(),
        (&mut write_factory as *mut *mut IDWriteFactory) as _
    );
    if result != S_OK {
        return Err(PlottersError::RendererInit("Failed to create the direct2D factory".into()));
    }

    // Build the factory
    let mut renderer: *mut ID2D1Factory = ptr::null_mut();
    let result = D2D1CreateFactory(
        D2D1_FACTORY_TYPE_SINGLE_THREADED,
        &ID2D1Factory::uuidof(),
        ptr::null(),
        (&mut renderer as *mut *mut ID2D1Factory) as *mut *mut c_void
    );

    if result != S_OK {
        (&*write_factory).Release();
        return Err(PlottersError::RendererInit("Failed to create the direct2D factory".into()));
    }

    // Build the render target
    let target = match build_render_target(handle, &mut *renderer) {
        Ok(target) => target,
        e @ Err(_) => {
            (&*renderer).Release();
            (&*write_factory).Release();
            e?
        }
    };

    let mut renderer = PlottersBackend {
        renderer,
        write_factory,
        text_formats: RefCell::new(Default::default()),
        target: RefCell::new(target),
        simple_stroke_style: ptr::null_mut(),
    };

    // Build static resources
    build_static_resources(&mut renderer)?;

    Ok(renderer)
}
