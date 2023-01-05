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

    use crate::{camera::Camera, mesh::Mesh, uniform::MeshUniform};

    use super::mesh::{DrawParametersComponent, Transform};

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
    use crate::container::Matrix4;

    use ecs_macro::EntityComponent;
    use glium::{texture::Texture3d, DrawParameters, Texture2d};

    #[derive(EntityComponent)]
    pub struct DrawParametersComponent(pub DrawParameters<'static>);

    #[derive(Debug)]
    pub enum TextureType {
        Texture2d(Texture2d),
        Texture3d(Texture3d),
    }

    #[derive(EntityComponent)]
    pub struct Transform {
        pub matrix: Matrix4,
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
