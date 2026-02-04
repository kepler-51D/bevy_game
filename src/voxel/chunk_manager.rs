use bevy::{asset::RenderAssetUsages, mesh::{Indices, PrimitiveTopology}, platform::collections::HashMap, prelude::*};
use crate::{player::camera::{Player}, voxel::voxel::{BlockID, Chunk, NeedsMeshUpdate, Renderable}};

#[derive(Resource,Clone)]
pub struct ChunkManager {
    pub map: HashMap<IVec3,Entity>,
    pub render_distance_hor: i32,
    pub render_distance_ver: i32,
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
    pub fn gen_mesh(
        chunk: &Chunk,
        manager: &ChunkManager, 
        chunk_query: &Query<&Chunk>
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

                    if manager.get_block(world_pos + IVec3::NEG_X, chunk_query) == BlockID::Air {
                        ChunkManager::add_quad(mesh_offset, Chunk::LEFTQUAD, &mut vertices, &mut indices);
                    }
                    if manager.get_block(world_pos + IVec3::X, chunk_query) == BlockID::Air {
                        ChunkManager::add_quad(mesh_offset, Chunk::RIGHTQUAD, &mut vertices, &mut indices);
                    }
                    if manager.get_block(world_pos + IVec3::Y, chunk_query) == BlockID::Air {
                        ChunkManager::add_quad(mesh_offset, Chunk::TOPQUAD, &mut vertices, &mut indices);
                    }
                    if manager.get_block(world_pos + IVec3::NEG_Y, chunk_query) == BlockID::Air {
                        ChunkManager::add_quad(mesh_offset, Chunk::BOTTOMQUAD, &mut vertices, &mut indices);
                    }
                    if manager.get_block(world_pos + IVec3::Z, chunk_query) == BlockID::Air {
                        ChunkManager::add_quad(mesh_offset, Chunk::FRONTQUAD, &mut vertices, &mut indices);
                    }
                    if manager.get_block(world_pos + IVec3::NEG_Z, chunk_query) == BlockID::Air {
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
        chunk_query: &Query<&Chunk>
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

        // 3. Look up the entity in the map
        if let Some(&entity) = self.map.get(&chunk_pos) {
            // 4. Look up the component data for that entity
            if let Ok(chunk) = chunk_query.get(entity) {
                return chunk.data[local_pos.x as usize][local_pos.y as usize][local_pos.z as usize];
            }
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
        let material_handle = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        });
        let chunk_entity = commands.spawn((
            chunk,
            Transform {
                translation: Vec3::new(
                    key.x as f32 * Chunk::CHUNKSIZE as f32 / 2.0,
                    key.y as f32 * Chunk::CHUNKSIZE as f32 / 2.0,
                    key.z as f32 * Chunk::CHUNKSIZE as f32 / 2.0,
                ),
                rotation: Quat::default(),
                scale: Vec3::new(1.0,1.0,1.0),
            },
            NeedsMeshUpdate,
            MeshMaterial3d(material_handle),
        )).id();

        self.map.insert(key, chunk_entity);
    }
}

pub fn process_chunks(
    mut commands: Commands,
    // mut params: ParamSet<(Query<(Entity, &mut Chunk), With<NeedsMeshUpdate>>,Query<&Chunk>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    chunk_manager: Res<ChunkManager>,
    all_chunks: Query<&Chunk>,
    mut dirty_chunks: Query<(Entity, &Chunk), With<NeedsMeshUpdate>>
) {
    for (entity, chunk) in dirty_chunks.iter_mut() {
        let new_mesh = ChunkManager::gen_mesh(
            &chunk,
            &chunk_manager,
            // &params.p1(),
            &all_chunks,
        );
        
        let mesh_handle = meshes.add(new_mesh);
        
        commands.entity(entity).insert(Mesh3d(mesh_handle));
        
        // chunk.state = ChunkState::Renderable;
        commands.entity(entity).remove::<NeedsMeshUpdate>();
        // commands.entity(entity).remove::<NeedsMeshUpdate>();
        commands.entity(entity).insert(Renderable);
    }

}

pub fn manage_chunks(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player: Query<&Transform, With<Player>>
) {
    for (transform) in player {
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
                        let mut data = [[[BlockID::Air; 32]; 32]; 32];
                        let mut toggle: bool = true;
                        for x in 0..16 {
                            for y in 0..16 {
                                for z in 0..16 {
                                    data[x][y][z] = if toggle {
                                        BlockID::Stone
                                    } else {
                                        BlockID::Air
                                    };
                                    toggle = !toggle;
                                }
                            }
                        }
                        chunk_manager.add_chunk(&mut commands, Chunk {data, pos: index}, &mut materials);
                    }
                }
            }
        }
    }

    // for x in 
}