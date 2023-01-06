use std::{
    cell::RefCell,
    time::{Duration, Instant},
};

use ecs::world::{SystemType, World};
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
    },
    index::{NoIndices, PrimitiveType},
    Display, DrawParameters,
};
use image::ImageFormat;
use render_gl::{
    buffer::IndexBufferCreator,
    camera::Camera,
    container::Matrix4,
    draw::{delta::TimeDelta, transform::DrawParametersComponent, vertex::Vertex},
    mesh::Mesh,
    uniform::{perspective::Perspective, MeshUniform},
    window::PlatformHandle,
};

pub struct SimplePlatform {
    fps_counter: u64,
    fps_counter_time: Instant,
    world: Option<World<Display>>,
    last_time: Instant,
    current_time: Instant,
}

impl SimplePlatform {
    pub fn new() -> Self {
        Self {
            fps_counter: 0,
            fps_counter_time: Instant::now(),
            world: None,
            last_time: Instant::now(),
            current_time: Instant::now(),
        }
    }
}

impl PlatformHandle<Display> for SimplePlatform {
    fn init_world(
        &mut self,
        mut world: ecs::world::World<Display>,
        display: &Display,
        _: &mut IndexBufferCreator,
    ) {
        let normal_tex = include_bytes!("../textures/normal.png");
        let diff_tex = include_bytes!("../textures/diffuse.jpg");

        let camera = world.entity_at(crate::CAMERA_INDEX);
        let delta = world.entity_at(crate::TIME_DELTA_INDEX);

        let entity = world.entity_at(crate::WALL_INDEX);

        world.with::<Camera>(
            camera,
            Camera::new([2.0, -1.0, 1.0], [-2.0, 1.0, 1.0], [0.0, 1.0, 0.0]),
        );

        world.with::<TimeDelta>(delta, TimeDelta::new());

        world
            .with::<Mesh>(
                entity,
                Mesh::buffered(
                    &display,
                    Vertex::from_vertices_with_tex(
                        &display,
                        &crate::wall::VERTICES,
                        &crate::wall::NORMALS,
                        &crate::wall::TEX_POS,
                    ),
                    NoIndices(PrimitiveType::TriangleStrip),
                    include_str!("../shaders/wall_vertex_shader.vert"),
                    include_str!("../shaders/wall_fragment_shader.fs"),
                )
                .unwrap(),
            )
            .with::<MeshUniform>(
                entity,
                MeshUniform::new(Matrix4::from([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 2.0, 1.0f32],
                ]))
                .light([-1.0, 0.4, 0.9])
                .perspective(Perspective::new(&display, 3.0, 1024.0, 0.1))
                .with_img_2d_diff_texture(ImageFormat::Jpeg, &display, diff_tex)
                .with_img_2d_norm_texture(ImageFormat::Png, &display, normal_tex),
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

        self.world = Some(world);
    }

    fn handle_event_loop<'a>(
        &mut self,
        display: &Display,
        event: Event<'a, ()>,
        _: &glium::glutin::event_loop::EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    ) {
        self.current_time = Instant::now();
        self.fps_counter += 1;

        if self.fps_counter_time.elapsed().as_secs() >= 1 {
            let fps = self.fps_counter as f64 / self.fps_counter_time.elapsed().as_secs_f64();
            display
                .gl_window()
                .window()
                .set_title(&format!("Skyward Engine [{:.2}] (wall demo)", fps));

            self.fps_counter = 0;
            self.fps_counter_time = std::time::Instant::now();
        }

        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            };
        }

        let _ = Instant::now() + Duration::from_nanos(16_666_667);
        // *flow = ControlFlow::WaitUntil(next_frame_time);

        let world = self.world.as_mut().unwrap();
        world.update(SystemType::Loop, &display);

        let table = &mut world.entity_query_table;
        let manager = &mut world.entity_manager;

        let entity = table
            .query_single::<TimeDelta>(manager)
            .unwrap()
            .first()
            .unwrap();

        let delta = manager.query_entity::<TimeDelta>(*entity).0.unwrap();

        delta.update_time_delta();

        self.last_time = self.current_time;
    }
}
