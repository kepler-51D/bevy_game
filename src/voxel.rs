use bevy::{asset::RenderAssetUsages, ecs::{component::Component, system::{Commands, Query}}, math::{IVec3, UVec3, Vec3}, mesh::{Indices, Mesh, PrimitiveTopology}, platform::collections::HashMap};

#[repr(u16)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BlockID {
    Air,
    Stone,
    Block
}
#[repr(u8)]
pub enum ChunkState {
    Renderable,
    MeshRendering,
    MeshDirty,
    UnLoaded,
}

#[derive(Component)]
pub struct Position(IVec3);

#[derive(Component)]
pub struct Chunk {
    pub data: [
        [
            [
                BlockID;Chunk::CHUNKSIZE
            ];Chunk::CHUNKSIZE
        ];Chunk::CHUNKSIZE
    ],
    pub model: Mesh,
    pub state: ChunkState,
}
impl Chunk {
    pub const CHUNKSIZE: usize = 32;
    pub const TOPQUAD: [Vec3; 4] = [
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        Vec3 {x: 1.0, y: 1.0, z: 0.0},
        Vec3 {x: 0.0, y: 1.0, z: 1.0},
        Vec3 {x: 1.0, y: 1.0, z: 1.0},
    ];
    pub const BOTTOMQUAD: [Vec3; 4] = [
        Vec3 {x: 0.0, y: 0.0, z: 0.0},
        Vec3 {x: 0.0, y: 0.0, z: 1.0},
        Vec3 {x: 1.0, y: 0.0, z: 0.0},
        Vec3 {x: 1.0, y: 0.0, z: 1.0},
    ];
    pub const LEFTQUAD: [Vec3; 4] = [
        Vec3 {x: 0.0, y: 0.0, z: 0.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        Vec3 {x: 0.0, y: 0.0, z: 1.0},
        Vec3 {x: 0.0, y: 1.0, z: 1.0},
    ];
    pub const RIGHTQUAD: [Vec3; 4] = [
        Vec3 {x: 1.0, y: 0.0, z: 0.0},
        Vec3 {x: 1.0, y: 0.0, z: 1.0},
        Vec3 {x: 1.0, y: 1.0, z: 0.0},
        Vec3 {x: 1.0, y: 1.0, z: 1.0},
    ];
    pub const FRONTQUAD: [Vec3; 4] = [
        Vec3 {x: 0.0, y: 0.0, z: 1.0},
        Vec3 {x: 0.0, y: 1.0, z: 1.0},
        Vec3 {x: 1.0, y: 0.0, z: 1.0},
        Vec3 {x: 1.0, y: 1.0, z: 1.0},
    ];
    pub const BACKQUAD: [Vec3; 4] = [
        Vec3 {x: 1.0, y: 1.0, z: 0.0},
        Vec3 {x: 1.0, y: 0.0, z: 0.0},
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        Vec3 {x: 0.0, y: 0.0, z: 0.0},
    ];
}
pub struct ChunkManager {
    pub map: HashMap<IVec3,Chunk>,
}
impl ChunkManager {
    pub fn add_quad(offset: Vec3, quad: [Vec3; 4], vertices: &mut Vec<Vec3>, indices: &mut Vec<u32>) {
        let mut working_quad = quad;
        for i in 0..4 {
            working_quad[i] += offset;
        }
        let s = vertices.len() as u32;
        indices.extend_from_slice(&[s+0,s+3,s+1,s+0,s+2,s+3]);
        vertices.extend_from_slice(&working_quad);
    }
    pub fn gen_mesh(&mut self, pos: IVec3) -> Mesh {
        let chunk = match self.map.get_mut(&pos) {
            Some(val) => {val},
            None => {todo!()}
        };
        let chunk_data = chunk.data;
        
        let mut vertices: Vec<Vec3> = Vec::new();
        // let mut normals: Vec<Vec3> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        
        chunk.state = ChunkState::MeshRendering;
        for x in 0..Chunk::CHUNKSIZE {
            for y in 0..Chunk::CHUNKSIZE {
                for z in 0..Chunk::CHUNKSIZE {
                    let offset = Vec3 {
                        x: x as f32 + pos.x as f32,
                        y: y as f32 + pos.y as f32,
                        z: z as f32 + pos.z as f32,
                    };
                    if chunk_data[x][y][z] != BlockID::Air {
                        if self.get_block(IVec3 {
                            x:x as i32 -1,
                            y:y as i32,
                            z:z as i32
                        }) == BlockID::Air {
                            ChunkManager::add_quad(
                                offset, Chunk::LEFTQUAD, &mut vertices, &mut indices
                            );
                        }
                        if self.get_block(IVec3 {
                            x:x as i32 +1,
                            y:y as i32,
                            z:z as i32
                        }) == BlockID::Air {
                            ChunkManager::add_quad(
                                offset, Chunk::RIGHTQUAD, &mut vertices, &mut indices
                            );
                        }
                        if self.get_block(IVec3 {
                            x:x as i32,
                            y:y as i32 -1,
                            z:z as i32
                        }) == BlockID::Air {
                            ChunkManager::add_quad(
                                offset, Chunk::BOTTOMQUAD, &mut vertices, &mut indices
                            );
                        }
                        if self.get_block(IVec3 {
                            x:x as i32,
                            y:y as i32 +1,
                            z:z as i32
                        }) == BlockID::Air {
                            ChunkManager::add_quad(
                                offset, Chunk::TOPQUAD, &mut vertices, &mut indices
                            );
                        }
                        if self.get_block(IVec3 {
                            x:x as i32,
                            y:y as i32,
                            z:z as i32 -1
                        }) == BlockID::Air {
                            ChunkManager::add_quad(
                                offset, Chunk::BACKQUAD, &mut vertices, &mut indices
                            );
                        }
                        if self.get_block(IVec3 {
                            x:x as i32,
                            y:y as i32,
                            z:z as i32 +1
                        }) == BlockID::Air {
                            ChunkManager::add_quad(
                                offset, Chunk::FRONTQUAD, &mut vertices, &mut indices
                            );
                        }
                    }
                }
            }
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_indices(Indices::U32(indices));
        mesh
    }

    pub fn get_block(&self, pos: IVec3) -> BlockID {
        let chunk_index: IVec3 = IVec3 {
            x: pos.x / Chunk::CHUNKSIZE as i32,
            y: pos.y / Chunk::CHUNKSIZE as i32,
            z: pos.z / Chunk::CHUNKSIZE as i32,
        };
        let block_index: IVec3 = IVec3 {
            x: pos.x % Chunk::CHUNKSIZE as i32,
            y: pos.y % Chunk::CHUNKSIZE as i32,
            z: pos.z % Chunk::CHUNKSIZE as i32,
        };
        let chunk = self.map.get(&chunk_index);
        match chunk {
            Some(val) => {
                val.data[block_index.x as usize][block_index.y as usize][block_index.z as usize]
            },
            None => {
                BlockID::Air
            }
        }
    }
    pub fn get_block_from_index(&self, chunk_index: IVec3, block_index: UVec3) -> BlockID {
        let chunk = self.map.get(&chunk_index);
        match chunk {
            Some(val) => {
                val.data[block_index.x as usize][block_index.y as usize][block_index.z as usize]
            },
            None => {
                BlockID::Air
            }
        }
    }
}

pub fn process_chunks(mut commands: Commands, query: Query<&mut Chunk>) {
    for mut chunk in query {
        match chunk.state {
            ChunkState::MeshDirty => {
                todo!();
                chunk.state = ChunkState::Renderable;
            }
            ChunkState::MeshRendering => {
                continue;
            }
            ChunkState::UnLoaded => {
                todo!();
                chunk.state = ChunkState::MeshDirty;
            }
            ChunkState::Renderable => {
                todo!() // will deal with after rendering works
            }
        }
    }
}