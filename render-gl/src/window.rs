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

pub struct Window {}

impl Window {
    #![allow(dead_code)]
    pub fn create(
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

pub trait PlatformHandle {
    fn initialize_display(&mut self, display: Display);

    fn initialize_cache(&mut self);

    fn handle_main_loop<'a>(
        &mut self,
        event: Event<'a, ()>,
        target: &EventLoopWindowTarget<()>,
        flow: &mut ControlFlow,
    );
}
