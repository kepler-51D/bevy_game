use std::{sync::Arc};

use bevy::{
    ecs::{component::Component},
    math::{IVec3, Vec3},
};

pub type BlockData = Arc<[[[BlockID;Chunk::CHUNKSIZE];Chunk::CHUNKSIZE];Chunk::CHUNKSIZE]>;

#[repr(u16)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BlockID {
    Air,
    Stone,
}
#[derive(Component)]
pub struct NeedsMeshUpdate;
// #[derive(Component)]
// pub struct NeedsLoading;
#[derive(Component)]
pub struct Renderable;

#[derive(Component)]
pub struct Chunk {
    pub data: BlockData,
    pub pos: IVec3,
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
        Vec3 {x: 0.0, y: 1.0, z: 0.0},
        Vec3 {x: 1.0, y: 0.0, z: 0.0},
        Vec3 {x: 0.0, y: 0.0, z: 0.0},
    ];
    pub const QUADS: [[Vec3; 4]; 6] = [
        Chunk::TOPQUAD,
        Chunk::BOTTOMQUAD,
        Chunk::LEFTQUAD,
        Chunk::RIGHTQUAD,
        Chunk::FRONTQUAD,
        Chunk::BACKQUAD,
    ];
}