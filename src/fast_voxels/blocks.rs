use strum_macros::EnumIter;


/// an enum for all the possible blocks.
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, EnumIter)]
pub enum BlockID {
    Water = 0,
    Steam = 1,
    Ground = 2,
    Stone = 3,
    Steel = 4,
    Copper = 5,
    Coal = 6,
    Fire = 7,
    Oil = 8,
    Wood = 9,
    Cloth = 10,
    MoltenMetal = 11,
    Leaf = 12,
    Plant = 13,
    Air = 14,
    Hydrogen = 15,
}
pub const TRANSPARENT_BLOCKS: &[BlockID] = &[
    BlockID::Water,
    BlockID::Air,
    BlockID::Hydrogen,
    BlockID::Steam,
];
pub const INVISIBLE_BLOCKS: &[BlockID] = &[
    BlockID::Air,
    BlockID::Hydrogen,
];

/// a specialised compressed version of the ``BlockID``, which contains fewer block ids and therefore fits in less
/// bits, which means smaller buffers and better performance.
/// mainly, it doesnt contain blocks that arent rendered, such as air and hydrogen.
/// it also doesnt contain blocks that look the same. there are no such examples currently, but in future it is possible that
/// two different blocks behave differently but look the same.
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, EnumIter)]
pub enum GPUBlockID {
    Water = 0,
    Steam = 1,
    Ground = 2,
    Stone = 3,
    Steel = 4,
    Copper = 5,
    Coal = 6,
    Fire = 7,
    Oil = 8,
    Wood = 9,
    Cloth = 10,
    MoltenMetal = 11,
    Leaf = 12,
    Plant = 13, // the numbers are just so i can keep track for bitpacking stuff
    TBD1 = 14,
    TBD2 = 15,
}