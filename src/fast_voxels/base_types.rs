use std::{array::from_fn, sync::Arc};

use bevy::{
    ecs::component::Component,
    math::{IVec3, UVec3},
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type BlockData = Arc<[[[BlockID;CHUNKSIZE];CHUNKSIZE];CHUNKSIZE]>;

#[repr(u16)]
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
#[repr(u32)]
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
#[derive(Component,Debug)]
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

/// the voxel pipeline iterates through each VoxelMesh
/// and sends each visible side to the gpu to be rendered
/// 
/// note: if the camera is outside the chunk, only 3 sides will have to be rendered.
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
    pub fn gen_mesh(chunk_pos: IVec3, data: [[[&Chunk; 3]; 3]; 3]) -> Self {
        let mut return_val: VoxelMesh = VoxelMesh::new(chunk_pos);
        for x in 0..CHUNKSIZE {
            for y in 0..CHUNKSIZE {
                for z in 0..CHUNKSIZE {
                    let block_index = IVec3::new(
                        x as i32,
                        y as i32,
                        z as i32,
                    );
                    let current_block = data[1][1][1].data[x][y][z];
                    for i in Direction::iter() {
                        if VoxelMesh::get_block(
                            data,
                            block_index + DIRECTION_VECS[i as usize])
                            != BlockID::Air
                        {
                            return_val.quads[i as usize].push(
                                Quad::new(UVec3::new(
                                    x as u32,
                                    y as u32,
                                    z as u32,
                                ))
                            );
                        }
                    }
                }
            }
        }

        return_val
    }
    fn get_block(data: [[[&Chunk; 3]; 3]; 3], mut block_index: IVec3) -> BlockID {
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
            .
            data[block_index.x as usize]
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