use glium::{implement_vertex, vertex::BufferCreationError, Display, VertexBuffer};

pub trait ToBuffer: Sized + Copy {
    fn to_buffer(
        display: &Display,
        shape: &[Self],
    ) -> Result<VertexBuffer<Self>, BufferCreationError>;
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_pos: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn empty() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            tex_pos: [0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
        }
    }

    pub fn from_vertices(
        display: &Display,
        vertices: &[(f32, f32, f32)],
        normals: &[(f32, f32, f32)],
    ) -> VertexBuffer<Vertex> {
        if vertices.len() != normals.len() {
            // todo: proper error handling
            panic!("Vertices and normals should be the same length!");
        };

        let mut vertex_vec = Vec::<Vertex>::new();

        for i in 0..vertices.len() {
            let position = vertices[i];
            let normal = normals[i];

            let vertex = Vertex {
                position: [position.0, position.1, position.2],
                normal: [normal.0, normal.1, normal.2],
                tex_pos: [0.0, 0.0],
            };

            vertex_vec.push(vertex);
        }

        Self::to_buffer(display, &vertex_vec).unwrap()
    }

    pub fn from_vertices_with_tex(
        display: &Display,
        vertices: &[(f32, f32, f32)],
        normals: &[(f32, f32, f32)],
        tex_pos: &[(f32, f32)],
    ) -> VertexBuffer<Vertex> {
        if (vertices.len() != normals.len()) || (vertices.len() != tex_pos.len()) {
            // todo: proper error handling
            panic!("Vertices, texture position and normals should be the same length!");
        };

        let mut vertex_vec = Vec::<Vertex>::new();

        for i in 0..vertices.len() {
            let texture_position = tex_pos[i];
            let position = vertices[i];
            let normal = normals[i];

            let vertex = Vertex {
                position: [position.0, position.1, position.2],
                normal: [normal.0, normal.1, normal.2],
                tex_pos: [texture_position.0, texture_position.1],
            };

            vertex_vec.push(vertex);
        }

        Self::to_buffer(display, &vertex_vec).unwrap()
    }
}

implement_vertex!(Vertex, position, tex_pos, normal);

#[macro_export]
macro_rules! vertex {
    ([$x:expr, $y:expr, $z:expr], [$xt:expr, $yt:expr]) => {
        Vertex {
            position: [$x, $y, $z],
            tex_pos: [$xt, $yt],
            normal: [0.0, 0.0, 0.0],
        }
    };
    ([$x:expr, $y:expr], [$xt:expr, $yt:expr]) => {
        Vertex {
            position: [$x, $y, 1.0],
            tex_pos: [$xt, $yt],
            normal: [0.0, 0.0, 0.0],
        }
    };
    ([$x:expr, $y:expr, $z:expr], [$xt:expr, $yt:expr], [$nx:expr, $ny:expr, $nz:expr]) => {
        Vertex {
            position: [$x, $y, $z],
            tex_pos: [$xt, $yt],
            normal: [$nx, $ny, $nz],
        }
    };
    ([$x:expr, $y:expr], [$xt:expr, $yt:expr], [$nx:expr, $ny:expr, $nz:expr]) => {
        Vertex {
            position: [$x, $y, 1.0],
            tex_pos: [$xt, $yt],
            normal: [$nx, $ny, $nz],
        }
    };
}

impl ToBuffer for Vertex {
    fn to_buffer(
        display: &glium::Display,
        shape: &[Self],
    ) -> Result<VertexBuffer<Self>, BufferCreationError> {
        VertexBuffer::new(display, shape)
    }
}
