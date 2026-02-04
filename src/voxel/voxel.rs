use bevy::{
    ecs::{component::Component},
    math::{IVec3, Vec3},
};

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
    pub data: [
        [
            [
                BlockID;Chunk::CHUNKSIZE
            ];Chunk::CHUNKSIZE
        ];Chunk::CHUNKSIZE
    ],
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
}