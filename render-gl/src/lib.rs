use glium::{
    backend::glutin::DisplayCreationError,
    glutin::{
        event::Event,
        event_loop::{ControlFlow, EventLoopBuilder, EventLoopWindowTarget},
        platform::windows::EventLoopBuilderExtWindows,
        window::WindowBuilder,
        ContextBuilder,
    },
    Display,
};

#[macro_use]
pub mod draw;

pub struct Window {}

impl Window {
    #![allow(dead_code)]
    fn create(
        title: &str,
        mut platform: Box<dyn PlatformHandle>,
    ) -> Result<(), DisplayCreationError> {
        let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
        let window_builder = WindowBuilder::new().with_title(title);
        let context_builder = ContextBuilder::new();

        let display = Display::new(window_builder, context_builder, &event_loop)?;

        platform.initialize_display(display);
        platform.initialize_cache();
        event_loop.run(move |event, target, control_flow| {
            (*platform).handle_main_loop(event, target, control_flow)
        });
    }
}

trait PlatformHandle {
    fn initialize_display(&mut self, display: Display);

    fn initialize_cache(&mut self);

    fn handle_main_loop<'a>(
        &mut self,
        event: Event<'a, ()>,
        target: &EventLoopWindowTarget<()>,
        flow: &mut ControlFlow,
    );
}

#[cfg(test)]
mod test {
    use std::{
        io::Cursor,
        time::{Duration, Instant},
    };

    use ecs::{system::System, world::World};
    use ecs_macro::EntityComponent;
    use glium::{
        glutin::{
            event::{Event, WindowEvent},
            event_loop::{ControlFlow, EventLoopWindowTarget},
        },
        index::{NoIndices, PrimitiveType},
        texture::SrgbTexture2d,
        uniform,
        uniforms::{AsUniformValue, Uniforms, UniformsStorage},
        Display, Program, Surface, VertexBuffer,
    };

    use crate::{
        draw::{TexturedVertex, ToBuffer},
        PlatformHandle, Window,
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
                let entities = table.query::<(
                    VertexBufferContainer,
                    ProgramContainer,
                    NoIndicesContainer,
                    Transform,
                    TwoDimensionalRgbTextureContainer,
                )>(manager)?;

                for entity in entities {
                    let (
                        buffer,
                        program,
                        indices,
                        transform,
                        texture
                    ) = manager.query_entity_five::<VertexBufferContainer, ProgramContainer, NoIndicesContainer, Transform, TwoDimensionalRgbTextureContainer>(entity)?;
                    let mut target = display.draw();

                    let t = transform.0;
                    let texture = &texture.0;

                    target.clear_color(1.0, 1.0, 1.0, 1.0);
                    let uniform = &uniform! {
                        matrix: [
                            [ t.cos(), t.sin(), 0.0, 0.0],
                            [-t.sin(), t.cos(), 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ],
                        tex: texture,
                    };

                    target
                        .draw(
                            &buffer.0,
                            &indices.0,
                            &program.0,
                            uniform,
                            &Default::default(),
                        )
                        .unwrap();
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
                let entities = table.query_single::<Transform>(manager)?;

                for entity in entities {
                    let transform = manager.query_entity::<Transform>(*entity)?.0;
                    let t = transform;

                    t.0 += 0.0002;
                    if t.0 > 0.5 {
                        // t.0 = -0.5;
                    }
                }

                None
            }
        }

        #[derive(EntityComponent)]
        struct Transform(f32);

        #[derive(EntityComponent)]
        struct ProgramContainer(Program);

        #[derive(EntityComponent)]
        struct VertexBufferContainer(VertexBuffer<TexturedVertex>);

        #[derive(EntityComponent)]
        struct NoIndicesContainer(NoIndices);

        #[derive(EntityComponent)]
        struct TwoDimensionalRgbTextureContainer(SrgbTexture2d);

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
                let buffer = TexturedVertex::to_buffer(
                    self.display.as_ref().unwrap(),
                    &[
                        vertex!([-0.5, -0.5], [0.0, 0.0]),
                        vertex!([0.5, -0.5], [0.0, 1.0]),
                        vertex!([-0.5, 0.5], [1.0, 0.0]),
                        vertex!([0.5, 0.5], [1.0, 1.0]),
                    ],
                )
                .unwrap();

                let vertex_shader_src = include_str!("../../shaders/test_vertex_shader.vert");
                let fragment_shader_src = include_str!("../../shaders/test_fragment_shader.fs");

                let program = Program::from_source(
                    self.display.as_ref().unwrap(),
                    vertex_shader_src,
                    fragment_shader_src,
                    None,
                )
                .unwrap();

                let entity = self.world.entity();

                let image = image::load(
                    Cursor::new(&include_bytes!("../../screenshots/rune.png")),
                    image::ImageFormat::Png,
                )
                .unwrap()
                .to_rgba8();
                let dimensions = image.dimensions();
                let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
                    &image.into_raw(),
                    dimensions,
                );
                let texture =
                    glium::texture::SrgbTexture2d::new(self.display.as_ref().unwrap(), image)
                        .unwrap();

                self.world
                    .with::<Transform>(entity, Transform(-0.5))
                    .with::<ProgramContainer>(entity, ProgramContainer(program))
                    .with::<VertexBufferContainer>(entity, VertexBufferContainer(buffer))
                    .with(entity, TwoDimensionalRgbTextureContainer(texture))
                    .with::<NoIndicesContainer>(
                        entity,
                        NoIndicesContainer(NoIndices(PrimitiveType::TriangleStrip)),
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
                // *flow = ControlFlow::WaitUntil(next_frame_time);

                self.world.update(self.display.as_ref().unwrap());
            }
        }

        Window::create("Hey!", Box::new(MetaPlatform::new())).unwrap();
    }
}
