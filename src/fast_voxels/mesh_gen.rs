use bevy::math::{IVec3, UVec3};
use strum::IntoEnumIterator;

use crate::fast_voxels::base_types::{BlockID, CHUNKSIZE, Chunk, DIRECTION_VECS, Direction, Quad, VoxelMesh};

impl VoxelMesh {
    pub fn gen_greedy_mesh(chunk_pos: IVec3, data: [[[&Chunk;3];3];3]) -> Self {
        let mut return_val = VoxelMesh::new(chunk_pos);

        


        return_val
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
                    if current_block != BlockID::Air {
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
        }

        return_val
    }
}