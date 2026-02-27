use bevy::math::{IVec3, UVec3};
use strum::IntoEnumIterator;

use crate::fast_voxels::{
    base_types::{CHUNKSIZE, Chunk, DIRECTION_VECS, Direction, FAST_CHUNKSIZE, FastChunk, Quad, VoxelMesh}, blocks::BlockID
};

pub struct ChunkBitMaskRow {
    pub data: u32,
}
/// a mask for a specific block for a slice. so it is either the block
/// it masks for, or not the block it masks for.
/// 
/// each bit
pub struct ChunkBitMaskSlice {
    pub data: [u32; 32],
}
impl ChunkBitMaskSlice {
    fn process(&self) -> Vec<(u32,u32)> {
        for row in self.data {
            let mut count_shift: u32 = 0;
            let mut row_copy: u32 = row;
            
            while row_copy != 0  || count_shift >= 32 {
                count_shift += row.leading_zeros();
                row_copy <<= row.leading_zeros();
            }
        }
        todo!()
    }
}
pub struct ChunkBitMask {
    pub data: [u32; (FAST_CHUNKSIZE+2)*(FAST_CHUNKSIZE+2)]
}
impl ChunkBitMask {
    pub fn new() -> Self {
        Self {
            data: [0; (FAST_CHUNKSIZE+2)*(FAST_CHUNKSIZE+2)]
        }
    }
}
impl VoxelMesh {
    pub fn gen_greedy_mesh(chunk_pos: IVec3, data: &FastChunk) -> Self {
        let mut return_val = VoxelMesh::new(chunk_pos);
        for block in BlockID::iter() {
            // get mask for each block
            let mask: ChunkBitMask = ChunkBitMask::new();
            for row in mask.data {
                let mask = row ^ (row << 1);
                
            }
        }
        return_val
    }
    pub fn gen_mesh(chunk_pos: IVec3, data: [[[&Chunk; 3]; 3]; 3]) -> Self {
        let mut return_val: VoxelMesh = VoxelMesh::new(chunk_pos);
        for x in 0..CHUNKSIZE as u32 {
            for y in 0..CHUNKSIZE as u32 {
                for z in 0..CHUNKSIZE as u32 {
                    let current_block = data[1][1][1].data[x as usize][y as usize][z as usize];
                    if current_block == BlockID::Air {continue;}
                    let block_index = IVec3::new(
                        x as i32,
                        y as i32,
                        z as i32,
                    );
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
}