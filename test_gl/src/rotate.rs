use ecs::system::System;
use glium::Display;
use render_gl::{draw::delta::TimeDelta, uniform::MeshUniform};

pub struct WallRotateSystem;

impl System<Display> for WallRotateSystem {
    fn update(
        &mut self,
        manager: &mut ecs::entity::EntityManager,
        table: &mut ecs::entity::EntityQueryTable,
        _: &Display,
    ) -> Option<()> {
        let delta = table.query_single::<TimeDelta>(manager)?.first()?;
        let delta = manager
            .query_entity::<TimeDelta>(*delta)
            .0?
            .get_time_delta_sec();

        let rotate_speed = 0.5 * delta;

        for entity in table.query_single::<MeshUniform>(manager)? {
            let uniform = manager.query_entity::<MeshUniform>(*entity).0?;
            let matrix = uniform.ref_matrix();

            matrix.rotate(rotate_speed, (1.0, 0.0, 0.0));
        }

        None
    }
}
