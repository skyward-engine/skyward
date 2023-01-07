use ecs::{
    component::Component,
    system::System,
    world::{SystemType, World},
};
use glium::{
    backend::glutin::DisplayCreationError,
    glutin::{
        event::Event,
        event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopWindowTarget},
        platform::windows::EventLoopBuilderExtWindows,
        window::WindowBuilder,
        ContextBuilder,
    },
    Display,
};

use crate::buffer::IndexBufferCreator;

pub struct Window<T> {
    world: World<T>,
    platform: Box<dyn PlatformHandle<T>>,
}

impl<T> Window<T>
where
    T: 'static,
{
    pub fn create(
        platform: impl PlatformHandle<T> + 'static,
    ) -> Result<Self, DisplayCreationError> {
        let world = World::<T>::new();

        // platform.initialize_cache(leaked_buffer);

        let constructed = Self {
            world,
            platform: Box::new(platform),
        };

        Ok(constructed)
    }

    fn create_display(title: &str) -> Result<(Display, EventLoop<()>), DisplayCreationError> {
        let event_loop = EventLoopBuilder::new().with_any_thread(true).build();

        let window_builder = WindowBuilder::new().with_title(title);
        let context_builder = ContextBuilder::new().with_depth_buffer(24);

        let display = Display::new(window_builder, context_builder, &event_loop)?;

        Ok((display, event_loop))
    }

    pub fn init(self, title: &str) -> Result<(), DisplayCreationError> {
        let buffer_creator = Box::new(IndexBufferCreator::new());
        let leaked_buffer = Box::leak(buffer_creator);

        let mut platform = self.platform;
        let (display, event_loop) = Self::create_display(title)?;

        platform.init_world(self.world, &display, leaked_buffer);

        event_loop.run(move |event, target, control_flow| {
            platform.handle_event_loop(&display, event, target, control_flow);
        });
    }

    pub fn system<F>(mut self, system_type: SystemType, system: F) -> Self
    where
        F: System<T> + 'static,
    {
        self.world.with_system(system_type, system);
        self
    }

    pub fn register<F>(mut self) -> Self
    where
        F: Component,
    {
        self.world.register::<F>();
        self
    }

    pub fn borrow_world(&mut self) -> &mut World<T> {
        &mut self.world
    }
}

pub trait PlatformHandle<T> {
    fn init_world(
        &mut self,
        render: World<T>,
        display: &Display,
        buffer: &'static mut IndexBufferCreator,
    );

    fn handle_event_loop<'a>(
        &mut self,
        display: &Display,
        event: Event<'a, ()>,
        target: &EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    );
}
