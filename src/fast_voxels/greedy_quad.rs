use bevy::math::{UVec2, UVec3};

use crate::fast_voxels::{base_types::Direction, blocks::GPUBlockID};

/// lowest 5 bits are z, next 5 bits are y, next 5 bits are x
/// (labelled X, Y and Z)
/// 
/// next 5 bits are for the width of the quad, next 5 bits are for height of quad
/// (labelled W and H)
/// 
/// next 3 bits are direction of quad
/// (Labelled D)
/// 
/// last 4 bits are blockID
/// (labelled B)
/// 
/// this is what it looks like:
/// 
/// BBBBDDDWWWWWHHHHHXXXXXYYYYYZZZZZ
/// 
/// (seperated for readability)
/// 
/// BBBB_DDD_WWWWW_HHHHH_XXXXX_YYYYY_ZZZZZ
pub struct GreedyQuad {
    pub data: u32
}
impl GreedyQuad {
    pub fn set_pos(&mut self, pos: UVec3) {
        let new: u32 = pos.x | (pos.y << 5) | (pos.z << 10);
        self.data = (self.data & !0b11_111) | new;
    }
    pub fn get_pos(&self) -> UVec3 {
        let mut pos: UVec3 = UVec3::ZERO;
        pos.x = self.data & 0b11_111;
        pos.y = (self.data >> 5) & 0b11_111;
        pos.z = (self.data >> 10) & 0b11_111;
        pos
    }
    pub fn set_size(&mut self, index: UVec2) {
        let new: u32 = index.x | (index.y << 5);
        self.data = self.data & !(0b1_111_111_111 << 15) | (new << 15);
    }
    pub fn get_size(&self) -> UVec2 {
        let mut pos: UVec2 = UVec2::ZERO;
        pos.x = (self.data >> 10) & 0b11_111;
        pos.y = (self.data >> 15) & 0b11_111;
        pos
    }
    pub fn set_dir(&mut self, dir: Direction) {
        self.data = self.data & !(0b111 << 25) | ((dir as u32) << 25);
    }
    pub fn get_dir(&self) -> Direction {
        let val = ((self.data >> 25) & 0b111) as u8;
        unsafe { std::mem::transmute(val) }
    }
    pub fn set_block_type(&mut self, block: GPUBlockID) {
        self.data = self.data & !(16 << 28) | ((block as u32) << 28);
    }
    pub fn get_block_type(&self) -> GPUBlockID {
        let val = ((self.data >> 28) & 0b1_111) as u8;
        unsafe { std::mem::transmute(val) }
    }
}