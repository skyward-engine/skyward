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
        any::Any,
        time::{Duration, Instant},
    };

    use ecs::Container;
    use ecs_macro::Component;
    use glium::{
        glutin::{
            event::{Event, WindowEvent},
            event_loop::{ControlFlow, EventLoopWindowTarget},
        },
        index::{NoIndices, PrimitiveType},
        uniforms::EmptyUniforms,
        Display, Program, Surface, VertexBuffer,
    };

    use crate::{
        draw::{ToBuffer, Vertex},
        PlatformHandle, Window,
    };

    #[test]
    fn test_platform() {
        struct MetaPlatform {
            display: Option<Display>,
            entities: Vec<Entity>,
        }

        impl MetaPlatform {
            pub fn new() -> Self {
                Self {
                    display: None,
                    entities: vec![],
                }
            }
        }

        #[derive(Component)]
        struct Entity {
            attrs: Vec<Box<dyn Any>>,
        }

        impl Entity {
            pub fn new() -> Self {
                Self { attrs: vec![] }
            }
        }

        impl PlatformHandle for MetaPlatform {
            fn initialize_display(&mut self, display: Display) {
                self.display = Some(display);
            }

            fn initialize_cache(&mut self) {
                let buffer = Vertex::to_buffer(
                    self.display.as_ref().unwrap(),
                    &[
                        vertex!([-0.5, -0.5], [1.0, 0.0, 0.0]),
                        vertex!([0.0, 0.5], [0.0, 1.0, 0.0]),
                        vertex!([0.5, -0.5], [0.0, 0.0, 1.0]),
                    ],
                )
                .unwrap();

                let vertex_shader_src = r#"
                    #version 140

                    in vec3 position;
                    in vec3 color;
                    out vec3 vColor;

                    void main() {
                        gl_Position = vec4(position, 1.0);
                        vColor = color;
                    }
                "#;

                let fragment_shader_src = r#"
                    #version 140

                    in vec3 vColor;
                    out vec4 f_color;

                    void main() {
                        f_color = vec4(vColor, 1.0);
                    }
                "#;

                let program = Program::from_source(
                    self.display.as_ref().unwrap(),
                    vertex_shader_src,
                    fragment_shader_src,
                    None,
                )
                .unwrap();

                let entity = Entity::new()
                    .with::<Program>(program)
                    .with::<VertexBuffer<Vertex>>(buffer)
                    .with::<NoIndices>(NoIndices(PrimitiveType::TrianglesList));

                self.entities.push(entity);
            }

            fn handle_main_loop<'a>(
                &mut self,
                event: Event<'a, ()>,
                _: &EventLoopWindowTarget<()>,
                flow: &mut ControlFlow,
            ) {
                let display = self.display.as_ref().unwrap();
                let mut target = display.draw();

                target.clear_color(1.0, 0.1, 0.5, 1.0);

                let entity = self.entities.first().unwrap();

                let vertex_buffer = entity.get::<VertexBuffer<Vertex>>().unwrap();
                let program = entity.get::<Program>().unwrap();
                let indices = entity.get::<NoIndices>().unwrap();

                target
                    .draw(
                        vertex_buffer,
                        indices,
                        program,
                        &EmptyUniforms,
                        &Default::default(),
                    )
                    .unwrap();
                target.finish().unwrap(); // todo: unwrap

                let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
                *flow = ControlFlow::WaitUntil(next_frame_time);

                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
                        _ => (),
                    };
                }
            }
        }

        Window::create("Hey!", Box::new(MetaPlatform::new())).unwrap();
    }
}
