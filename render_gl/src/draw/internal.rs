use ecs::system::System;
use glium::{uniforms::EmptyUniforms, Display, Surface};

use crate::{camera::Camera, mesh::Mesh, uniform::MeshUniform};

use super::transform::{DrawParametersComponent, Transform};

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

            let entries =
                manager.query_entity_three::<Mesh, MeshUniform, DrawParametersComponent>(*entity);
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
