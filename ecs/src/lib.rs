pub mod component;
pub mod entity;
pub mod system;
pub mod world;

#[cfg(test)]
mod test {
    use crate::{
        component::Component,
        entity::{EntityManager, EntityQueryTable},
        system::System,
        world::{Leak, SystemType, World},
    };

    #[test]
    fn simple_test() {
        #[derive(Debug)]
        struct NameSystem;

        impl System<()> for NameSystem {
            fn update(
                &mut self,
                manager: &mut EntityManager,
                table: &mut EntityQueryTable,
                _: &(),
            ) -> Option<()> {
                // restrain entities to only contain entities which have Named and Position
                let entity_ids = table.query::<(Named, Position)>(manager)?;

                for entity in entity_ids {
                    // get the Position component of the entity
                    let queried = manager.query_entity_two::<Position, Named>(entity);
                    let (position, name) = (queried.0?, queried.1);

                    // mutate the x/y of the entity
                    position.x += 0.1;
                    position.y += 0.3;

                    println!(
                        "{} moved to: {:.1}, {:.1}",
                        name.unwrap().0,
                        position.x,
                        position.y
                    );
                }

                None
            }
        }

        struct Named(&'static str);
        struct Position {
            x: f32,
            y: f32,
        }

        impl Component for Named {}
        impl Component for Position {}

        let mut world = World::new();

        world
            .register::<Named>()
            .register::<Position>()
            .with_system(SystemType::Loop, NameSystem);

        let entity = world.entity();

        world
            .with::<Named>(entity, Named("NV6"))
            .with::<Position>(entity, Position { x: 37.3, y: 37.1 });

        for _ in 0..3 {
            world.update(SystemType::Loop, &());
        }
    }
}
