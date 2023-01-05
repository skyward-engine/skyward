use glium::{index::PrimitiveType, Display, IndexBuffer};

pub struct IndexBufferCreator {
    index_buffers_u32: Vec<IndexBuffer<u32>>,
    index_buffers_u16: Vec<IndexBuffer<u16>>,
    index_buffers_u8: Vec<IndexBuffer<u8>>,
}

impl IndexBufferCreator {
    pub fn new() -> Self {
        Self {
            index_buffers_u32: vec![],
            index_buffers_u16: vec![],
            index_buffers_u8: vec![],
        }
    }

    pub fn create_index_buffer_u32(
        &mut self,
        display: &Display,
        vertices: &[u32],
        primitive_type: PrimitiveType,
    ) -> &mut Self {
        {
            let index_buffer = IndexBuffer::new(display, primitive_type, vertices).unwrap();
            self.index_buffers_u32.push(index_buffer);
        }

        self
    }

    pub fn get_index_buffer_u32<'a>(&self) -> &IndexBuffer<u32> {
        self.index_buffers_u32.last().unwrap()
    }

    pub fn create_index_buffer_u16(
        &mut self,
        display: &Display,
        vertices: &[u16],
        primitive_type: PrimitiveType,
    ) -> &mut Self {
        {
            let index_buffer = IndexBuffer::new(display, primitive_type, vertices).unwrap();
            self.index_buffers_u16.push(index_buffer);
        }

        self
    }

    pub fn get_index_buffer_u16<'a>(&self) -> &IndexBuffer<u16> {
        self.index_buffers_u16.last().unwrap()
    }

    pub fn create_index_buffer_u8(
        &mut self,
        display: &Display,
        vertices: &[u8],
        primitive_type: PrimitiveType,
    ) -> &mut Self {
        {
            let index_buffer = IndexBuffer::new(display, primitive_type, vertices).unwrap();
            self.index_buffers_u8.push(index_buffer);
        }

        self
    }

    pub fn get_index_buffer_u8<'a>(&self) -> &IndexBuffer<u8> {
        self.index_buffers_u8.last().unwrap()
    }
}
