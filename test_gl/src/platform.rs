use std::time::Instant;

use ecs::world::{SystemType, World};
use glium::{
    glutin::{
        dpi::PhysicalPosition,
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
        window::{CursorGrabMode, Fullscreen},
    },
    index::{NoIndices, PrimitiveType},
    Display, DrawParameters,
};
use image::ImageFormat;
use render_gl::{
    buffer::IndexBufferCreator,
    camera::Camera,
    container::Matrix4,
    draw::{
        delta::TimeDelta, instanced::Instanced, transform::DrawParametersComponent, vertex::Vertex,
    },
    mesh::Mesh,
    uniform::{perspective::Perspective, MeshUniform},
    window::PlatformHandle,
};

pub struct SimplePlatform {
    fps_counter: u64,
    fps_counter_time: Instant,
    world: Option<World<Display>>,
    last_delta: f32,
}

impl SimplePlatform {
    pub fn new() -> Self {
        Self {
            fps_counter: 0,
            fps_counter_time: Instant::now(),
            world: None,
            last_delta: 0.0,
        }
    }
}

impl PlatformHandle<Display> for SimplePlatform {
    fn init_world(
        &mut self,
        mut world: ecs::world::World<Display>,
        display: &Display,
        _: &'static mut IndexBufferCreator,
    ) {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        window.set_fullscreen(Some(Fullscreen::Borderless(None)));

        let normal_tex = include_bytes!("../textures/normal.png");
        let diff_tex = include_bytes!("../textures/diffuse.jpg");

        let camera = world.entity_at(crate::CAMERA_INDEX);
        let delta = world.entity_at(crate::TIME_DELTA_INDEX);

        world.with::<Camera>(
            camera,
            Camera::new([2.0, -1.0, 1.0], [-2.0, 1.0, 1.0], [0.0, 1.0, 0.0]),
        );

        world.with::<TimeDelta>(delta, TimeDelta::new());

        let wall_mesh_entity = world.entity_at(crate::WALL_MESH_ENTITY);
        println!("{}", wall_mesh_entity);

        world
            .with::<MeshUniform>(
                wall_mesh_entity,
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
                wall_mesh_entity,
                DrawParametersComponent(DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    smooth: Some(glium::Smooth::Nicest),
                    ..Default::default()
                }),
            )
            .with::<Mesh>(
                wall_mesh_entity,
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
            );

        let mut walls = (0..20000)
            .map(|_| {
                let pos: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
                let dir: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
                let pos = (pos.0 * 1.5 - 0.75, pos.1 * 1.5 - 0.75, pos.2 * 1.5 - 0.75);
                let dir = (dir.0 * 1.5 - 0.75, dir.1 * 1.5 - 0.75, dir.2 * 1.5 - 0.75);
                (pos, dir)
            })
            .collect::<Vec<_>>();

        let mut i = 0;
        for src in walls.iter_mut() {
            (src.0).0 += (src.1).0 * 0.00001;
            (src.0).1 += (src.1).1 * 0.00001;
            (src.0).2 += (src.1).2 * 0.00001;

            let entity = world.entity_at(wall_mesh_entity + 1 + i);
            i += 1;

            world.with::<Instanced>(
                entity,
                Instanced::create(
                    crate::WALL_MESH_ENTITY as u32,
                    ((src.0).0, (src.0).1, (src.0).2),
                ),
            );
        }

        self.world = Some(world);
    }

    fn handle_event_loop<'a>(
        &mut self,
        display: &Display,
        event: Event<'a, ()>,
        _: &glium::glutin::event_loop::EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    ) {
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

        let world = self.world.as_mut().unwrap();
        world.update(SystemType::Loop, &display);

        let table = &mut world.entity_query_table;
        let manager = &mut world.entity_manager;

        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Focused(focused) => {
                    let gl_window = display.gl_window();
                    let window = gl_window.window();

                    if focused {
                        window.set_cursor_visible(false);
                        window
                            .set_cursor_grab(CursorGrabMode::Confined)
                            .expect("Unable to grab cursor");
                    } else {
                        window.set_cursor_visible(true);
                        window
                            .set_cursor_grab(CursorGrabMode::None)
                            .expect("Unable to grab cursor");
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let entity = table.query_first_single::<Camera>(manager).unwrap();
                    let camera = manager.query_entity::<Camera>(*entity).0.unwrap();

                    let (x, y): (f32, f32) = position.into();
                    let (last_x, last_y) = camera.get_last_pos();

                    let dx: f32 = x - last_x;
                    let dy: f32 = y - last_y;

                    let move_x = (dx as f32 * 50.5) * self.last_delta;
                    let move_y = (dy as f32 * 50.5) * self.last_delta;

                    camera.change_direction(move_x, move_y);

                    let gl_window = display.gl_window();
                    let window = gl_window.window();

                    let (width, height): (u32, u32) = window.inner_size().into();
                    let new_pos = PhysicalPosition::new(width as f64 / 2.0, height as f64 / 2.0);

                    camera.update_pos(new_pos.into());
                    window.set_cursor_position(new_pos).unwrap();
                }
                _ => (),
            };
        }

        let entity = table
            .query_single::<TimeDelta>(manager)
            .unwrap()
            .first()
            .unwrap();

        let delta = manager.query_entity::<TimeDelta>(*entity).0.unwrap();
        self.last_delta = delta.get_time_delta_sec();

        delta.update_time_delta();

        // *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(10));
    }
}
