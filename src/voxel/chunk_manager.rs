use std::sync::Arc;

use bevy::{
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
        self.map.insert(key, Arc::clone(&chunk.data));
        let material_handle = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        });
        commands.spawn((
            chunk,
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
                        let mut data = [[[BlockID::Air; 32]; 32]; 32];
                        let mut toggle: bool = true;
                        for x in data.iter_mut() {
                            for y in x.iter_mut() {
                                for z in y.iter_mut() {
                                    if index.y <= 0 {
                                        *z = BlockID::Stone;
                                    } else {
                                        *z = BlockID::Air;
                                    }
                                    // *z = if toggle {
                                    //     BlockID::Stone
                                    // } else {
                                    //     BlockID::Air
                                    // };
                                    // toggle = !toggle;
                                }
                            }
                        }
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