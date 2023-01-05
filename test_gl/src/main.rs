use render_gl::window::Window;
use teapot::platform::TeaPotPlatform;

mod teapot;
mod wall;

#[profiling::function]
fn main() {
    Window::create("Skyward Engine (teapot demo)", Box::new(TeaPotPlatform::new())).unwrap();
}
