use std::io::Cursor;

use ecs_macro::EntityComponent;
use glium::{
    texture::RawImage2d,
    uniforms::{UniformValue, Uniforms},
    Display, Texture2d,
};
use image::ImageFormat;

use crate::{
    container::{Matrix4, Vec3},
    draw::transform::Transform,
    mesh::TextureType,
};

use self::perspective::Perspective;

pub mod perspective;

#[derive(EntityComponent, Debug)]
pub struct MeshUniform {
    matrix: Option<Matrix4>,
    view_matrix: Option<Matrix4>,
    light: Option<Vec3>,
    perspective: Option<Perspective>,
    texture: Option<TextureType>,
    diffuse_texture: Option<TextureType>,
    normal_texture: Option<TextureType>,
    world_location: Option<Vec3>,
}

impl MeshUniform {
    pub fn new(matrix: Matrix4) -> Self {
        Self {
            matrix: Some(matrix),
            view_matrix: None,
            light: None,
            texture: None,
            perspective: None,
            diffuse_texture: None,
            normal_texture: None,
            world_location: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            matrix: None,
            view_matrix: None,
            light: None,
            texture: None,
            perspective: None,
            diffuse_texture: None,
            normal_texture: None,
            world_location: None,
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

    pub fn normal_texture(mut self, texture: TextureType) -> Self {
        self.normal_texture = Some(texture);
        self
    }

    pub fn diffuse_texture(mut self, texture: TextureType) -> Self {
        self.diffuse_texture = Some(texture);
        self
    }

    pub fn perspective(mut self, perspective: Perspective) -> Self {
        self.perspective = Some(perspective.clone());
        self
    }

    pub fn location(mut self, location: impl Into<Vec3>) -> Self {
        self.world_location = Some(location.into());
        self
    }

    pub fn view_matrix(&mut self, matrix: impl Into<Matrix4>) -> &mut Self {
        self.view_matrix = Some(matrix.into());
        self
    }

    pub fn matrix(&mut self, matrix: impl Into<Matrix4>) {
        self.matrix = Some(matrix.into());
    }

    pub fn transform(&mut self, transform: &Transform) {
        self.matrix = Some(transform.matrix.clone());
    }

    pub fn ref_matrix(&mut self) -> Option<&mut Matrix4> {
        self.matrix.as_mut()
    }

    /// Creates a new `Mesh` instance with an image texture.
    ///
    /// # Arguments
    ///
    /// * `format` - The image format of the texture.
    /// * `display` - The display to use for creating the texture.
    /// * `bytes` - The bytes of the image data.
    ///
    /// # Returns
    ///
    /// A new `Mesh` instance with an image texture.
    ///
    /// # Examples
    ///
    /// ```
    /// use skyward::render::draw::mesh::{Mesh, Vertex};
    /// use glium::{Display, ImageFormat};
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// let mesh = Mesh::new(display, &[], &[], "", "")
    ///     .unwrap()
    ///     .with_img_2d_texture(ImageFormat::Png, display, include_bytes!("picture.png"));
    /// ```
    pub fn with_img_2d_texture(
        mut self,
        format: ImageFormat,
        display: &Display,
        bytes: &[u8],
    ) -> Self {
        let image = image::load(Cursor::new(bytes), format).unwrap().to_rgba8();
        let dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
        let texture = Texture2d::new(display, image).unwrap();
        self.texture = Some(TextureType::Texture2d(texture));
        self
    }

    /// Creates a new `Mesh` instance with an image texture.
    ///
    /// # Arguments
    ///
    /// * `format` - The image format of the texture.
    /// * `display` - The display to use for creating the texture.
    /// * `bytes` - The bytes of the image data.
    ///
    /// # Returns
    ///
    /// A new `Mesh` instance with an image texture.
    ///
    /// # Examples
    ///
    /// ```
    /// use skyward::render::draw::mesh::{Mesh, Vertex};
    /// use glium::{Display, ImageFormat};
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// let mesh = Mesh::new(display, &[], &[], "", "")
    ///     .unwrap()
    ///     .with_img_2d_texture(ImageFormat::Png, display, include_bytes!("picture.png"));
    /// ```
    pub fn with_img_2d_diff_texture(
        mut self,
        format: ImageFormat,
        display: &Display,
        bytes: &[u8],
    ) -> Self {
        let image = image::load(Cursor::new(bytes), format).unwrap().to_rgba8();
        let dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
        let texture = Texture2d::new(display, image).unwrap();
        self.diffuse_texture = Some(TextureType::Texture2d(texture));
        self
    }

    /// Creates a new `Mesh` instance with an image texture.
    ///
    /// # Arguments
    ///
    /// * `format` - The image format of the texture.
    /// * `display` - The display to use for creating the texture.
    /// * `bytes` - The bytes of the image data.
    ///
    /// # Returns
    ///
    /// A new `Mesh` instance with an image texture.
    ///
    /// # Examples
    ///
    /// ```
    /// use skyward::render::draw::mesh::{Mesh, Vertex};
    /// use glium::{Display, ImageFormat};
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// let mesh = Mesh::new(display, &[], &[], "", "")
    ///     .unwrap()
    ///     .with_img_2d_texture(ImageFormat::Png, display, include_bytes!("picture.png"));
    /// ```
    pub fn with_img_2d_norm_texture(
        mut self,
        format: ImageFormat,
        display: &Display,
        bytes: &[u8],
    ) -> Self {
        let image = image::load(Cursor::new(bytes), format).unwrap().to_rgba8();
        let dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
        let texture = Texture2d::new(display, image).unwrap();
        self.normal_texture = Some(TextureType::Texture2d(texture));
        self
    }
}

impl Uniforms for MeshUniform {
    fn visit_values<'b, F: FnMut(&str, glium::uniforms::UniformValue<'b>)>(&'b self, mut f: F) {
        if let Some(matrix) = self.matrix {
            f("matrix", UniformValue::Mat4(matrix.inner()));
        }

        if let Some(light) = self.light {
            f("u_light", UniformValue::Vec3(light.inner()));
        }

        if let Some(perspective) = self.perspective {
            f("perspective", UniformValue::Mat4(perspective.inner()));
        }

        if let Some(view_matrix) = self.view_matrix {
            f("view", UniformValue::Mat4(view_matrix.inner()));
        }

        if let Some(location) = self.world_location {
            f("world_location", UniformValue::Vec3(location.inner()));
        }

        for entry in [
            (&self.texture, "tex"),
            (&self.diffuse_texture, "diffuse_tex"),
            (&self.normal_texture, "norm_tex"),
        ] {
            let texture = entry.0;
            let id = entry.1;

            if let Some(texture) = texture {
                match texture {
                    TextureType::Texture2d(texture) => {
                        f(id, UniformValue::Texture2d(&texture, None))
                    }
                    TextureType::Texture3d(texture) => {
                        f(id, UniformValue::Texture3d(&texture, None))
                    }
                };
            }
        }
    }
}
