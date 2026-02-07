use bevy::{asset::RenderAssetUsages, math::{IVec3, Vec3}, mesh::{Indices, Mesh, PrimitiveTopology}};
use crate::voxel::{chunk_manager::ChunkManager, voxel_types::{BlockID, Chunk}};

pub enum Orientation {
    Up,Down,Left,Right,Front,Back
}

pub struct Quad {
    pub orientation: Orientation,
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