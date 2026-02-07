use std::sync::Arc;

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    platform::collections::HashMap,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures_lite::future}
};
use crate::{player::camera::Player, voxel::voxel_types::{BlockData, BlockID, Chunk, NeedsMeshUpdate, Renderable}};


#[derive(Component)]
pub struct GenMesh(Task<(Entity, Mesh)>);

#[derive(Resource,Clone)]
pub struct ChunkManager {
    // pub map: HashMap<IVec3,Entity>,
    pub map: HashMap<IVec3,BlockData>,
    pub render_distance_hor: i32,
    pub render_distance_ver: i32,
}
impl ChunkManager {
    pub fn add_quad(offset: Vec3, quad: [Vec3; 4], vertices: &mut Vec<Vec3>, indices: &mut Vec<u32>) {
        let mut working_quad = quad;
        // for i in 0..4 {
        //     working_quad[i] += offset;
        // }
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
                        z as i32 - Chunk::CHUNKSIZE as i32 / 2
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

    pub fn get_block(
        &self, 
        pos: IVec3,
    ) -> BlockID {
        let chunk_pos = IVec3::new(
            pos.x.div_euclid(32), 
            pos.y.div_euclid(32), 
            pos.z.div_euclid(32)
        );
        
        let local_pos = IVec3::new(
            pos.x.rem_euclid(32),
            pos.y.rem_euclid(32),
            pos.z.rem_euclid(32)
        );

        if let Some(data) = self.map.get(&chunk_pos) {
            return data[local_pos.x as usize][local_pos.y as usize][local_pos.z as usize];
            // if let Ok(chunk) = chunk_query.get(entity) {
            //     return chunk.data[local_pos.x as usize][local_pos.y as usize][local_pos.z as usize];
            // }
        }
        BlockID::Air
    }
    pub fn add_chunk(
        &mut self,
        commands: &mut Commands,
        chunk: Chunk,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let key = chunk.pos;
        
        // Store the data pointer in our manager for neighbors to find
        self.map.insert(key, Arc::clone(&chunk.data));

        let material_handle = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        });

        commands.spawn((
            chunk,
            // Multiply by 32.0 (Chunk size) to space them out in 3D space
            Transform::from_translation(key.as_vec3() * Chunk::CHUNKSIZE as f32 / 1.0),
            MeshMaterial3d(material_handle),
            NeedsMeshUpdate,
        ));
    }
}

pub fn process_chunks(
    mut commands: Commands,
    chunk_manager: Res<ChunkManager>,
    dirty_chunks: Query<(Entity, &Chunk), With<NeedsMeshUpdate>>
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for (entity, chunk) in dirty_chunks.iter() {
        let data = Arc::clone(&chunk.data);
        let pos = chunk.pos;
        let map = chunk_manager.clone();
        let task = thread_pool.spawn(async move {
            let new_mesh = map.gen_mesh(
                &Chunk {
                    data,
                    pos,
                },
            );
            
            (entity, new_mesh)
        });

        commands.entity(entity)
            .insert(GenMesh(task))
            .remove::<NeedsMeshUpdate>();
    }
}

pub fn manage_chunks(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player: Query<&Transform, With<Player>>
) {
    for transform in player {
        let center: IVec3 = IVec3::new(
            transform.translation.x as i32 / Chunk::CHUNKSIZE as i32 *2,
            transform.translation.y as i32 / Chunk::CHUNKSIZE as i32 *2,
            transform.translation.z as i32 / Chunk::CHUNKSIZE as i32 *2,
        );
        let start: IVec3 = IVec3::new(
            center.x-chunk_manager.render_distance_hor,
            center.y-chunk_manager.render_distance_ver,
            center.z-chunk_manager.render_distance_hor,
        );
        let end: IVec3 = IVec3::new(
            center.x+chunk_manager.render_distance_hor,
            center.y+chunk_manager.render_distance_ver,
            center.z+chunk_manager.render_distance_hor,
        );
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                for z in start.z..=end.z {
                    let index: IVec3 = IVec3::new(x,y,z);
                    let exists: bool = chunk_manager.map.contains_key(&index);
                    if !exists {
                        // jobs.push(index);
                        let mut data = [[[BlockID::Air; 32]; 32]; 32];
                        let mut toggle: bool = true;
                        for x in data.iter_mut() {
                            for y in x.iter_mut() {
                                for z in y.iter_mut() {
                                    *z = if toggle {
                                        BlockID::Stone
                                    } else {
                                        BlockID::Air
                                    };
                                    toggle = !toggle;
                                }
                            }
                        }
                        // for x in 0..Chunk::CHUNKSIZE {
                        //     for y in 0..Chunk::CHUNKSIZE {
                        //         for z in 0..Chunk::CHUNKSIZE {
                        //             data[x][y][z] = if toggle {
                        //                 BlockID::Stone
                        //             } else {
                        //                 BlockID::Air
                        //             };
                        //             toggle = !toggle;
                        //         }
                        //     }
                        // }
                        chunk_manager.add_chunk(&mut commands, Chunk {data:Arc::new(data), pos: index}, &mut materials);
                    }
                }
            }
        }
    }
}

pub fn poll_mesh_tasks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tasks: Query<(Entity, &mut GenMesh)>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some((target_entity, new_mesh)) = future::block_on(future::poll_once(&mut task.0)) {
            let mesh_handle = meshes.add(new_mesh);
            
            if let Ok(mut entity_cmds) = commands.get_entity(target_entity) {
                entity_cmds.insert(Mesh3d(mesh_handle));
                entity_cmds.insert(Renderable);
            }

            commands.entity(entity).remove::<GenMesh>();
        }
    }
}