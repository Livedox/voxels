use super::voxel::{self, Voxel};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_WIDTH*CHUNK_HEIGHT*CHUNK_DEPTH;


pub struct Chunk {
    pub voxels: [voxel::Voxel; CHUNK_VOLUME],
    pub modified: bool,
    pub x: i64,
    pub y: i64,
    pub z: i64
}


impl Chunk {
    pub fn new(pos_x: i64, pos_y: i64, pos_z: i64) -> Chunk {
        let mut voxels = [Voxel::new(0); CHUNK_VOLUME];
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_DEPTH {
                for x in 0..CHUNK_WIDTH {
                    let real_x = x + pos_x as usize*CHUNK_WIDTH;
                    let real_y = y + pos_y as usize*CHUNK_HEIGHT;
                    let real_z = z + pos_z as usize*CHUNK_DEPTH;
                    if real_y as f64 <= ((real_x as f64 *0.3).sin() * 0.5 + 0.5) * 10. {
                        voxels[(y*CHUNK_DEPTH+z)*CHUNK_WIDTH+x].id = 1;
                    }
                    if real_y <= 2 {
                        voxels[(y*CHUNK_DEPTH+z)*CHUNK_WIDTH+x].id = 2;
                    }
                }
            }
        }
        Chunk { voxels, x: pos_x, y: pos_y, z: pos_z, modified: true }
    }
}