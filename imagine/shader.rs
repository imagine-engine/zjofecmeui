/*******************************************************************************
  shader.rs
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

use std::fs;
use std::str;
use gl::types::*;
use std::path::Path;
use pyo3::prelude::*;
use super::renderer::RENDERER;

#[pyclass]
pub struct Shader {
  pub id: GLuint,
  shader_type: GLenum
}

impl Shader {
  pub fn new(_source: String, shader_type: GLenum) -> Self {
    let path = Path::new(&_source);
    let source: String = if path.exists() {
      fs::read_to_string(path).expect("failed to open shader file")
    } else {
      _source
    };

    Self {
      id: RENDERER.lock().unwrap().add_shader(source, shader_type),
      shader_type: shader_type
    }
  }
}

#[pymethods]
impl Shader {
  #[getter(type)]
  fn get_type(&self) -> PyResult<String> {
    match self.shader_type {
      gl::VERTEX_SHADER => Ok("vertex".to_string()),
      gl::FRAGMENT_SHADER => Ok("fragment".to_string()),
      _ => Ok("N/A".to_string())
    }
  }
}

#[pyfunction]
#[pyo3(name = "VertexShader")]
pub fn vertex_shader(_path: String) -> PyResult<Shader> {
  Ok(Shader::new(_path, gl::VERTEX_SHADER))
}

#[pyfunction]
#[pyo3(name = "FragmentShader")]
pub fn fragment_shader(_path: String) -> PyResult<Shader> {
  Ok(Shader::new(_path, gl::FRAGMENT_SHADER))
}

#[pyfunction]
#[pyo3(name = "ComputeShader")]
pub fn compute_shader(_path: String) -> PyResult<Shader> {
  Ok(Shader::new(_path, gl::COMPUTE_SHADER))
}