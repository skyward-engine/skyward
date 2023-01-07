use std::f32::consts::PI;

use ecs::system::MultiThreadSystem;
use ecs_macro::EntityComponent;

use crate::{
    container::{Matrix4, Vec3},
    draw::delta::TimeDelta,
};

struct MouseCamera;

impl MultiThreadSystem for MouseCamera {
    fn update(
        &mut self,
        manager: &mut ecs::entity::EntityManager,
        table: &mut ecs::entity::EntityQueryTable,
    ) -> Option<()> {
        let delta = table
            .query_first_single::<TimeDelta>(manager)
            .expect("No TimeDelta initialized!");
        let delta = manager.query_entity::<TimeDelta>(*delta).0?;
        let time_delta = delta.get_time_delta_sec();

        let camera = table
            .query_first_single::<Camera>(manager)
            .expect("No camera initialized!");

        let camera = manager.query_entity::<Camera>(*camera).0?;

        match camera.input_event {
            InputEvent::MouseMovement(x, y) => {
                let (last_x, last_y) = camera.get_last_pos();

                let dx: f32 = x - last_x;
                let dy: f32 = y - last_y;

                let move_x = (dx as f32 * 50.5) * time_delta;
                let move_y = (dy as f32 * 50.5) * time_delta;

                camera.change_direction(move_x, move_y);
            }
            _ => println!("Unhandled event: {:?}", camera.input_event),
        }

        None
    }
}

#[derive(EntityComponent, Debug, Clone)]
pub struct Camera {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    last_pos: (f32, f32),
    input_event: InputEvent,
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    MouseMovement(f32, f32),
    MouseButtonPressed,
    MouseButtonReleased,
    KeyPressed(u32),
    KeyReleased(u32),
    Quit,
    None,
}

impl From<[[f32; 3]; 3]> for Camera {
    fn from(value: [[f32; 3]; 3]) -> Self {
        Self::new(value[0], value[1], value[2])
    }
}

impl Camera {
    pub fn new(position: impl Into<Vec3>, direction: impl Into<Vec3>, up: impl Into<Vec3>) -> Self {
        Self {
            position: position.into(),
            direction: direction.into(),
            up: up.into(),
            last_pos: (0.0, 0.0),
            input_event: InputEvent::None,
        }
    }

    pub fn input_event(&mut self, event: InputEvent) {
        self.input_event = event;
    }

    pub fn position(&mut self, position: impl Into<Vec3>) {
        self.position = position.into();
    }

    pub fn direction(&mut self, direction: impl Into<Vec3>) {
        self.direction = direction.into();
    }

    pub fn up(&mut self, up: impl Into<Vec3>) {
        self.up = up.into();
    }

    pub fn ref_position(&self) -> &Vec3 {
        &self.position
    }

    pub fn ref_direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn ref_up(&self) -> &Vec3 {
        &self.up
    }

    pub fn add_position(&mut self, position: impl Into<Vec3>) -> &mut Self {
        self.position = self.position + position.into();
        self
    }

    pub fn add_direction(&mut self, direction: impl Into<Vec3>) -> &mut Self {
        self.direction = self.direction + direction.into();
        self
    }

    pub fn add_up(&mut self, up: impl Into<Vec3>) -> &mut Self {
        self.up = self.up + up.into();
        self
    }

    pub fn get_last_pos(&self) -> (f32, f32) {
        self.last_pos
    }

    pub fn update_pos(&mut self, pos: (f32, f32)) {
        self.last_pos = pos;
    }

    pub fn change_direction(&mut self, yaw: f32, pitch: f32) {
        let yaw = yaw * PI / 180.0;
        let pitch = pitch * PI / 180.0;
        let x = self.direction[0] * pitch.cos() - self.direction[1] * pitch.sin();
        let y = self.direction[0] * pitch.sin() + self.direction[1] * pitch.cos();
        let z = self.direction[2];

        self.direction = Vec3::new(x, y, z);

        let x = self.direction[0] * yaw.cos() + self.direction[2] * yaw.sin();
        let y = self.direction[1];
        let z = -self.direction[0] * yaw.sin() + self.direction[2] * yaw.cos();

        self.direction = Vec3::new(x, y, z);
    }

    pub fn view_matrix(&self) -> Matrix4 {
        let f = {
            let len = (self.direction[0] * self.direction[0]
                + self.direction[1] * self.direction[1]
                + self.direction[2] * self.direction[2])
                .sqrt();

            [
                self.direction[0] / len,
                self.direction[1] / len,
                self.direction[2] / len,
            ]
        };

        let up = self.up;
        let s = [
            up[1] * f[2] - up[2] * f[1],
            up[2] * f[0] - up[0] * f[2],
            up[0] * f[1] - up[1] * f[0],
        ];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

        let u = [
            f[1] * s_norm[2] - f[2] * s_norm[1],
            f[2] * s_norm[0] - f[0] * s_norm[2],
            f[0] * s_norm[1] - f[1] * s_norm[0],
        ];

        let position = self.position;
        let p = [
            -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
            -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
            -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
        ];

        Matrix4::from([
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ])
    }
}
