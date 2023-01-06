use ecs_macro::EntityComponent;
use glium::{
    index::IndicesSource, texture::Texture3d, Display, Program, ProgramCreationError, Texture2d,
    VertexBuffer,
};

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
            program,
        };

        Ok(constructed)
    }
}
