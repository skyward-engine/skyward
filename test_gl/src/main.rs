use std::time::{Duration, Instant};

use ecs::{system::System, world::World};
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoopWindowTarget},
    },
    index::PrimitiveType,
    Display, DrawParameters,
};
use render_gl::{
    container::Matrix4,
    draw::{
        internal::{InternalSystem, InternalTransformSystem},
        mesh::{DrawParametersComponent, IndexBufferCreator, Mesh, MeshUniform, Transform},
        Vertex,
    },
    window::{PlatformHandle, Window},
};
mod teapot;

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
            transform.rotate(0.00005, (1.0, 0.0, 0.0));
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
            .with_system(InternalTransformSystem)
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
                .with::<MeshUniform>(
                    entity,
                    MeshUniform::new(Matrix4::from([
                        [0.01, 0.0, 0.0, 0.0],
                        [0.0, 0.01, 0.0, 0.0],
                        [0.0, 0.0, 0.01, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32],
                    ]))
                    .light([-1.0, 0.4, 0.9]),
                )
                .with::<Transform>(
                    entity,
                    Transform::from([
                        [0.01, 0.0, 0.0, 0.0],
                        [0.0, 0.01, 0.0, 0.0],
                        [0.0, 0.0, 0.01, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32],
                    ]),
                )
                .with::<DrawParametersComponent>(
                    entity,
                    DrawParametersComponent(DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
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

        let _ = Instant::now() + Duration::from_nanos(16_666_667);
        // *flow = ControlFlow::WaitUntil(next_frame_time);

        self.world.update(self.display.as_ref().unwrap());
    }
}

// todo: add profiling
#[profiling::function]
fn main() {
    Window::create("Hey!", Box::new(MetaPlatform::new())).unwrap();
}