/*
    for each VoxelMaterial, run the shader, taking in the global offset and direction as a uniform,
    and the quad_offsets as a buffer.

*/
use bevy::{math::{UVec2, Vec3}, reflect::TypePath, render::render_resource::AsBindGroup};

#[derive(Clone, Copy,Debug)]
pub struct Quad { // todo: bitpack
    pub pos: Vec3,
    _padding: u32,
    pub size: UVec2,
}
impl Quad {
    pub fn from(pos: Vec3, size_x: u32, size_y: u32) -> Self{
        Self {
            pos,
            _padding: 0,
            size: UVec2::new(size_x,size_y),
        }
    }
}

#[derive(Clone)]
pub struct VoxelMaterial {
    pub global_offset: Vec3,
    pub direction: u32, // Side enum cast to u32
    pub quad_offsets: Vec<Vec3>, // instance buffer
    // pub quad_sizes: Vec<Vec2>,
}
impl VoxelMaterial {
    pub fn new() -> Self {
        Self {
            global_offset: Vec3::ZERO,
            // direction: Side::Top,
            direction: 0,
            quad_offsets: Vec::new()
        }
    }
    pub fn arr_init(_index: usize) -> Self {
        Self {
            global_offset: Vec3::ZERO,
            direction: 0,
            quad_offsets: Vec::new()
        }
    }
    pub fn push_quad(&mut self, quad: Quad) {
        self.quad_offsets.push(quad.pos);
    }
}