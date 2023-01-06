use ecs::world::SystemType;
use glium::{backend::glutin::DisplayCreationError, Display};
use platform::SimplePlatform;
use render_gl::{
    draw::{
        internal::GlRenderSystem,
        transform::{DrawParametersComponent, Transform},
    },
    uniform::MeshUniform,
    window::Window,
};
use rotate::WallRotateSystem;

mod platform;
mod rotate;
mod wall;

pub const WALL_INDEX: usize = 0;
pub const CAMERA_INDEX: usize = 1;
pub const TIME_DELTA_INDEX: usize = 2;

#[profiling::function]
fn main() -> Result<(), DisplayCreationError> {
    Window::<Display>::create(SimplePlatform::new())?
        .system(SystemType::Loop, GlRenderSystem)
        .system(SystemType::Loop, WallRotateSystem)
        .register::<MeshUniform>()
        .register::<Transform>()
        .register::<DrawParametersComponent>()
        .init("Skyward Engine")
}
