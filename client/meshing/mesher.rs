use std::collections::HashMap;

use shared::{blocks::block::{Block, BlockFace, get_block_texture_index, is_block_transparent}, loaders::texture_bin::{self, TextureBin}, util::xz_to_index, world::chunk::Chunk};

use crate::meshing::vertex::surface_vertex::SurfaceVertex;

fn get_block_at_absolute(abs_x: i32, abs_y: u32, abs_z: i32, chunks: &HashMap<u32, Chunk>) -> &Block {
	let (cx, cz) = (abs_x.div_euclid(16), abs_z.div_euclid(16));

	let chunk_opt = chunks.get(&xz_to_index(cx, cz));

	if let Some(chunk) = chunk_opt {
		return chunk.get_block_local(cx.rem_euclid(16) as u32, abs_y, cz.rem_euclid(16) as u32);
	}
	else {
		return &Block::Air;
	}
}

pub fn mesh_slice(
    chunk_x: i32,
    chunk_z: i32,
    slice: u32,
    chunks: &HashMap<u32, Chunk>,
    texture_bin: &TextureBin
) -> (Vec<SurfaceVertex>, Vec<u32>, u32) {
	let mut vertices = Vec::with_capacity(16 * 16 * 16 * 24);
    let mut indices = Vec::with_capacity(16 * 16 * 16 * 36);

	let chunk = chunks.get(&xz_to_index(chunk_x, chunk_z)).unwrap();

	for y in 0..16 {
		let abs_y = y + slice * 16;
		for x in 0..16 {
			let abs_x = x as i32 + chunk_x * 16;
			for z in 0..16 {
				let abs_z = z as i32 + chunk_z * 16;
			
				let block = chunk.get_block_local(x, abs_y, z);

				if is_block_transparent(block) {continue};

				let neighbors = [
                    get_block_at_absolute(abs_x, abs_y, abs_z + 1, chunks),
                    get_block_at_absolute(abs_x, abs_y, abs_z - 1, chunks),
                    get_block_at_absolute(abs_x + 1, abs_y, abs_z, chunks),
                    get_block_at_absolute(abs_x - 1, abs_y, abs_z, chunks),
                    get_block_at_absolute(abs_x, abs_y + 1, abs_z, chunks),
                    if abs_y == 0 {&Block::Air} else {get_block_at_absolute(abs_x, abs_y - 1, abs_z, chunks)},
                ];

				for (i, neighbor) in neighbors.iter().enumerate() {
					if is_block_transparent(*neighbor) {
						let (face_vertices, face_indices) = match i {
                            0 => (
                                [
                                    [x, y, z + 1],
                                    [x + 1, y, z + 1],
                                    [x, y + 1, z + 1],
                                    [x + 1, y + 1, z + 1],
                                ],
                                [0, 1, 2, 1, 3, 2],
                            ),
                            1 => (
                                [
                                    [x, y, z],
                                    [x + 1, y, z],
                                    [x, y + 1, z],
                                    [x + 1, y + 1, z],
                                ],
                                [2, 1, 0, 2, 3, 1],
                            ),
                            2 => (
                                [
                                    [x + 1, y, z],
                                    [x + 1, y, z + 1],
                                    [x + 1, y + 1, z],
                                    [x + 1, y + 1, z + 1],
                                ],
                                [2, 1, 0, 2, 3, 1],
                            ),
                            3 => (
                                [
                                    [x, y, z],
                                    [x, y, z + 1],
                                    [x, y + 1, z],
                                    [x, y + 1, z + 1],
                                ],
                                [0, 1, 2, 1, 3, 2],
                            ),
                            4 => (
                                [
                                    [x, y + 1, z],
                                    [x, y + 1, z + 1],
                                    [x + 1, y + 1, z],
                                    [x + 1, y + 1, z + 1],
                                ],
                                [0, 1, 2, 1, 3, 2],
                            ),
                            5 => (
                                [
                                    [x, y, z],
                                    [x, y, z + 1],
                                    [x + 1, y, z],
                                    [x + 1, y, z + 1],
                                ],
                                [2, 1, 0, 2, 3, 1],
                            ),
							_ => panic!("Invalid Neighbor index {}", i)
                        };

						let current_l = vertices.len();

						indices.extend(face_indices.iter().map(|&index| (index + current_l) as u32));

                        for (j, &pos) in face_vertices.iter().enumerate() {
                            vertices.push(SurfaceVertex::new(
                                pos, get_block_texture_index(texture_bin, block, BlockFace::from(i as u32)), BlockFace::from(i as u32), j as u32));
                        }
					}
				}
			}
		}
	}
	let l = indices.len();
    (vertices, indices, l as u32)
}