#[macro_use]
pub mod draw;
pub mod container;
pub mod window;

#[cfg(test)]
mod test {
    use std::time::{Duration, Instant};

    use ecs::{system::System, world::World};
    use glium::{
        glutin::{
            event::{Event, WindowEvent},
            event_loop::{ControlFlow, EventLoopWindowTarget},
        },
        index::{NoIndices, PrimitiveType},
        Display,
    };

    use crate::{
        draw::{
            internal::InternalSystem,
            mesh::{Mesh, Transform},
            Vertex,
        },
        window::{PlatformHandle, Window},
    };

    #[test]
    fn test_platform() {
        struct MetaPlatform {
            display: Option<Display>,
            world: World<Display>,
        }

        impl MetaPlatform {
            pub fn new() -> Self {
                Self {
                    display: None,
                    world: World::new(),
                }
            }
        }

        struct TransformSystem;

        impl System<Display> for TransformSystem {
            fn update(
                &mut self,
                manager: &mut ecs::entity::EntityManager,
                table: &mut ecs::entity::EntityQueryTable,
                _: &Display,
            ) -> Option<()> {
                for entity in table.query_single::<Transform>(manager)? {
                    let transform = manager.query_entity::<Transform>(*entity).0?;
                    transform.rotate(45.0, (1.0, 0.0, 0.0));
                }

                None
            }
        }

        impl PlatformHandle for MetaPlatform {
            fn initialize_display(&mut self, display: Display) {
                self.display = Some(display);
            }

            fn initialize_cache(&mut self) {
                let entity = self.world.entity();

                self.world
                    .with::<Transform>(entity, Transform::new())
                    .with::<Mesh>(
                        entity,
                        Mesh::new(
                            self.display.as_ref().unwrap(),
                            &[
                                vertex!([-0.5, -0.5], [0.0, 0.0]),
                                vertex!([0.5, -0.5], [0.0, 1.0]),
                                vertex!([-0.5, 0.5], [1.0, 0.0]),
                                vertex!([0.5, 0.5], [1.0, 1.0]),
                            ],
                            NoIndices(PrimitiveType::TriangleStrip).into(),
                            include_str!("../../shaders/test_vertex_shader.vert"),
                            include_str!("../../shaders/test_fragment_shader.fs"),
                        )
                        .unwrap()
                        .with_img_texture(
                            image::ImageFormat::Png,
                            self.display.as_ref().unwrap(),
                            include_bytes!("../../screenshots/rune.png"),
                        ),
                    )
                    .with_system(InternalSystem)
                    .with_system(TransformSystem);
            }

            fn handle_main_loop<'a>(
                &mut self,
                event: Event<'a, ()>,
                _: &EventLoopWindowTarget<()>,
                flow: &mut ControlFlow,
            ) {
                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
                        _ => (),
                    };
                }

                let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
                *flow = ControlFlow::WaitUntil(next_frame_time);

                self.world.update(self.display.as_ref().unwrap());
            }
        }

        Window::create("Hey!", Box::new(MetaPlatform::new())).unwrap();
    }
}
