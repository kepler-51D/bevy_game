use std::{array::from_fn, sync::Arc};
use std::simd::{u8x16, u8x64, u32x4};

use bevy::{
    ecs::component::Component,
    math::{IVec3, UVec3},
};
use strum_macros::EnumIter;

use crate::fast_voxels::blocks::BlockID;
use crate::fast_voxels::mesh_gen::ChunkBitMask;

pub type BlockData = Arc<[[[BlockID;CHUNKSIZE];CHUNKSIZE];CHUNKSIZE]>;

/// defines the chunk size, by the length of one side of the chunk
pub const CHUNKSIZE: usize = 32;
pub const FAST_CHUNKSIZE: usize = 30;


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
const LEN: usize = (FAST_CHUNKSIZE+2)*(FAST_CHUNKSIZE+2)*(FAST_CHUNKSIZE+2)/2;

/// each u32x4 is one row
pub type FastBlockData = [u8; LEN];
/// same as the Chunk type but uses simd to make it faster
#[derive(Debug, Clone, Copy, Component)]
pub struct FastChunk { // presumes each block is 4 bits
    pub pos: IVec3,
    pub data: FastBlockData,
}
impl FastChunk {
    pub fn return_mask(&self, block: BlockID) -> ChunkBitMask {
        let ret: ChunkBitMask = ChunkBitMask {
            data: [0; (FAST_CHUNKSIZE+2)*(FAST_CHUNKSIZE+2)],
        };

        for (index, block) in ret.data.iter().enumerate() {
            
        }
        ret
    }
    pub fn new(pos: IVec3, data: FastBlockData) -> Self {
        Self {
            pos,
            data,
        }
    }
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