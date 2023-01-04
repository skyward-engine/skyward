#[macro_use]
pub mod draw;
pub mod container;
pub mod window;

#[cfg(test)]
mod test {
    use std::time::{Duration, Instant};

    use ecs::{system::System, world::World};
    use ecs_macro::EntityComponent;
    use glium::{
        glutin::{
            event::{Event, WindowEvent},
            event_loop::{ControlFlow, EventLoopWindowTarget},
        },
        index::{NoIndices, PrimitiveType},
        uniform,
        uniforms::{AsUniformValue, Uniforms, UniformsStorage},
        Display, Surface,
    };

    use crate::{
        draw::{
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

        struct InternalSystem;
        struct TransformSystem;

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
                    let (mesh, _) = (entries.0?, entries.1);

                    let matrix: [[f32; 4]; 4] = [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ];

                    // if let Some(transform) = transform {
                    //     matrix = transform.inner();
                    // }

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

        impl System<Display> for TransformSystem {
            fn update(
                &mut self,
                manager: &mut ecs::entity::EntityManager,
                table: &mut ecs::entity::EntityQueryTable,
                _: &Display,
            ) -> Option<()> {
                for entity in table.query_single::<Transform>(manager)? {
                    let transform = manager.query_entity::<Transform>(*entity).0?;
                    transform.rotate(1.0, (1.0, 1.0, 1.0));
                }

                None
            }
        }

        #[derive(EntityComponent)]
        struct MatrixContainer([[f32; 4]; 4]);

        #[derive(EntityComponent)]
        struct UniformContainer<T, R>(UniformsStorage<'static, T, R>)
        where
            T: AsUniformValue + 'static,
            R: Uniforms + 'static;

        impl PlatformHandle for MetaPlatform {
            fn initialize_display(&mut self, display: Display) {
                self.display = Some(display);
            }

            fn initialize_cache(&mut self) {
                let vertex_shader_src = include_str!("../../shaders/test_vertex_shader.vert");
                let fragment_shader_src = include_str!("../../shaders/test_fragment_shader.fs");

                let entity = self.world.entity();

                self.world
                    // .with::<Transform>(entity, Transform(-0.5))
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
                            vertex_shader_src,
                            fragment_shader_src,
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
