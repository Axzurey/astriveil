use crate::loaders::{texture_bin::TextureBin};

#[derive(Clone, Copy, Debug)]
pub enum Block {
    Air,
    Stone,
    Dirt,
    Grass
}

#[derive(Clone, Copy, Debug)]
pub enum BlockFace {
    Front,
    Back,
    Right,
    Left,
    Top,
    Bottom,
}

impl From<u32> for BlockFace {
    fn from(value: u32) -> Self {
        match value {
            0 => BlockFace::Front,
            1 => BlockFace::Back,
            2 => BlockFace::Right,
            3 => BlockFace::Left,
            4 => BlockFace::Top,
            5 => BlockFace::Bottom,
            _ => panic!("{}", format!("BlockFace value {} is out of range (0-5)", value))
        }
    }
}

impl Into<u32> for BlockFace {
    fn into(self) -> u32 {
        match self {
            BlockFace::Front => 0,
            BlockFace::Back => 1,
            BlockFace::Right => 2,
            BlockFace::Left => 3,
            BlockFace::Top => 4,
            BlockFace::Bottom => 5
        }
    }
}

pub fn get_block_texture_index(texture_bin: &TextureBin, block: &Block, face: BlockFace) -> u32 {
    match block {
        Block::Stone => texture_bin.get_texture_index_by_name_block("stone"),
        Block::Dirt => texture_bin.get_texture_index_by_name_block("dirt"),
        Block::Grass => match face {
            BlockFace::Top => texture_bin.get_texture_index_by_name_block("grass-top"),
            BlockFace::Bottom => texture_bin.get_texture_index_by_name_block("dirt"),
            _ => texture_bin.get_texture_index_by_name_block("grass-side")
        },
        _ => panic!("get_block_texture_index should never be called for block: {:?}", block)
    }
}

pub fn is_block_transparent(block: &Block) -> bool {
    match block {
        Block::Air => true,
        _ => false
    }
}