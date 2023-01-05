use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    x: Vec4,
    y: Vec4,
    z: Vec4,
    w: Vec4,
}

impl Matrix4 {
    pub fn new() -> Self {
        Self::from([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let translate_matrix = Matrix4::from([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let multiplied = multiply(translate_matrix, *self);

        self[0] = multiplied[0];
        self[1] = multiplied[1];
        self[2] = multiplied[2];
        self[3] = multiplied[3];
    }

    pub fn rotate(&mut self, angle: f32, axis: (f32, f32, f32)) {
        let (x, y, z) = axis;
        let c = angle.cos();
        let s = angle.sin();
        let rotate_matrix = Matrix4::from([
            [
                x * x * (1.0 - c) + c,
                y * x * (1.0 - c) + z * s,
                z * x * (1.0 - c) - y * s,
                0.0,
            ],
            [
                x * y * (1.0 - c) - z * s,
                y * y * (1.0 - c) + c,
                z * y * (1.0 - c) + x * s,
                0.0,
            ],
            [
                x * z * (1.0 - c) + y * s,
                y * z * (1.0 - c) - x * s,
                z * z * (1.0 - c) + c,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let multiplied = multiply(rotate_matrix, *self);

        self[0] = multiplied[0];
        self[1] = multiplied[1];
        self[2] = multiplied[2];
        self[3] = multiplied[3];
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        let scale_matrix = Matrix4::from([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let multiplied = multiply(scale_matrix, *self);

        self[0] = multiplied[0];
        self[1] = multiplied[1];
        self[2] = multiplied[2];
        self[3] = multiplied[3];
    }

    pub fn inner(&self) -> [[f32; 4]; 4] {
        let first = self[0];
        let second = self[1];
        let third = self[2];
        let fourth = self[3];

        [first.inner(), second.inner(), third.inner(), fourth.inner()]
    }
}

pub fn multiply(a: Matrix4, b: Matrix4) -> Matrix4 {
    let mut result = Matrix4::new();
    for i in 0..4 {
        for j in 0..4 {
            result[i][j] =
                a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
        }
    }
    result
}

unsafe impl Send for Matrix4 {}
unsafe impl Sync for Matrix4 {}

impl From<[[f32; 4]; 4]> for Matrix4 {
    fn from(value: [[f32; 4]; 4]) -> Self {
        Self {
            x: Vec4::from(value[0]),
            y: Vec4::from(value[1]),
            z: Vec4::from(value[2]),
            w: Vec4::from(value[3]),
        }
    }
}

impl Index<usize> for Matrix4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Matrix4 index out of range"),
        }
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Matrix4 index out of range"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

unsafe impl Send for Vec4 {}
unsafe impl Sync for Vec4 {}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn inner(&self) -> [f32; 4] {
        [self[0], self[1], self[2], self[3]]
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(value: [f32; 4]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
            w: value[3],
        }
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Vec4 index out of range"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Vec4 index out of range"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

unsafe impl Send for Vec3 {}
unsafe impl Sync for Vec3 {}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn inner(&self) -> [f32; 3] {
        [self[0], self[1], self[2]]
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of range"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of range"),
        }
    }
}
