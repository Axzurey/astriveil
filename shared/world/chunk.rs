use nalgebra::Vector2;

use crate::{blocks::block::Block, util::local_xyz_to_index};

pub struct Chunk {
    pub grid: [[Block; 4096]; 16],
    pub position: Vector2<i32>
}

impl Chunk {
    pub fn generate_terrain(position: Vector2<i32>) -> Chunk {
        let grid: [[Block; 4096]; 16] = (0..16).map(|y_slice| {
            let mut blocks: [Block; 4096] = std::iter::repeat_n(Block::Air, 4096).collect::<Vec<Block>>().try_into().unwrap();

            for y in 0..16 {
                let abs_y = y_slice * 16 + y as i32;
                for x in 0..16 {
                    let abs_x = position.x * 16 + x as i32;
                    for z in 0..16 {
                        let abs_z = position.y * 16 + z as i32;

                        if abs_y < 30 {
                            blocks[local_xyz_to_index(x, y, z) as usize] = Block::Stone;
                        }
                        else if abs_y < 34 {
                            blocks[local_xyz_to_index(x, y, z) as usize] = Block::Dirt;
                        }
                        else if abs_y < 35 {
                            blocks[local_xyz_to_index(x, y, z) as usize] = Block::Grass;
                        }
                    }
                }
            }

            blocks
        }).collect::<Vec<[Block; 4096]>>().try_into().unwrap();
        

        Self {
            position,
            grid
        }
    }

    pub fn get_block_local(&self, x: u32, y: u32, z: u32) -> &Block {
        return &self.grid[y as usize / 16][local_xyz_to_index(x, y % 16, z) as usize];
    }
}