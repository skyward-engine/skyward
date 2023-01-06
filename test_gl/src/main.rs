use render_gl::window::Window;
use wall::platform::WallPlatform;

mod teapot;
mod wall;

#[profiling::function]
fn main() {
    Window::create(
        "Skyward Engine (teapot demo)",
        Box::new(WallPlatform::new()),
    )
    .unwrap();
}
