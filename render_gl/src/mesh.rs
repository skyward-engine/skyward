use std::io::Cursor;

use ecs_macro::EntityComponent;
use glium::{
    index::IndicesSource,
    texture::{RawImage2d, Texture3d},
    Display, Program, ProgramCreationError, Texture2d, VertexBuffer,
};
use image::ImageFormat;

use crate::draw::vertex::{ToBuffer, Vertex};

#[derive(Debug)]
pub enum TextureType {
    Texture2d(Texture2d),
    Texture3d(Texture3d),
}

/// A struct representing a 3D mesh.
///
/// A mesh consists of a vertex buffer, an index buffer, a program for rendering the mesh, and an optional texture.
/// The vertex buffer contains the vertex data for the mesh, and the index buffer specifies how the vertices should be connected to form the mesh.
/// The program consists of vertex and fragment shaders, which are responsible for transforming the vertices and applying colors or textures to the surface of the mesh.
/// The optional texture can be applied to the surface of the mesh, adding an additional level of detail.
#[derive(EntityComponent)]
pub struct Mesh {
    /// The vertex buffer for the mesh.
    ///
    /// The vertex buffer stores the vertex data for the mesh. This data includes the position, normal, and texture coordinates for each vertex in the mesh.
    pub vertex_buffer: VertexBuffer<Vertex>,
    /// The index buffer for the mesh.
    ///
    /// The index buffer specifies how the vertices in the vertex buffer should be connected to form the mesh. It is an array of integers that reference the vertices in the vertex buffer.
    pub index_buffer: IndicesSource<'static>,
    /// The program for rendering the mesh.
    ///
    /// The program consists of a vertex shader and a fragment shader. The vertex shader is responsible for transforming the vertices of the mesh, and the fragment shader is responsible for applying colors or textures to the surface of the mesh.
    pub program: Program,
    /// An optional texture for the mesh.
    ///
    /// The texture can be applied to the surface of the mesh, adding an additional level of detail. If no texture is provided, the mesh will be rendered with a solid color.
    pub texture: Option<TextureType>,
}

impl Mesh {
    /// Creates a new `Mesh` instance.
    ///
    /// # Arguments
    ///
    /// * `display` - The display to use for creating the vertex buffer and program.
    /// * `vertexes` - The vertex data for the mesh.
    /// * `index_buffer` - The index buffer for the mesh.
    /// * `vertex_shader` - The vertex shader source code.
    /// * `fragment_shader` - The fragment shader source code.
    ///
    /// # Returns
    ///
    /// A new `Mesh` instance, or a `ProgramCreationError` if there was a problem creating the program.
    pub fn new(
        display: &Display,
        vertices: &[Vertex],
        index_buffer: IndicesSource<'static>,
        vertex_shader: &'static str,
        fragment_shader: &'static str,
    ) -> Result<Self, ProgramCreationError> {
        let buffer = Vertex::to_buffer(display, vertices).unwrap();
        let program = Program::from_source(display, vertex_shader, fragment_shader, None)?;

        let constructed = Self {
            vertex_buffer: buffer,
            index_buffer,
            texture: None,
            program,
        };

        Ok(constructed)
    }

    pub fn buffered(
        display: &Display,
        vertices: VertexBuffer<Vertex>,
        index_buffer: impl Into<IndicesSource<'static>>,
        vertex_shader: &'static str,
        fragment_shader: &'static str,
    ) -> Result<Self, ProgramCreationError> {
        let program = Program::from_source(display, vertex_shader, fragment_shader, None)?;

        let constructed = Self {
            vertex_buffer: vertices,
            index_buffer: index_buffer.into(),
            texture: None,
            program,
        };

        Ok(constructed)
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
}
