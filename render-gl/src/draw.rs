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
}

implement_vertex!(Vertex, position, tex_pos);

#[macro_export]
macro_rules! vertex {
    ([$x:expr, $y:expr, $z:expr], [$xt:expr, $yt:expr]) => {
        Vertex {
            position: [$x, $y, $z],
            tex_pos: [$xt, $yt],
        }
    };
    ([$x:expr, $y:expr], [$xt:expr, $yt:expr]) => {
        Vertex {
            position: [$x, $y, 1.0],
            tex_pos: [$xt, $yt],
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
    use glium::{uniform, Display, Surface};

    use super::mesh::{Mesh, Transform};

    pub struct InternalSystem;

    impl System<Display> for InternalSystem {
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
                    [0.0, 1.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ];

                if let Some(transform) = transform {
                    matrix = transform.inner();
                }

                target.clear_color(1.0, 1.0, 1.0, 1.0);

                match &mesh.texture {
                    Some(texture) => {
                        let uniform = uniform! {
                            matrix: matrix,
                            tex: texture,
                        };
                        target
                            .draw(
                                &mesh.vertex_buffer,
                                mesh.index_buffer.clone(),
                                &mesh.program,
                                &uniform,
                                &Default::default(),
                            )
                            .unwrap();
                    }
                    None => {
                        let uniform = uniform! {
                            matrix: matrix,
                        };
                        target
                            .draw(
                                &mesh.vertex_buffer,
                                mesh.index_buffer.clone(),
                                &mesh.program,
                                &uniform,
                                &Default::default(),
                            )
                            .unwrap();
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
        index::IndicesSource, texture::RawImage2d, Display, Program, ProgramCreationError,
        Texture2d, VertexBuffer,
    };
    use image::ImageFormat;

    #[derive(EntityComponent)]
    pub struct Mesh {
        pub vertex_buffer: VertexBuffer<Vertex>,
        pub index_buffer: IndicesSource<'static>,
        pub program: Program,
        pub texture: Option<Texture2d>,
    }

    impl Mesh {
        pub fn new(
            display: &Display,
            vertexes: &[Vertex],
            index_buffer: IndicesSource<'static>,
            vertex_shader: &'static str,
            fragment_shader: &'static str,
        ) -> Result<Self, ProgramCreationError> {
            let buffer = Vertex::to_buffer(display, vertexes).unwrap();
            let program = Program::from_source(display, vertex_shader, fragment_shader, None)?;

            let constructed = Self {
                vertex_buffer: buffer,
                index_buffer,
                texture: None,
                program,
            };

            Ok(constructed)
        }

        pub fn with_img_texture(
            mut self,
            format: ImageFormat,
            display: &Display,
            bytes: &[u8],
        ) -> Self {
            let image = image::load(Cursor::new(bytes), format).unwrap().to_rgba8();
            let dimensions = image.dimensions();
            let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
            let texture = Texture2d::new(display, image).unwrap();

            self.texture = Some(texture);

            self
        }
    }

    #[derive(EntityComponent)]
    pub struct Transform {
        matrix: Matrix4,
    }

    impl Transform {
        pub fn new() -> Self {
            Self {
                matrix: Matrix4::from([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 1.0],
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
            // Create a scaling matrix and multiply it with the current transformation matrix
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
