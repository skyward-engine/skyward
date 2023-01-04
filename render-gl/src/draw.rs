use core::panic;
use glium::{implement_vertex, vertex::BufferCreationError, Display, VertexBuffer};
use std::{collections::HashMap, hash::Hash};

pub trait ToBuffer: Sized + Copy {
    fn to_buffer(
        display: &Display,
        shape: &[Self],
    ) -> Result<VertexBuffer<Self>, BufferCreationError>;
}

pub struct VertexBufferCache<K, V>
where
    K: PartialEq + Eq + Hash,
    V: Copy + ToBuffer,
{
    buffer_map: HashMap<K, VertexBuffer<V>>,
}

impl<K, V> VertexBufferCache<K, V>
where
    K: PartialEq + Eq + Hash,
    V: Copy + ToBuffer,
{
    pub fn new() -> Self {
        Self {
            buffer_map: HashMap::new(),
        }
    }

    pub fn insert_buffer(&mut self, key: K, buffer: VertexBuffer<V>) {
        self.buffer_map.insert(key, buffer);
    }

    pub fn get_vertex_buffer(&self, key: K) -> Option<&VertexBuffer<V>> {
        self.buffer_map.get(&key)
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_pos: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn from_vertices(
        display: &Display,
        vertices: &[(f32, f32, f32)],
        normals: &[(f32, f32, f32)],
    ) -> VertexBuffer<Vertex> {
        if vertices.len() != normals.len() {
            // todo: proper error handling
            panic!("Vertices and normals should be the same length!");
        };

        let mut vertex_vec = Vec::<Vertex>::new();

        for i in 0..vertices.len() {
            let position = vertices[i];
            let normal = normals[1];

            let vertex = Vertex {
                position: [position.0, position.1, position.2],
                normal: [normal.0, normal.1, normal.2],
                tex_pos: [0.0, 0.0],
            };

            vertex_vec.push(vertex);
        }

        Self::to_buffer(display, &vertex_vec).unwrap()
    }
}

implement_vertex!(Vertex, position, tex_pos, normal);

#[macro_export]
macro_rules! vertex {
    ([$x:expr, $y:expr, $z:expr], [$xt:expr, $yt:expr]) => {
        Vertex {
            position: [$x, $y, $z],
            tex_pos: [$xt, $yt],
            normal: [0.0, 0.0, 0.0],
        }
    };
    ([$x:expr, $y:expr], [$xt:expr, $yt:expr]) => {
        Vertex {
            position: [$x, $y, 1.0],
            tex_pos: [$xt, $yt],
            normal: [0.0, 0.0, 0.0],
        }
    };
    ([$x:expr, $y:expr, $z:expr], [$xt:expr, $yt:expr], [$nx:expr, $ny:expr, $nz:expr]) => {
        Vertex {
            position: [$x, $y, $z],
            tex_pos: [$xt, $yt],
            normal: [$nx, $ny, $nz],
        }
    };
    ([$x:expr, $y:expr], [$xt:expr, $yt:expr], [$nx:expr, $ny:expr, $nz:expr]) => {
        Vertex {
            position: [$x, $y, 1.0],
            tex_pos: [$xt, $yt],
            normal: [$nx, $ny, $nz],
        }
    };
}

impl ToBuffer for Vertex {
    fn to_buffer(
        display: &glium::Display,
        shape: &[Self],
    ) -> Result<VertexBuffer<Self>, BufferCreationError> {
        VertexBuffer::new(display, shape)
    }
}

pub mod internal {
    use ecs::system::System;
    use glium::{uniform, uniforms::Uniforms, Display, Frame, Surface};

    use super::mesh::{Mesh, TextureType, Transform};

    pub struct InternalSystem;

    impl System<Display> for InternalSystem {
        /// Renders the mesh components of all entities that have both a `Mesh` and a `Transform` component.
        /// If an entity has a `Mesh` component but no `Transform` component, the default identity matrix is used.
        ///
        /// # Parameters
        ///
        /// - `&mut self`: This system instance.
        /// - `manager`: The `EntityManager` that manages the entities in the world.
        /// - `table`: The `EntityQueryTable` used to query the entities in the world.
        /// - `display`: A `Display` object that is used to draw to the screen.
        ///
        /// # Returns
        ///
        /// If any of the component queries fail, an `Option` containing `None` is returned. Otherwise, an `Option` containing `Some(())` is returned.
        fn update(
            &mut self,
            manager: &mut ecs::entity::EntityManager,
            table: &mut ecs::entity::EntityQueryTable,
            display: &Display,
        ) -> Option<()> {
            for entity in table.query_single::<Mesh>(manager)? {
                let mut target = display.draw();

                let entries = manager.query_entity_two::<Mesh, Transform>(*entity);
                let (mesh, transform) = (entries.0?, entries.1);

                let mut matrix: [[f32; 4]; 4] = [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ];

                if let Some(transform) = transform {
                    matrix = transform.inner();
                }

                target.clear_color(0.0, 1.0, 0.0, 1.0);

                fn draw<T>(target: &mut Frame, mesh: &Mesh, uniform: &T)
                where
                    T: Uniforms,
                {
                    target
                        .draw(
                            &mesh.vertex_buffer,
                            mesh.index_buffer.clone(),
                            &mesh.program,
                            uniform,
                            &Default::default(),
                        )
                        .unwrap();
                }

                match &mesh.texture {
                    Some(texture) => {
                        match texture {
                            TextureType::Texture2d(texture) => {
                                let uniform = uniform! {
                                    matrix: matrix,
                                    tex: texture,
                                };

                                draw(&mut target, &mesh, &uniform);
                            }
                            TextureType::Texture3d(texture) => {
                                let uniform = uniform! {
                                    matrix: matrix,
                                    tex: texture,
                                };

                                draw(&mut target, &mesh, &uniform);
                            }
                        };
                    }
                    None => {
                        let uniform = uniform! {
                            matrix: matrix,
                        };
                        draw(&mut target, &mesh, &uniform);
                    }
                }

                target.finish().unwrap();
            }

            None
        }
    }
}

pub mod mesh {
    use std::io::Cursor;

    use crate::container::Matrix4;

    use super::{ToBuffer, Vertex};
    use ecs_macro::EntityComponent;
    use glium::{
        index::{IndicesSource, PrimitiveType},
        texture::{RawImage2d, Texture3d},
        Display, IndexBuffer, Program, ProgramCreationError, Texture2d, VertexBuffer,
    };
    use image::ImageFormat;

    pub enum TextureType {
        Texture2d(Texture2d),
        Texture3d(Texture3d),
    }

    pub struct IndexBufferCreator {
        index_buffers_u32: Vec<IndexBuffer<u32>>,
        index_buffers_u16: Vec<IndexBuffer<u16>>,
        index_buffers_u8: Vec<IndexBuffer<u8>>,
    }

    impl IndexBufferCreator {
        pub fn new() -> Self {
            Self {
                index_buffers_u32: vec![],
                index_buffers_u16: vec![],
                index_buffers_u8: vec![],
            }
        }

        pub fn create_index_buffer_u32(
            &mut self,
            display: &Display,
            vertices: &[u32],
            primitive_type: PrimitiveType,
        ) -> &mut Self {
            {
                let index_buffer = IndexBuffer::new(display, primitive_type, vertices).unwrap();
                self.index_buffers_u32.push(index_buffer);
            }

            self
        }

        pub fn get_index_buffer_u32<'a>(&self) -> &IndexBuffer<u32> {
            self.index_buffers_u32.last().unwrap()
        }

        pub fn create_index_buffer_u16(
            &mut self,
            display: &Display,
            vertices: &[u16],
            primitive_type: PrimitiveType,
        ) -> &mut Self {
            {
                let index_buffer = IndexBuffer::new(display, primitive_type, vertices).unwrap();
                self.index_buffers_u16.push(index_buffer);
            }

            self
        }

        pub fn get_index_buffer_u16<'a>(&self) -> &IndexBuffer<u16> {
            self.index_buffers_u16.last().unwrap()
        }

        pub fn create_index_buffer_u8(
            &mut self,
            display: &Display,
            vertices: &[u8],
            primitive_type: PrimitiveType,
        ) -> &mut Self {
            {
                let index_buffer = IndexBuffer::new(display, primitive_type, vertices).unwrap();
                self.index_buffers_u8.push(index_buffer);
            }

            self
        }

        pub fn get_index_buffer_u8<'a>(&self) -> &IndexBuffer<u8> {
            self.index_buffers_u8.last().unwrap()
        }
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
        /// use fox::render::draw::mesh::{Mesh, Vertex};
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

    #[derive(EntityComponent)]
    pub struct Transform {
        matrix: Matrix4,
    }

    impl From<[[f32; 4]; 4]> for Transform {
        fn from(value: [[f32; 4]; 4]) -> Self {
            Self {
                matrix: Matrix4::from(value),
            }
        }
    }

    impl Transform {
        pub fn new() -> Self {
            Self {
                matrix: Matrix4::from([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]),
            }
        }

        pub fn translate(&mut self, x: f32, y: f32, z: f32) {
            let translate_matrix = Matrix4::from([
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            self.matrix = multiply(translate_matrix, self.matrix);
        }

        pub fn rotate(&mut self, angle: f32, axis: (f32, f32, f32)) {
            let (x, y, z) = axis;
            let c = angle.cos();
            let s = angle.sin();
            let rotate_matrix = Matrix4::from([
                [
                    x * x * (1.0 - c) + c,
                    y * x * (1.0 - c) + z * s,
                    z * x * (1.0 - c) - y * s,
                    0.0,
                ],
                [
                    x * y * (1.0 - c) - z * s,
                    y * y * (1.0 - c) + c,
                    z * y * (1.0 - c) + x * s,
                    0.0,
                ],
                [
                    x * z * (1.0 - c) + y * s,
                    y * z * (1.0 - c) - x * s,
                    z * z * (1.0 - c) + c,
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            self.matrix = multiply(rotate_matrix, self.matrix);
        }

        pub fn scale(&mut self, x: f32, y: f32, z: f32) {
            let scale_matrix = Matrix4::from([
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            self.matrix = multiply(scale_matrix, self.matrix);
        }

        pub fn inner(&self) -> [[f32; 4]; 4] {
            let first = self.matrix[0];
            let second = self.matrix[1];
            let third = self.matrix[2];
            let fourth = self.matrix[3];

            [
                [first[0], first[1], first[2], first[3]],
                [second[0], second[1], second[2], second[3]],
                [third[0], third[1], third[2], third[3]],
                [fourth[0], fourth[1], fourth[2], fourth[3]],
            ]
        }
    }

    pub fn multiply(a: Matrix4, b: Matrix4) -> Matrix4 {
        let mut result = Matrix4::new();
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] =
                    a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
            }
        }
        result
    }
}
