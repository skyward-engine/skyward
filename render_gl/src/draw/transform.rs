use crate::container::Matrix4;

use ecs_macro::EntityComponent;
use glium::DrawParameters;

#[derive(EntityComponent)]
pub struct DrawParametersComponent(pub DrawParameters<'static>);

#[derive(EntityComponent)]
pub struct Transform {
    pub matrix: Matrix4,
}

impl From<[[f32; 4]; 4]> for Transform {
    fn from(value: [[f32; 4]; 4]) -> Self {
        Self {
            matrix: Matrix4::from(value),
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Self {
            matrix: Matrix4::from([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
        }
    }

    pub fn ref_matrix(&mut self) -> &mut Matrix4 {
        &mut self.matrix
    }

    pub fn inner(&self) -> [[f32; 4]; 4] {
        self.matrix.inner()
    }
}
