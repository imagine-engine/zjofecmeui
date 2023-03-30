/*******************************************************************************
  camera.rs
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
use super::frame::Frame;
use ffmpeg_next as ffmpeg;
use super::renderer::RENDERER;
// use super::math::vector::Vector;
use super::main_scene::MAIN_SCENE;

pub struct Camera {
  pub fps: i32,
  pub path: String,
  pub output_width: i32,
  pub output_height: i32,
  pub recording: bool,
  // pub projection: Matrix4
}

unsafe impl Send for Camera {}

impl Camera {
  pub fn new() -> Self {
    ffmpeg::init().unwrap();
    ffmpeg::log::set_level(ffmpeg::log::Level::Warning);

    Self {
      fps: 24,
      path: "video.mp4".to_string(),
      output_width: 1920,
      output_height: 1080,
      recording: false,
      // context,
      // format,
      // stream
    }
  }

  pub fn resolution(&self) -> (i32, i32) {
    return (self.output_width, self.output_height);
  }

  // pub fn encode(&self) {
  // }
}

#[pyclass(name="Camera")]
pub struct PyCamera {
  #[pyo3(get, set)]
  output: Py<VideoOutput>,
  // #[pyo3(get, set)]
  // view: Py<CameraViewport>
}

#[pymethods]
impl PyCamera {
  #[new]
  pub fn new(py: Python<'_>) -> Self {
    Self {
      output: Py::new(py, VideoOutput {}).unwrap()
    }
  }

  #[getter(recording)]
  fn get_recording_status(&self) -> PyResult<bool> {
    Ok(MAIN_SCENE.lock().unwrap().camera.recording)
  }

  fn snapshot(&self) {
    let renderer = RENDERER.lock().unwrap();
    Frame {
      width: renderer.width,
      height: renderer.height,
      pixels: renderer.render()
    }.save("snapshot.png");
  }
}

#[pyclass]
pub struct VideoOutput;

#[pymethods]
impl VideoOutput {
  #[getter(fps)]
  fn get_fps(&self) -> PyResult<i32> {
    Ok(MAIN_SCENE.lock().unwrap().camera.fps)
  }

  #[setter(fps)]
  fn set_fps(&self, fps: i32) {
    MAIN_SCENE.lock().unwrap().camera.fps = fps;
  }

  #[getter(width)]
  fn get_width(&self) -> PyResult<i32> {
    Ok(MAIN_SCENE.lock().unwrap().camera.output_width)
  }

  #[setter(width)]
  fn set_width(&self, width: i32) {
    MAIN_SCENE.lock().unwrap().camera.output_width = width;
  }

  #[getter(height)]
  fn get_height(&self) -> PyResult<i32> {
    Ok(MAIN_SCENE.lock().unwrap().camera.output_height)
  }

  #[setter(height)]
  fn set_height(&self, height: i32) {
    MAIN_SCENE.lock().unwrap().camera.output_height = height;
  }
}

// #[pyclass]
// pub struct CameraViewport;

// #[pymethods]
// impl CameraViewport {
//   #[getter(max)]
//   fn get_max() -> PyResult<Vector> {
//   }
// }