#[macro_use]
pub mod draw;
pub mod container;
mod teapot;
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
        index::PrimitiveType,
        Display,
    };

    use crate::{
        draw::{
            internal::InternalSystem,
            mesh::{IndexBufferCreator, Mesh, Transform},
            Vertex,
        },
        teapot,
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
                    transform.rotate(0.005, (1.0, 0.0, 0.0));
                    // transform.translate(0.1, 0.3, 0.2);
                }

                None
            }
        }

        impl PlatformHandle for MetaPlatform {
            fn initialize_display(&mut self, display: Display) {
                self.display = Some(display);
            }

            fn initialize_cache(&mut self, buffer_creator: &'static mut IndexBufferCreator) {
                self.world
                    .with_system(InternalSystem)
                    .with_system(TransformSystem)
                    .register::<Transform>();

                // tea pot
                {
                    let entity = self.world.entity();

                    self.world
                        .with::<Mesh>(
                            entity,
                            Mesh::buffered(
                                self.display.as_ref().unwrap(),
                                Vertex::from_vertices(
                                    self.display.as_ref().unwrap(),
                                    &teapot::VERTICES,
                                    &teapot::NORMALS,
                                ),
                                buffer_creator
                                    .create_index_buffer_u16(
                                        self.display.as_ref().unwrap(),
                                        &teapot::INDICES,
                                        PrimitiveType::TrianglesList,
                                    )
                                    .get_index_buffer_u16(),
                                include_str!("../../shaders/tea_vertex_shader.vert"),
                                include_str!("../../shaders/tea_fragment_shader.fs"),
                            )
                            .unwrap(),
                        )
                        .with::<Transform>(
                            entity,
                            Transform::from([
                                [0.01, 0.0, 0.0, 0.0],
                                [0.0, 0.01, 0.0, 0.0],
                                [0.0, 0.0, 0.01, 0.0],
                                [0.0, 0.0, 0.0, 1.0f32],
                            ]),
                        );
                }
            }

            fn handle_main_loop<'b>(
                &mut self,
                event: Event<'b, ()>,
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
                // *flow = ControlFlow::WaitUntil(next_frame_time);

                self.world.update(self.display.as_ref().unwrap());
            }
        }

        Window::create("Hey!", Box::new(MetaPlatform::new())).unwrap();
    }
}
