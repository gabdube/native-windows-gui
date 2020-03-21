use crate::nwg;
use crate::gl;
use crate::glutin::{
    ContextBuilder, GlRequest, GlProfile, PossiblyCurrent, RawContext, Api,
    dpi::PhysicalSize,
    os::windows::RawContextExt
};

use std::cell::RefCell;
use std::ffi::c_void;
use std::{ptr, mem};


/**
    A macro that loads the required opengl functions pointers.
*/
macro_rules! gl {
    ($ctx:expr, [$($fn:ident),+]) => (
        $(gl::$fn::load_with(|s| $ctx.get_proc_address(s) as *const _));+
    )
}

pub type Texel = [u8; 4];
type Ctx = RawContext<PossiblyCurrent>;


#[derive(Default, Copy, Clone)]
pub struct Texture {
    handle: u32,
    width: u32,
    height: u32
}


/**
    A wrapper over an extern canvas that adds support for opengl.
    The canvas supports drawing and resizing
*/
#[derive(Default)]
pub struct OpenGlCanvas {
    ctx: RefCell<Option<Ctx>>,
    texture: RefCell<Texture>,
    canvas: nwg::ExternCanvas,
}

// Allow the control type to be used with native-windows-gui and native-windows-derive
nwg::subclass_control!(OpenGlCanvas, ExternCanvas, canvas);

impl OpenGlCanvas {

    /// Create an opengl canvas with glutin & gl
    pub fn create_context(&self) -> Result<(), &'static str> {
        let (ctx, texture) = unsafe {
            let ctx = ContextBuilder::new()
                .with_gl(GlRequest::Specific(Api::OpenGl, (4, 3)))
                .with_gl_profile(GlProfile::Core)
                .build_raw_context(self.canvas.handle.hwnd().unwrap() as *mut c_void)
                .map_err(|_e| "Failed to build opengl context")?
                .make_current()
                .map_err(|_e| "Failed to set opengl context as current")?;

            // Load the opengl functions pointers.
            gl!( ctx, [
                Clear, ClearColor, Viewport, CreateShader, ShaderSource, CompileShader, CreateProgram, AttachShader, LinkProgram, UseProgram,
                GenBuffers, BindBuffer, BufferData, GenVertexArrays, BindVertexArray, EnableVertexAttribArray, VertexAttribPointer, DrawArrays,
                TexParameteri, TexParameterfv, GenTextures, BindTexture, TexStorage2D, TexSubImage2D, GetShaderiv, GetShaderInfoLog,
                GetProgramiv, GetProgramInfoLog, BindTexture, ActiveTexture, DeleteTextures, GetnTexImage, PixelStorei, CopyImageSubData
            ]);

            // Initial GL setup
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, [1.0, 1.0, 1.0, 1.0].as_ptr());

            // Shaders
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vs, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
            gl::CompileShader(vs);
            check_shader_status(vs);

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fs, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
            gl::CompileShader(fs);
            check_shader_status(fs);

            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
            gl::UseProgram(program);
            check_program_status(program);

            // Texture
            let (width, height) = self.canvas.size();

            let mut tex = 0;
            gl::GenTextures(1, &mut tex);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA8, width as _, height as _);
            clear_texture(width, height);

            // Mesh
            let vertex_data: &[f32] = &[
               -1.0,  1.0,   0.0, 0.0,
               -1.0, -1.0,   0.0, 1.0,
                1.0, -1.0,   1.0, 1.0,

               -1.0,  1.0,   0.0, 0.0,
                1.0,  1.0,   1.0, 0.0,
                1.0, -1.0,   1.0, 1.0,
            ];
            let vertex_size = vertex_data.len() * mem::size_of::<f32>();

            let mut vb = mem::zeroed();
            gl::GenBuffers(1, &mut vb);
            gl::BindBuffer(gl::ARRAY_BUFFER, vb);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                vertex_size as gl::types::GLsizeiptr,
                vertex_data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // VAO
            let mut vao = mem::zeroed();
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);

            let stride = mem::size_of::<f32>() * 4;
            let color_offset = 8 as *const c_void; 
            gl::VertexAttribPointer(0, 2, gl::FLOAT, 0, stride as i32, ptr::null());
            gl::VertexAttribPointer(1, 2, gl::FLOAT, 0, stride as i32, color_offset);

            let texture = Texture {
                handle: tex,
                width,
                height
            };

            (ctx, texture)
        };

        *self.ctx.borrow_mut() = Some(ctx);
        *self.texture.borrow_mut() = texture;

        Ok(())
    }

    /// Paints the pixel at `pos` with the selected color.
    pub fn paint(&self, pos: (i32, i32), color: [u8; 4]) {
        const BRUSH: i32 = 10;
        const HALF_BRUSH: i32 = BRUSH/2;

        let (mut x, mut y) = pos;
        x = (x)-HALF_BRUSH;
        y = (y)-HALF_BRUSH;

        let mut data = Vec::with_capacity((BRUSH*BRUSH) as usize);
        for _ in 0..(BRUSH*BRUSH) {
            data.push(color);
        }
        
        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,                        // mip level
                x, y,                     // offset 
                HALF_BRUSH , HALF_BRUSH,  //size
                gl::RGBA, gl::UNSIGNED_BYTE,
                data.as_ptr() as *mut c_void
            );
        }
    }

    /// Updates the canvas on the screen
    pub fn render(&self) {
        const VERTEX_COUNT: i32 = 6;

        self.ctx.borrow().as_ref().map(|ctx| unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, VERTEX_COUNT);
            ctx.swap_buffers().unwrap();
            self.canvas.invalidate()
        });
    }

    /// Resizes the opengl viewport and the canvas on the screen
    pub fn resize(&self) {
        self.ctx.borrow().as_ref().map(|ctx| unsafe {
            let (w, h) = self.canvas.size();
            gl::Viewport(0, 0, w as _, h as _);
            ctx.resize(PhysicalSize::new(w as f64, h as f64));
        });
    }

    /// Returns the current texture size
    pub fn texture_size(&self) -> (u32, u32) {
        let texture = self.texture.borrow();
        (texture.width, texture.height)
    }

    /// Return the canvas texture data
    pub fn texture_data(&self) -> Vec<Texel> {
        let tex = self.texture.borrow();
        let tex_size = (tex.width * tex.height) as usize;
        let buffer_size = tex_size * mem::size_of::<Texel>();
        let mut buffer: Vec<Texel> = Vec::with_capacity(tex_size);

        unsafe {
            buffer.set_len(tex_size);
            gl::GetnTexImage(gl::TEXTURE_2D, 0, gl::RGBA, gl::UNSIGNED_BYTE, buffer_size as _, buffer.as_mut_ptr() as *mut c_void);
        }

        buffer
    } 

    /// Sets the texture content of the canvas
    /// Panics if the size of data is not enough to fill the texture or if the new texture size do not match the old
    pub fn set_texture_data(&self, width: u32, height: u32, data: &[Texel]) {
        self.resize_texture(width, height, false);

        let texture = self.texture.borrow();
        
        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0, 0,
                texture.width as _, texture.height as _,
                gl::RGBA, gl::UNSIGNED_BYTE,
                data.as_ptr() as *mut c_void
            );
        }
    }

    /// Resize the underlying texture to match the canvas size
    /// If `preserve` is set to true, the content of the previous textuer will be copied into the new one
    pub fn resize_texture(&self, width: u32, height: u32, preserve: bool) {
        use std::cmp;

        let mut texture = self.texture.borrow_mut();

        unsafe {
            // Generate a new resized texture
            let mut tex = 0;
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA8, width as _, height as _);
            
            if preserve {
                clear_texture(width, height);

                let (copy_w, copy_h) = (cmp::min(texture.width, width), cmp::min(texture.height, height));
                gl::CopyImageSubData(
                    texture.handle,       // src
                    gl::TEXTURE_2D,
                    0,                    // src level
                    0, 0, 0,              // src XYZ
                    tex,                  // dst
                    gl::TEXTURE_2D,
                    0,                    // dst level
                    0, 0, 0,              // dst xyz
                    copy_w as _,          // width, height, depth
                    copy_h as _,
                    1
                );
            }
            
            // Free / update the application resources
            gl::DeleteTextures(1, &texture.handle);
            texture.handle = tex;
            texture.width = width;
            texture.height = height;
        }
    }

}

unsafe fn check_shader_status(shader: u32) {
    use std::ffi::CStr;

    let mut status = 1;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    if status == 0 {
        let mut error_length = 0;
        gl::GetShaderiv(shader, gl::SHADER_SOURCE_LENGTH, &mut error_length);

        error_length += 1;
        let mut logs: Vec<gl::types::GLchar> = Vec::with_capacity(error_length as usize);
        logs.set_len(error_length as usize);

        gl::GetShaderInfoLog(shader, error_length, &mut error_length, logs.as_mut_ptr());
        
        panic!("\n\n{}\n\n", CStr::from_ptr(logs.as_ptr()).to_str().unwrap());
    }
}

unsafe fn check_program_status(program: u32) {
    use std::ffi::CStr;

    let mut status = 1;
    gl::GetProgramiv(program, gl::COMPILE_STATUS, &mut status);
    if status == 0 {
        let mut error_length = 0;
        gl::GetProgramiv(program, gl::SHADER_SOURCE_LENGTH, &mut error_length);

        error_length += 1;
        let mut logs: Vec<gl::types::GLchar> = Vec::with_capacity(error_length as usize);
        logs.set_len(error_length as usize);

        gl::GetProgramInfoLog(program, error_length, &mut error_length, logs.as_mut_ptr());
        
        panic!("\n\n{}\n\n", CStr::from_ptr(logs.as_ptr()).to_str().unwrap());
    }
}

unsafe fn clear_texture(w: u32, h: u32) {
    let texel_count = (w * h) as usize;
    let mut texture_data: Vec<Texel> = Vec::with_capacity(texel_count);
    for _ in 0..texel_count {
        texture_data.push([255, 255, 255, 255]);
    }

    gl::TexSubImage2D(
        gl::TEXTURE_2D,
        0,      // mip level
        0, 0,   // offset 
        w as _, h as _,
        gl::RGBA, gl::UNSIGNED_BYTE,
        texture_data.as_mut_ptr() as *mut c_void
    );
}

//
// The shader sources
//

const VS_SRC: &'static [u8] = b"#version 330
layout (location=0) in vec2 a_position;
layout (location=1) in vec2 a_uv;

out vec2 uv;

void main() {
    uv = a_uv;
    gl_Position = vec4(a_position, 0.0, 1.0);
}
\0";

const FS_SRC: &'static [u8] = b"#version 330
precision mediump float;

in vec2 uv;

out vec4 outColor;

uniform sampler2D target;
 
void main() {
    outColor = texture(target, uv);
}
\0";

