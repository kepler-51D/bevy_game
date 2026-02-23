mod voxel;
mod player;
// mod fast_voxels;

use crate::player::camera::{grab_mouse, spawn_player, update_player};
use crate::voxel::chunk_manager::{ChunkManager, manage_chunks, poll_mesh_tasks, process_chunks};

use bevy::platform::collections::HashMap;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        .add_systems(Startup, setup)
        .add_systems(Update,grab_mouse)
        
        .add_systems(Startup, spawn_player)
        .add_systems(Update, update_player)

        .insert_resource(ChunkManager { map: HashMap::default(), render_distance_hor: 2, render_distance_ver: 1})
        
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
    // let mut data = [[[BlockID::Air; 32]; 32]; 32];
    // let mut toggle: bool = true;
    // for x in 0..32 {
    //     for y in 0..32 {
    //         for z in 0..32 {
    //             if y < 16;
    //         }
    //     }
    // }
    // chunk_manager.add_chunk(&mut commands, Chunk {data:Arc::new(data), pos: IVec3::ZERO}, &mut materials);
}
