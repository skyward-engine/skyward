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

#[derive(Copy, Clone, Debug)]
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
            let normal = normals[i];

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
    use glium::{uniforms::EmptyUniforms, Display, Surface};

    use crate::camera::Camera;

    use super::mesh::{DrawParametersComponent, Mesh, MeshUniform, Transform};

    pub struct InternalSystem;
    pub struct InternalTransformSystem;

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
            let view = {
                let entity = table
                    .query_single::<Camera>(manager)
                    .expect("No camera is initialized!")
                    .first()
                    .expect("No camera is initialized!");

                let entity = *entity;

                let view = manager.query_entity::<Camera>(entity).0;
                let view = view.expect("No camera is initialized!");

                view.view_matrix()
            };

            for entity in table.query_single::<Mesh>(manager)? {
                let mut target = display.draw();

                let entries = manager
                    .query_entity_three::<Mesh, MeshUniform, DrawParametersComponent>(*entity);
                let (mesh, uniform, draw_parameters) = (entries.0?, entries.1, entries.2);

                let draw_parameters = match draw_parameters {
                    Some(value) => value.0.clone(),
                    None => Default::default(),
                };

                target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                match uniform {
                    Some(uniform) => {
                        let uniform = uniform.view_matrix(view);

                        target
                            .draw(
                                &mesh.vertex_buffer,
                                mesh.index_buffer.clone(),
                                &mesh.program,
                                uniform,
                                &draw_parameters,
                            )
                            .unwrap();
                    }
                    None => {
                        target
                            .draw(
                                &mesh.vertex_buffer,
                                mesh.index_buffer.clone(),
                                &mesh.program,
                                &EmptyUniforms,
                                &draw_parameters,
                            )
                            .unwrap();
                    }
                }

                target.finish().unwrap();
            }

            None
        }
    }

    impl System<Display> for InternalTransformSystem {
        fn update(
            &mut self,
            manager: &mut ecs::entity::EntityManager,
            table: &mut ecs::entity::EntityQueryTable,
            _: &Display,
        ) -> Option<()> {
            for entity in table.query::<(Transform, MeshUniform)>(manager)? {
                let entry = manager.query_entity_two::<Transform, MeshUniform>(entity);
                let (transform, mesh) = (entry.0?, entry.1?);

                mesh.transform(transform);
            }

            None
        }
    }
}

pub mod mesh {
    use std::io::Cursor;

    use crate::container::{Matrix4, Vec3};

    use super::{perspective::Perspective, ToBuffer, Vertex};
    use ecs_macro::EntityComponent;
    use glium::{
        index::{IndicesSource, PrimitiveType},
        texture::{RawImage2d, Texture3d},
        uniforms::{UniformValue, Uniforms},
        Display, DrawParameters, IndexBuffer, Program, ProgramCreationError, Texture2d,
        VertexBuffer,
    };
    use image::ImageFormat;

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

    #[derive(EntityComponent)]
    pub struct DrawParametersComponent(pub DrawParameters<'static>);

    #[derive(Debug)]
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

        pub fn ref_matrix(&mut self) -> &mut Matrix4 {
            &mut self.matrix
        }

        pub fn inner(&self) -> [[f32; 4]; 4] {
            self.matrix.inner()
        }
    }
}

pub mod perspective {
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
}
