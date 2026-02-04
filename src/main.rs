mod voxel;
mod player;
use crate::player::camera::{grab_mouse, spawn_player, update_player};
use crate::voxel::chunk_manager::{ChunkManager, manage_chunks, process_chunks};
use crate::voxel::voxel::{BlockID, Chunk};

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
        .add_systems(Update, process_chunks)
        .add_systems(Update,manage_chunks)
        .run();
}



fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {

    // let material_handle = materials.add(StandardMaterial {
    //     base_color: Color::srgb(1.0, 1.0, 1.0),
    //     ..default()
    // });

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
    chunk_manager.add_chunk(&mut commands, Chunk {data, pos: IVec3::ZERO}, &mut materials);
    // let chunk_pos = IVec3::ZERO;
    // let chunk_entity = commands.spawn((
    //     Chunk {
    //         data,
    //         pos: chunk_pos,
    //     },
    //     NeedsMeshUpdate,
    //     MeshMaterial3d(material_handle),
    // )).id();

    // chunk_manager.map.insert(chunk_pos, chunk_entity);
}