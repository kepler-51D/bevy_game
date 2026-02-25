use std::{array::from_fn, sync::Arc};

use bevy::{
    ecs::component::Component,
    math::{IVec3, UVec2, UVec3}, prelude::{Deref, DerefMut},
};
use strum_macros::EnumIter;

pub type BlockData = Arc<[[[BlockID;CHUNKSIZE];CHUNKSIZE];CHUNKSIZE]>;

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BlockID {
    Air,
    Stone,
}
#[derive(Component)]
pub struct NeedsMeshUpdate;

#[derive(Component)]
pub struct Renderable;

pub const CHUNKSIZE: usize = 32;

/// represents any of the 6 cardinal directions
#[repr(u8)]
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}
/// cast the direction enum to a usize and index this array to get the direction
/// as a vector
pub const DIRECTION_VECS: [IVec3; 6] = [
    IVec3::Y,
    IVec3::NEG_Y,
    IVec3::NEG_X,
    IVec3::X,
    IVec3::Z,
    IVec3::NEG_Z,
];

/// stores the position and blockdata for a chunk.
#[derive(Component)]
pub struct Chunk {
    pub data: BlockData,
    pub pos: IVec3,
}

/// contains the data for one quad
/// each column of the pos vector is guaranteed to be between 0 and 31, inclusive
#[derive(Debug,Clone, Copy)]
pub struct Quad {
    pub pos: UVec3,
}
impl Quad {
    pub fn new(pos: UVec3) -> Self {
        Self {
            pos,
        }
    }
}
/// lowest 5 bits are z, next 5 bits are y, next 5 bits are x
/// (labelled X, Y and Z)
/// 
/// next 5 bits are for the width of the quad, next 5 bits are for height of quad
/// (labelled W and H)
/// 
/// next 3 bits are direction of quad
/// (Labelled D)
/// 
/// last 4 bits are blockID
/// (labelled B)
/// 
/// BBBBDDDWWWWWHHHHHXXXXXYYYYYZZZZZ
/// 
/// (seperated for readability)
/// 
/// BBBB_DDD_WWWWW_HHHHH_XXXXX_YYYYY_ZZZZZ
pub struct GreedyQuad {
    pub data: u32
}
impl GreedyQuad {
    pub fn set_pos(&mut self, pos: UVec3) {
        let new: u32 = pos.x | (pos.y << 5) | (pos.z << 10);
        self.data = (self.data & !31) | new;
    }
    pub fn get_pos(&self) -> UVec3 {
        let mut pos: UVec3 = UVec3::ZERO;
        pos.x = self.data & 31;
        pos.y = (self.data >> 5) & 31;
        pos.z = (self.data >> 10) & 31;
        pos
    }
    pub fn set_size(&mut self, index: UVec2) {
        let new: u32 = index.x | (index.y << 5);
        self.data = self.data & !(1023 << 15) | (new << 15);
    }
    pub fn get_size(&self) -> UVec2 {
        let mut pos: UVec2 = UVec2::ZERO;
        pos.x = (self.data >> 10) & 31;
        pos.y = (self.data >> 15) & 31;
        pos
    }
    pub fn set_dir(&mut self, dir: Direction) {
        self.data = self.data & !(7 << 25) | ((dir as u32) << 25);
    }
    pub fn get_dir(&self) -> Direction {
        let val = ((self.data >> 25) & 7) as u8;
        unsafe { std::mem::transmute(val) }
    }
    pub fn set_block_type(&mut self, block: BlockID) {
        self.data = self.data & !(15 << 28) | ((block as u32) << 28);
    }
    pub fn get_block_type(&self) -> BlockID {
        let val = ((self.data >> 28) & 15) as u8;
        unsafe { std::mem::transmute(val) }
    }
}

/// the voxel pipeline iterates through each VoxelMesh
/// and sends each visible side to the gpu to be rendered
#[derive(Debug,Component,Clone)]
pub struct VoxelMesh {
    pub chunk_pos: IVec3,
    pub quads: [Vec<Quad>; 6],
}
impl VoxelMesh {
    pub fn new(chunk_pos: IVec3) -> Self{
        Self {
            chunk_pos,
            quads: from_fn(|_index| {
                Vec::new()
            })
        }
    }
    pub fn get_block(data: [[[&Chunk; 3]; 3]; 3], mut block_index: IVec3) -> BlockID {
        let mut chunk_index: UVec3 = UVec3::new(1,1,1);
        chunk_index.x -= (block_index.x < 0) as u32;
        chunk_index.y -= (block_index.y < 0) as u32;
        chunk_index.z -= (block_index.z < 0) as u32;

        chunk_index.x += (block_index.x > 15) as u32;
        chunk_index.y += (block_index.y > 15) as u32;
        chunk_index.z += (block_index.z > 15) as u32;

        block_index.x &= 15;
        block_index.y &= 15;
        block_index.z &= 15;
        data[chunk_index.x as usize]
            [chunk_index.y as usize]
            [chunk_index.z as usize]
            .data[block_index.x as usize]
                [block_index.y as usize]
                [block_index.z as usize]
    }
}

/// Contains one side of a VoxelMesh, which can be sent directly to a GPU
#[derive(Debug,Clone)]
pub struct VoxelMeshToGPU {
    pub chunk_pos: [i32; 3],
    pub orientation: u32,
    pub quads: Vec<Quad>,
}