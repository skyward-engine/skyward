use ecs::system::System;
use glium::Display;
use render_gl::{draw::delta::TimeDelta, uniform::MeshUniform};

const ROTATION_SPEED: f32 = 0.5;

pub struct WallRotateSystem;

impl System<Display> for WallRotateSystem {
    fn update(
        &mut self,
        manager: &mut ecs::entity::EntityManager,
        table: &mut ecs::entity::EntityQueryTable,
        _: &Display,
    ) -> Option<()> {
        let delta_entity = table.query_first_single::<TimeDelta>(manager)?;
        let delta_component = manager.query_entity::<TimeDelta>(*delta_entity).0?;

        let time_delta = delta_component.get_time_delta_sec();
        let rotate_speed = ROTATION_SPEED * time_delta;

        for entity in table.query_single::<MeshUniform>(manager)? {
            let uniform = manager.query_entity::<MeshUniform>(*entity).0?;
            let matrix = uniform.ref_matrix();

            matrix?.rotate(rotate_speed, (1.0, 0.0, 0.0));
        }

        None
    }
}
