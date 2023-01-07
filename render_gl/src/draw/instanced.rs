use ecs_macro::EntityComponent;
use glium::implement_vertex;

#[derive(EntityComponent, Debug, Clone, Copy)]
pub struct Instanced {
    pub mesh_entity: u32,
    pub world_position: (f32, f32, f32),
}

implement_vertex!(Instanced, world_position);

impl Instanced {
    pub fn new(mesh_entity: u32) -> Self {
        Self {
            mesh_entity,
            world_position: (0.0, 0.0, 0.0),
        }
    }

    pub fn create(mesh_entity: u32, location: (f32, f32, f32)) -> Self {
        Self {
            mesh_entity,
            world_position: location,
        }
    }

    pub fn location(mut self, location: (f32, f32, f32)) -> Self {
        self.world_position = location;
        self
    }
}
