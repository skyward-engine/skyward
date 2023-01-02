use glium::{implement_vertex, vertex::BufferCreationError, Display, VertexBuffer};
use std::{collections::HashMap, hash::Hash};

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
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

#[macro_export]
macro_rules! vertex {
    ([$x:expr, $y:expr, $z:expr], [$r:expr, $g:expr, $b:expr]) => {
        Vertex {
            position: [$x, $y, $z],
            color: [$r, $g, $b],
        }
    };
    ([$x:expr, $y:expr], [$r:expr, $g:expr, $b:expr]) => {
        Vertex {
            position: [$x, $y, 1.0],
            color: [$r, $g, $b],
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
