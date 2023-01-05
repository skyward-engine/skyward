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
    buffer::IndexBufferCreator,
    camera::Camera,
    container::{Matrix4, Vec3},
    draw::{
        internal::{InternalSystem, InternalTransformSystem},
        transform::{DrawParametersComponent, Transform},
        vertex::Vertex,
    },
    mesh::Mesh,
    uniform::{perspective::Perspective, MeshUniform},
    window::{PlatformHandle, Window},
};
mod teapot;

struct MetaPlatform {
    display: Option<Display>,
    world: World<Display>,
    fps_counter: u64,
    fps_counter_time: Instant,
}

impl MetaPlatform {
    pub fn new() -> Self {
        Self {
            display: None,
            world: World::new(),
            fps_counter: 0,
            fps_counter_time: Instant::now(),
        }
    }
}

struct TransformSystem;
struct CameraTurnSystem;

impl System<Display> for TransformSystem {
    fn update(
        &mut self,
        manager: &mut ecs::entity::EntityManager,
        table: &mut ecs::entity::EntityQueryTable,
        _: &Display,
    ) -> Option<()> {
        for entity in table.query_single::<MeshUniform>(manager)? {
            let uniform = manager.query_entity::<MeshUniform>(*entity).0?;
            let matrix = uniform.ref_matrix();

            matrix.rotate(0.00005, (1.0, 0.0, 0.0));
        }

        None
    }
}

impl System<Display> for CameraTurnSystem {
    fn update(
        &mut self,
        manager: &mut ecs::entity::EntityManager,
        table: &mut ecs::entity::EntityQueryTable,
        _: &Display,
    ) -> Option<()> {
        for entity in table.query_single::<Camera>(manager)? {
            let mut pos = 0.00006;
            let camera = manager.query_entity::<Camera>(*entity).0?;

            if camera.ref_position()[0] > 2.5 {
                pos = -pos;
            }

            camera
                .add_position(Vec3::from([pos, pos, pos]))
                .add_direction(Vec3::from([-pos, -pos, -pos]))
                .add_up(Vec3::from([pos, pos, pos]));
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
            .with_system(CameraTurnSystem)
            .register::<Transform>();

        {
            let camera_entity = self.world.entity();

            self.world.with::<Camera>(
                camera_entity,
                Camera::new([2.0, -1.0, 1.0], [-2.0, 1.0, 1.0], [0.0, 1.0, 0.0]),
            );
        }

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
                        [0.0, 0.0, 2.0, 1.0f32],
                    ]))
                    .light([-1.0, 0.4, 0.9])
                    .perspective(Perspective::new(
                        self.display.as_ref().unwrap(),
                        3.0,
                        1024.0,
                        0.1,
                    )),
                )
                .with::<DrawParametersComponent>(
                    entity,
                    DrawParametersComponent(DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            ..Default::default()
                        },
                        smooth: Some(glium::Smooth::Nicest),
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
        self.fps_counter += 1;

        if self.fps_counter_time.elapsed().as_secs() >= 1 {
            let fps = self.fps_counter as f64 / self.fps_counter_time.elapsed().as_secs_f64();
            self.display
                .as_ref()
                .unwrap()
                .gl_window()
                .window()
                .set_title(&format!("FOX [{:.2}] (teapot demo)", fps));

            self.fps_counter = 0;
            self.fps_counter_time = std::time::Instant::now();
        }

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
    Window::create("FOX (teapot demo)", Box::new(MetaPlatform::new())).unwrap();
}
