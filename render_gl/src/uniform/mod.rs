use ecs_macro::EntityComponent;
use glium::uniforms::{UniformValue, Uniforms};

use crate::{
    container::{Matrix4, Vec3},
    draw::transform::Transform,
    mesh::TextureType,
};

use self::perspective::Perspective;

pub mod perspective;

#[derive(EntityComponent, Debug)]
pub struct MeshUniform {
    matrix: Matrix4,
    view_matrix: Option<Matrix4>,
    light: Option<Vec3>,
    texture: Option<TextureType>,
    perspective: Option<Perspective>,
}

impl MeshUniform {
    pub fn new(matrix: Matrix4) -> Self {
        Self {
            matrix,
            view_matrix: None,
            light: None,
            texture: None,
            perspective: None,
        }
    }

    pub fn light(mut self, light: impl Into<Vec3>) -> Self {
        self.light = Some(light.into());
        self
    }

    pub fn texture(mut self, texture: TextureType) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn perspective(mut self, perspective: Perspective) -> Self {
        self.perspective = Some(perspective.clone());
        self
    }

    pub fn view_matrix(&mut self, matrix: Matrix4) -> &mut Self {
        self.view_matrix = Some(matrix);
        self
    }

    pub fn matrix(&mut self, matrix: Matrix4) {
        self.matrix = matrix;
    }

    pub fn transform(&mut self, transform: &Transform) {
        self.matrix = transform.matrix.clone();
    }

    pub fn ref_matrix(&mut self) -> &mut Matrix4 {
        &mut self.matrix
    }
}

impl Uniforms for MeshUniform {
    fn visit_values<'b, F: FnMut(&str, glium::uniforms::UniformValue<'b>)>(&'b self, mut f: F) {
        f("matrix", UniformValue::Mat4(self.matrix.inner()));

        if let Some(light) = self.light {
            f("u_light", UniformValue::Vec3(light.inner()));
        }

        if let Some(perspective) = self.perspective {
            f("perspective", UniformValue::Mat4(perspective.inner()));
        }

        if let Some(view_matrix) = self.view_matrix {
            f("view", UniformValue::Mat4(view_matrix.inner()));
        }

        if let Some(texture) = &self.texture {
            match texture {
                TextureType::Texture2d(texture) => {
                    f("tex", UniformValue::Texture2d(&texture, None))
                }
                TextureType::Texture3d(texture) => {
                    f("tex", UniformValue::Texture3d(&texture, None))
                }
            };
        }
    }
}
