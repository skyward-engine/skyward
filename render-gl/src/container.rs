// macro_rules! declare_vec {
//     ($name:ident<$amount:expr, $($field:ident),+>) => {
//         struct $name {
//             $(
//                 $field: f32,
//             )+
//         }

//         impl $name {
//             pub fn new($($field: f32,)+) -> Self {
//                 Self {
//                     $($field,)+
//                 }
//             }
//         }

//         impl From<[f32; $amount]> for $name {
//             fn from(value: [f32; $amount]) -> Self {
//                 Self {
//                     $(
//                         $field: value[ $i - 1 ],
//                     )+
//                 }
//             }
//         }
//     };
// }

// declare_vec!(Vec2<2, x, y>);

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
