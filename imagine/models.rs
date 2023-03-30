/*******************************************************************************
  models.rs
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

use std::io::BufReader;
use super::scene_children::SceneObject;

#[pyfunction]
pub fn load_model(path: &str) -> PyResult<> {
  let reader = BufReader::new(File::open(path)?);
  let object: SceneObject

  for buffer in reader.lines() {
    let line = buffer.unwrap();
    if line.starts_with("#") {
      continue;
    } else if line.starts_with("v") {
    } else if line.starts_with("vn") {
    } else if line.starts_with("f") {
    } else {
    }
  }
}