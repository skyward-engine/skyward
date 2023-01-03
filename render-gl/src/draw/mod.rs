use glium::{implement_vertex, vertex::BufferCreationError, Display, VertexBuffer};
use std::{borrow::Cow, collections::HashMap, hash::Hash};

pub trait ToBuffer: Sized + Copy {
    fn to_buffer(
        display: &Display,
        shape: &[Self],
    ) -> Result<VertexBuffer<Self>, BufferCreationError>;
}

pub struct VertexBufferCache<K, V>
where
    K: PartialEq + Eq + Hash,
    V: Copy + ToBuffer,
{
    buffer_map: HashMap<K, VertexBuffer<V>>,
}

impl<K, V> VertexBufferCache<K, V>
where
    K: PartialEq + Eq + Hash,
    V: Copy + ToBuffer,
{
    pub fn new() -> Self {
        Self {
            buffer_map: HashMap::new(),
        }
    }

    pub fn insert_buffer(&mut self, key: K, buffer: VertexBuffer<V>) {
        self.buffer_map.insert(key, buffer);
    }

    pub fn get_vertex_buffer(&self, key: K) -> Option<&VertexBuffer<V>> {
        self.buffer_map.get(&key)
    }
}

#[derive(Copy, Clone)]
pub struct ColoredVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct TexturedVertex {
    pub position: [f32; 3],
    pub tex_pos: [f32; 2],
}

implement_vertex!(ColoredVertex, position, color);
implement_vertex!(TexturedVertex, position, tex_pos);

#[macro_export]
macro_rules! vertex {
    ([$x:expr, $y:expr, $z:expr], [$r:expr, $g:expr, $b:expr]) => {
        ColoredVertex {
            position: [$x, $y, $z],
            color: [$r, $g, $b],
        }
    };
    ([$x:expr, $y:expr], [$r:expr, $g:expr, $b:expr]) => {
        ColoredVertex {
            position: [$x, $y, 1.0],
            color: [$r, $g, $b],
        }
    };
    ([$x:expr, $y:expr, $z:expr], [$xt:expr, $yt:expr]) => {
        TexturedVertex {
            position: [$x, $y, $z],
            tex_pos: [$xt, $yt],
        }
    };
    ([$x:expr, $y:expr], [$xt:expr, $yt:expr]) => {
        TexturedVertex {
            position: [$x, $y, 1.0],
            tex_pos: [$xt, $yt],
        }
    };
}

impl ToBuffer for ColoredVertex {
    fn to_buffer(
        display: &glium::Display,
        shape: &[Self],
    ) -> Result<VertexBuffer<Self>, BufferCreationError> {
        VertexBuffer::new(display, shape)
    }
}

impl ToBuffer for TexturedVertex {
    fn to_buffer(
        display: &Display,
        shape: &[Self],
    ) -> Result<VertexBuffer<Self>, BufferCreationError> {
        VertexBuffer::new(display, shape)
    }
}
