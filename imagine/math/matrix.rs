/*******************************************************************************
  matrix.rs
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

use pyo3::prelude::*;
use nalgebra::base::Matrix4;

#[pyclass]
pub struct Matrix {
  #[pyo3(get, set)]
  pub dim: u8,
  pub elements: Vec<Vec<f32>>
}

// impl Matrix {
//   pub fn to_matrix4(&self) -> Matrix4 {
//   }
// }

#[pymethods]
impl Matrix {
  #[new]
  pub fn new(elements: Vec<Vec<f32>>) -> Self {
    Self {
      dim: elements.len(),
      elements
    }
  }
}