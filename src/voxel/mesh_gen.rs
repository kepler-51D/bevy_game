use std::array::from_fn;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use bevy::{
    asset::{RenderAssetUsages},
    math::{IVec3, Vec3},
    mesh::{Indices, Mesh, PrimitiveTopology},
};
use crate::voxel::{
    chunk_manager::ChunkManager,
    voxel_shaders::{Quad, VoxelMaterial},
    voxel_types::{BlockID, Chunk},
};

#[repr(u8)]
#[derive(Debug, EnumIter,Clone, Copy)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}
pub const DIRECTION: [IVec3; 6] = [
    IVec3::Y,
    IVec3::NEG_Y,
    IVec3::X,
    IVec3::NEG_X,
    IVec3::Z,
    IVec3::NEG_Z,
];

pub struct VoxelMesh {
    pub offset: Vec3,
    pub sides: [VoxelMaterial; 6],
}
impl VoxelMesh {
    pub fn new() -> Self {
        Self {
            offset: Vec3::ZERO,
            sides: from_fn(VoxelMaterial::arr_init),
        }
    }
    pub fn get_mesh(&self, side: Side) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleStrip,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
        ).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            Vec::from(Chunk::QUADS[side as usize])
        )
    }
    pub fn gen_mesh(chunk_index: IVec3, chunk_manager: &ChunkManager) -> Self {
        let mut return_val = VoxelMesh::new();
        for x in 0..Chunk::CHUNKSIZE {
            for y in 0..Chunk::CHUNKSIZE {
                for z in 0..Chunk::CHUNKSIZE {
                    let index = IVec3::new(x as i32,y as i32,z as i32);
                    let block_pos = index + chunk_index;
                    if chunk_manager.get_block(block_pos) == BlockID::Air {continue;}
                    
                    for direction in Side::iter() {
                        if chunk_manager.get_block(block_pos + DIRECTION[direction as usize]) != BlockID::Air {
                            return_val.sides[direction as usize].push_quad(
                                Quad::from(
                                    Vec3::new(x as f32, y as f32, z as f32),
                                    1,
                                    1
                                )
                            );
                        }
                    }
                }
            }
        }
        return_val
    }
}

impl ChunkManager {
    pub fn add_quad(offset: Vec3, quad: [Vec3; 4], vertices: &mut Vec<Vec3>, indices: &mut Vec<u32>) {
        let mut working_quad = quad;
        for vert in working_quad.iter_mut() {
            *vert += offset;
        }
        let s = vertices.len() as u32;
        indices.extend_from_slice(&[s,s+3,s+1,s,s+2,s+3]);
        vertices.extend_from_slice(&working_quad);
    }
    pub fn gen_mesh(
        &self,
        chunk: &Chunk,
    ) -> Mesh {
        let mut vertices: Vec<Vec3> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        
        let world_offset = chunk.pos * Chunk::CHUNKSIZE as i32;

        for x in 0..Chunk::CHUNKSIZE {
            for y in 0..Chunk::CHUNKSIZE {
                for z in 0..Chunk::CHUNKSIZE {
                    if chunk.data[x][y][z] == BlockID::Air { continue; }

                    let local_pos = IVec3::new(
                        x as i32 - Chunk::CHUNKSIZE as i32 / 2,
                        y as i32 - Chunk::CHUNKSIZE as i32 / 2,
                        z as i32 - Chunk::CHUNKSIZE as i32 / 2,
                    );
                    let mesh_offset = local_pos.as_vec3();
                    
                    let world_pos = world_offset + local_pos;

                    if self.get_block(world_pos + IVec3::NEG_X) == BlockID::Air {
                        ChunkManager::add_quad(mesh_offset, Chunk::LEFTQUAD, &mut vertices, &mut indices);
                    }
                    if self.get_block(world_pos + IVec3::X) == BlockID::Air {
                        ChunkManager::add_quad(mesh_offset, Chunk::RIGHTQUAD, &mut vertices, &mut indices);
                    }
                    if self.get_block(world_pos + IVec3::Y) == BlockID::Air {
                        ChunkManager::add_quad(mesh_offset, Chunk::TOPQUAD, &mut vertices, &mut indices);
                    }
                    if self.get_block(world_pos + IVec3::NEG_Y) == BlockID::Air {
                        ChunkManager::add_quad(mesh_offset, Chunk::BOTTOMQUAD, &mut vertices, &mut indices);
                    }
                    if self.get_block(world_pos + IVec3::Z) == BlockID::Air {
                        ChunkManager::add_quad(mesh_offset, Chunk::FRONTQUAD, &mut vertices, &mut indices);
                    }
                    if self.get_block(world_pos + IVec3::NEG_Z) == BlockID::Air {
                        ChunkManager::add_quad(mesh_offset, Chunk::BACKQUAD, &mut vertices, &mut indices);
                    }
                }
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_indices(Indices::U32(indices));
        mesh
    }
}