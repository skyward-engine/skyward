use std::collections::HashMap;

use ecs::{entity::EntityManager, system::System};
use glium::{Display, Surface, VertexBuffer};

use crate::{camera::Camera, container::Matrix4, mesh::Mesh, uniform::MeshUniform};

use super::{
    instanced::Instanced,
    transform::{DrawParametersComponent, Transform},
    vertex::Vertex,
};

pub struct GlRenderSystem;
pub struct InternalTransformSystem;

impl System<Display> for GlRenderSystem {
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

        let mut vertices = HashMap::<usize, Vec<Instanced>>::new();

        for entity in table.query_single::<Instanced>(manager)? {
            let instanced = manager.query_entity::<Instanced>(*entity).0?;
            let mesh_id = instanced.mesh_entity as usize;

            if !vertices.contains_key(&mesh_id) {
                vertices.insert(mesh_id, vec![]);
            }

            let vec = vertices.get_mut(&mesh_id)?;

            vec.push(*instanced);
        }

        for (mesh_parent, instances) in vertices.iter() {
            let vertex_buffer = VertexBuffer::dynamic(display, &instances).ok()?;
            draw_entity::<Instanced>(*mesh_parent, &view, display, manager, Some(vertex_buffer));
        }

        for entity in table.query_single::<Mesh>(manager)? {
            if vertices.contains_key(entity) {
                continue;
            }

            draw_entity::<Vertex>(*entity, &view, display, manager, None);
        }

        None
    }
}

fn draw_entity<T>(
    entity: usize,
    view: &Matrix4,
    display: &Display,
    manager: &mut EntityManager,
    extra_buffer: Option<VertexBuffer<T>>,
) where
    T: Copy,
{
    let mut target = display.draw();
    let mut empty_mesh = MeshUniform::empty();

    let entries = manager.query_entity_three::<Mesh, MeshUniform, DrawParametersComponent>(entity);

    let (mesh, uniform, draw_parameters) = (
        entries.0.unwrap(),
        entries.1.unwrap_or_else(|| &mut empty_mesh),
        entries.2,
    );

    let uniform = uniform.view_matrix(*view);
    let draw_parameters = match draw_parameters {
        Some(value) => value.0.clone(),
        None => Default::default(),
    };

    target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

    if let Some(extra_buffer) = extra_buffer {
        let vertex_buffer = (&mesh.vertex_buffer, extra_buffer.per_instance().unwrap());

        target
            .draw(
                vertex_buffer,
                mesh.index_buffer.clone(),
                &mesh.program,
                uniform,
                &draw_parameters,
            )
            .unwrap();
    } else {
        let vertex_buffer = &mesh.vertex_buffer;

        target
            .draw(
                vertex_buffer,
                mesh.index_buffer.clone(),
                &mesh.program,
                uniform,
                &draw_parameters,
            )
            .unwrap();
    }

    target.finish().unwrap();
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
