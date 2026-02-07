mod voxel;
mod player;
use std::sync::Arc;

use crate::player::camera::{grab_mouse, spawn_player, update_player};
use crate::voxel::chunk_manager::{ChunkManager, manage_chunks, poll_mesh_tasks, process_chunks};
use crate::voxel::voxel_types::{BlockID, Chunk};

use bevy::platform::collections::HashMap;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update,grab_mouse)
        
        .add_systems(Startup, spawn_player)
        .add_systems(Update, update_player)

        // .add_systems(Update, poll_mesh_tasks)
        
        .insert_resource(ChunkManager { map: HashMap::default(), render_distance_hor: 2, render_distance_ver: 1})
        // .add_systems(Update, process_chunks)
        // .add_systems(Update,manage_chunks)
        .add_systems(Update, (
            manage_chunks,
            process_chunks,
            poll_mesh_tasks,
        ).chain())
        .run();
}



fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
//     let thread_pool = AsyncComputeTaskPool::get();
    
    // let task = thread_pool.spawn(async move {
    //     let mesh = ChunkManager::gen_mesh(
    //         todo!(),
    //         todo!(),
    //         todo!(),
    //     );
    // });

    // commands.spawn(GenMesh(task));
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
    chunk_manager.add_chunk(&mut commands, Chunk {data:Arc::new(data), pos: IVec3::ZERO}, &mut materials);
}