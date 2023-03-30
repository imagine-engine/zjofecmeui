/*******************************************************************************
  renderer.rs
********************************************************************************
  Copyright 2023 Menelik Eyasu

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*******************************************************************************/

use std::ptr;
use gl::types::*;
use pyo3::prelude::*;
use std::sync::Mutex;
use std::ffi::CString;
use super::frame::Frame;
use euclid::default::Size2D;
use lazy_static::lazy_static;
use super::main_scene::MAIN_SCENE;
use surfman::{
  Context,
  Connection,
  ContextAttributeFlags,
  ContextAttributes,
  GLVersion,
  SurfaceAccess,
  SurfaceType
};
use surfman::platform::macos::cgl::device::Device;//
// use surfman::platform::windows::angle::device::Device;

lazy_static! {
  pub static ref RENDERER: Mutex<Renderer> = Mutex::new(Renderer::new());
}

pub struct Renderer {
  pub width: i32,
  pub height: i32,
  pub buffer_size: usize,
  device: Device,
  context: Context,
  pub shaders: Vec<GLuint>,
  pipeline: GLuint,
  vao: GLuint,
  vbo: GLuint
}

unsafe impl Send for Renderer {}

impl Drop for Renderer {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.pipeline);
      for shader in self.shaders.iter() {
        gl::DeleteShader(*shader);
      }
    }
    self.device.destroy_context(&mut self.context).unwrap();
  }
}

impl Renderer {
  pub fn new() -> Self {
    let connection = Connection::new().unwrap();
    let adapter = connection.create_adapter().unwrap();
    let mut device = connection.create_device(&adapter).unwrap();

    let attrs = ContextAttributes {
      version: GLVersion::new(3, 3),
      flags: ContextAttributeFlags::empty()
    };
    let context_desc = device.create_context_descriptor(&attrs).unwrap();
    let mut context = device.create_context(&context_desc, None).unwrap();

    let (width, height) = MAIN_SCENE.lock().unwrap().camera.resolution();
    let surface = device.create_surface(
      &context,
      SurfaceAccess::GPUOnly,
      SurfaceType::Generic {
        size: Size2D::new(width, height)
      }
    ).unwrap();
    device.bind_surface_to_context(&mut context, surface).unwrap();
    device.make_context_current(&context).unwrap();

    gl::load_with(|s| device.get_proc_address(&context, s));

    let mut shaders = vec![];
    unsafe {
      gl::Viewport(0, 0, 5, 5);

      let vertex_shader = Self::create_shader(
        String::from_utf8(
          include_bytes!("res/shaders/default.vert").to_vec()
        ).unwrap(),
        gl::VERTEX_SHADER
      );
      let fragment_shader = Self::create_shader(
        String::from_utf8(
          include_bytes!("res/shaders/default.frag").to_vec()
        ).unwrap(),
        gl::FRAGMENT_SHADER
      );
      shaders.push(vertex_shader);
      shaders.push(fragment_shader);

      let mut success = 0;
      let pipeline = gl::CreateProgram();
      gl::AttachShader(pipeline, vertex_shader);
      gl::AttachShader(pipeline, fragment_shader);
      gl::LinkProgram(pipeline);
      gl::GetProgramiv(pipeline, gl::LINK_STATUS, &mut success);
      if success != gl::TRUE as GLint {
        panic!("failed to establish rendering pipeline");
      }

      let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
      ];
      let mut vbo = 0;
      gl::GenBuffers(1, &mut vbo);
      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
        vertices.as_ptr() as *const GLvoid,
        gl::STATIC_DRAW
      );

      let mut vao = 0;
      gl::GenVertexArrays(1, &mut vao);
      gl::BindVertexArray(vao);
      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      let position_attr = gl::GetAttribLocation(
        pipeline,
        b"aPosition\0".as_ptr() as *const GLchar
      ) as GLuint;
      let color_attr = gl::GetAttribLocation(
        pipeline,
        "aColor\0".as_ptr() as *const GLchar
      ) as GLuint;
      gl::VertexAttribPointer(
          position_attr,
          2,
          gl::FLOAT,
          gl::FALSE,
          12,
          0 as *const GLvoid,
      );
      gl::VertexAttribPointer(
          color_attr,
          4,
          gl::UNSIGNED_BYTE,
          gl::TRUE,
          12,
          8 as *const GLvoid,
      );
      gl::EnableVertexAttribArray(position_attr);
      gl::EnableVertexAttribArray(color_attr);
      gl::BindVertexArray(vao);

      let surface_info = device.context_surface_info(&context).unwrap().unwrap();
      gl::BindFramebuffer(gl::FRAMEBUFFER, surface_info.framebuffer_object);

      Self {
        width,
        height,
        buffer_size: (4 * width * height) as usize,
        device,
        context,
        shaders,
        pipeline,
        vao,
        vbo
      }
    }
  }

  // pub fn resize(&mut self, width: i32, height: i32) {
  //   self.width = width;
  //   self.height = height;
  //   self.buffer_size = (4 * width * height) as usize;

  //   self.device.destroy_context(&mut self.context).unwrap();
  //   let new_surface = device.create_surface(
  //     &context,
  //     SurfaceAccess::GPUOnly,
  //     SurfaceType::Generic {
  //       size: Size2D::new(width, height)
  //     }
  //   ).unwrap();
  //   device.bind_surface_to_context(&mut context, surface).unwrap();
  //   device.make_context_current(&context).unwrap();
  // }

  pub fn add_shader(&mut self, source: String, shader_type: GLenum) -> GLuint {
    let shader = Self::create_shader(source, shader_type);
    self.shaders.push(shader);
    shader
  }

  fn create_shader(source: String, shader_type: GLenum) -> GLuint {
    unsafe {
      let shader = gl::CreateShader(shader_type);
      let code = CString::new(source).unwrap();
      gl::ShaderSource(shader, 1, &code.as_ptr(), ptr::null());
      gl::CompileShader(shader);

      let mut success = 0;
      gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
      if success != gl::TRUE as GLint {
        panic!("failed to load shader");
      }

      shader
    }
  }

  pub fn render(&self) -> Vec<u8> {
    unsafe {
      gl::ClearColor(0.0, 0.0, 0.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);
      let mut pixels: Vec<u8> = Vec::new();

      // let vertices = MAIN_SCENE.lock().unwrap().vertex_buffer();

      for _ in 0..self.width*self.height {
        pixels.push(0);
        pixels.push(0);
        pixels.push(0);
        pixels.push(255);
      }

      // gl::UseProgram(self.pipeline);
      // gl::DrawArrays(gl::TRIANGLES, 0, 3);
      // gl::ReadPixels(
      //   0,
      //   0,
      //   self.width as GLsizei,
      //   self.height as GLsizei,
      //   gl::RGBA,
      //   gl::UNSIGNED_BYTE,
      //   pixels.as_mut_ptr() as *mut GLvoid
      // );
      // gl::Flush();

      pixels
    }
  }
}

// #[pyclass(name="Renderer")]
// pub struct PyRenderer;

// #[pymethods]
// impl PyRenderer {
//   // fn __clear__(&self) {
//   //   device.destroy_context(&mut self.context).unwrap();
//   // }
// }

#[pyfunction]
pub fn render() -> PyResult<Frame> {
  let renderer = RENDERER.lock().unwrap();

  Ok(Frame {
    width: renderer.width,
    height: renderer.height,
    pixels: renderer.render()
  })
}