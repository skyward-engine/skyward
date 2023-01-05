use std::f32::consts::PI;

use ecs_macro::EntityComponent;
use glium::Display;

use crate::container::Matrix4;

#[derive(EntityComponent, Debug, Clone, Copy)]
pub struct Perspective {
    width: f32,
    height: f32,
    fov_div: f32,
    zfar: f32,
    znear: f32,
}

unsafe impl Send for Perspective {}
unsafe impl Sync for Perspective {}

impl Perspective {
    pub fn new(display: &Display, fov_div: f32, zfar: f32, znear: f32) -> Self {
        let entries = display.get_framebuffer_dimensions();
        let (width, height) = (entries.0 as f32, entries.1 as f32);

        Self {
            width,
            height,
            fov_div,
            zfar,
            znear,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        {
            self.width = width;
        }
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        {
            self.height = height;
        }
        self
    }

    pub fn fov_div(mut self, fov_div: f32) -> Self {
        {
            self.fov_div = fov_div;
        }
        self
    }

    pub fn zfar(mut self, zfar: f32) -> Self {
        {
            self.zfar = zfar;
        }
        self
    }

    pub fn znear(mut self, znear: f32) -> Self {
        {
            self.znear = znear;
        }
        self
    }

    pub fn matrix(&self) -> Matrix4 {
        let fov = PI / self.fov_div;
        let f = 1.0 / (fov / 2.0).tan();

        let aspect_ratio = self.height / self.width;

        let zfar = self.zfar;
        let znear = self.znear;

        Matrix4::from([
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ])
    }

    pub fn inner(&self) -> [[f32; 4]; 4] {
        self.matrix().inner()
    }
}
