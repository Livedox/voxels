use glium::Display;

use crate::mesh::Mesh;
use crate::voxels::chunk::{Chunk, CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_DEPTH};
use crate::voxels::chunks::{Chunks, self};
use crate::voxels::voxel::Voxel;

const TEXTURE_ATLAS_COUNT: u64 = 16;

fn cdiv(x: f64, a: f64) -> f64 {
    if x < 0. {x / a - 1.} else {x / a}
}

fn local_neg(x: f64, size: f64) -> f64 {
    if x < 0. {(size)+x} else {x}
}

fn local(x: f64, size: f64) -> f64 {
    if x >= size {x - size} else {local_neg(x, size)}
}

fn get_chunk<'a>(x: f64, y: f64, z: f64, chunks: &'a [Option<&'a Chunk>; 7]) -> Option<&'a Chunk> {
    let index = (3. - cdiv(y, CHUNK_HEIGHT as f64).trunc() * 3. -
        cdiv(z, CHUNK_DEPTH as f64).trunc() * 2. -
        cdiv(x, CHUNK_WIDTH as f64).trunc()).abs() as usize;
    chunks[index]
}

fn is_chunk(x: f64, y: f64, z: f64, chunks: &[Option<&Chunk>; 7]) -> bool {
    get_chunk(x, y, z, chunks).is_some()
}

fn is_in(x: f64, y: f64, z: f64) -> bool {
    x >= 0. && x < CHUNK_WIDTH as f64 && y >= 0. && y < CHUNK_HEIGHT as f64 && z >= 0. && z < CHUNK_DEPTH as f64
}

// fn voxel<'a>(x: f64, y: f64, z: f64, chunks: &'a [Option<&'a Chunk>; 27]) -> Option<&'a Voxel> {
//     if let Some(chunk) = get_chunk(x, y, z, chunks) {
//         return Some(&chunk.voxels[(
//             local(y, CHUNK_HEIGHT as f64) * CHUNK_DEPTH as f64 +
//             local(z, CHUNK_DEPTH as f64) * CHUNK_WIDTH as f64 +
//             local(x, CHUNK_WIDTH as f64)) as usize]);
//     }
//     None
// }

fn voxel(x: f64, y: f64, z: f64, chunk: &Chunk) -> &Voxel {
    &chunk.voxels[(
        (
            local(y, CHUNK_HEIGHT as f64) * CHUNK_DEPTH as f64 + 
            local(z, CHUNK_DEPTH as f64)
        ) * CHUNK_WIDTH as f64 + local(x, CHUNK_WIDTH as f64)
        ) as usize]
}

fn is_blocked(x: f64, y: f64, z: f64, chunk: &Chunk, chunks: &[Option<&Chunk>; 7], t_x: f64, t_y: f64, t_z: f64) -> bool {
    is_chunk(x+t_x, y+t_y, z+t_z, chunks) && voxel(x+t_x, y+t_y, z+t_z, chunk).id != 0
    // voxel(x+t_x, y+t_y, z+t_z, chunk).id != 0
    // is_chunk(x+t_x, y+t_y, z+t_z, chunks)
}

// fn is_blocked(x: f64, y: f64, z: f64, chunks: &[Option<&Chunk>; 27]) -> bool {
//     if let Some(voxel) = voxel(x, y, z, chunks) {
//         return !is_chunk(x, y, z, chunks) || voxel.id != 0;
//     }
//     !is_chunk(x, y, z, chunks);
//     false
// }

// #define IS_CHUNK(X,Y,Z) (GET_CHUNK(X,Y,Z) != nullptr)
// #define GET_CHUNK(X,Y,Z) (chunks[((CDIV(Y, CHUNK_H)+1) * 3 + CDIV(Z, CHUNK_D) + 1) * 3 + CDIV(X, CHUNK_W) + 1])

// #define VOXEL(X,Y,Z) (GET_CHUNK(X,Y,Z)->voxels[(LOCAL(Y, CHUNK_H) * CHUNK_D + LOCAL(Z, CHUNK_D)) * CHUNK_W + LOCAL(X, CHUNK_W)])
// #define IS_BLOCKED(X,Y,Z) ((!IS_CHUNK(X, Y, Z)) || VOXEL(X, Y, Z).id)


pub struct VoxelRenderer {
    buffer: Vec<(f64, f64, f64, f64, f64, f64)>,
}


impl VoxelRenderer {
    pub fn new() -> VoxelRenderer {
        VoxelRenderer { buffer: vec![] }
    }


    pub fn render(&mut self, chunk: &Chunk, chunks: &[Option<&Chunk>; 7], display: &Display) -> Mesh {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_DEPTH {
                for x in 0..CHUNK_WIDTH {
                    let id = chunk.voxels[(y*CHUNK_DEPTH+z)*CHUNK_WIDTH+x].id;
                    if id == 0 { continue };
                    let (x, y, z) = (x as f64, y as f64, z as f64);

                    let mut l: f64 = 1.0;
                    let uvsize: f64 = 1.0/TEXTURE_ATLAS_COUNT as f64;
                    let u: f64 = (id%TEXTURE_ATLAS_COUNT) as f64 * uvsize;
                    let v: f64 = 1.0 - (1 + id/TEXTURE_ATLAS_COUNT) as f64 * uvsize;

                    if !is_blocked(x, y, z, chunk, chunks, 0., 1., 0.) {
                        self.vertex(x-0.5, y+0.5, z-0.5, u+uvsize, v, l);
                        self.vertex(x-0.5, y+0.5, z+0.5, u+uvsize,v+uvsize, l);
                        self.vertex(x+0.5, y+0.5, z+0.5, u,v+uvsize, l);
    
                        self.vertex(x-0.5, y+0.5, z-0.5, u+uvsize,v, l);
                        self.vertex(x+0.5, y+0.5, z+0.5, u,v+uvsize, l);
                        self.vertex(x+0.5, y+0.5, z-0.5, u,v, l);
                    }
                    if !is_blocked(x, y, z, chunk, chunks, 0., -1., 0.) {
                        l = 0.7;
                        self.vertex(x-0.5, y-0.5, z-0.5, u,v, l);
                        self.vertex(x+0.5, y-0.5, z+0.5, u+uvsize,v+uvsize, l);
                        self.vertex(x-0.5, y-0.5, z+0.5, u,v+uvsize, l);

                        self.vertex(x-0.5, y-0.5, z-0.5, u,v, l);
                        self.vertex(x+0.5, y-0.5, z-0.5, u+uvsize,v, l);
                        self.vertex(x+0.5, y-0.5, z+0.5, u+uvsize,v+uvsize, l);
                    }

                    if !is_blocked(x, y, z, chunk, chunks, 1., 0., 0.) {
                        l = 0.6;
                        self.vertex(x+0.5, y-0.5, z-0.5, u+uvsize,v, l);
                        self.vertex(x+0.5, y+0.5, z-0.5, u+uvsize,v+uvsize, l);
                        self.vertex(x+0.5, y+0.5, z+0.5, u,v+uvsize, l);

                        self.vertex(x+0.5, y-0.5, z-0.5, u+uvsize,v, l);
                        self.vertex(x+0.5, y+0.5, z+0.5, u,v+uvsize, l);
                        self.vertex(x+0.5, y-0.5, z+0.5, u,v, l);
                    }
                    if !is_blocked(x, y, z, chunk, chunks, -1., 0., 0.) {
                        l = 0.55;
                        self.vertex(x-0.5, y-0.5, z-0.5, u,v, l);
                        self.vertex(x-0.5, y+0.5, z+0.5, u+uvsize,v+uvsize, l);
                        self.vertex(x-0.5, y+0.5, z-0.5, u,v+uvsize, l);

                        self.vertex(x-0.5, y-0.5, z-0.5, u,v, l);
                        self.vertex(x-0.5, y-0.5, z+0.5, u+uvsize,v, l);
                        self.vertex(x-0.5, y+0.5, z+0.5, u+uvsize,v+uvsize, l);
                    }

                    if !is_blocked(x, y, z, chunk, chunks, 0., 0., 1.) {
                        l = 0.4;
                        self.vertex(x-0.5, y-0.5, z+0.5, u,v, l);
                        self.vertex(x+0.5, y+0.5, z+0.5, u+uvsize,v+uvsize, l);
                        self.vertex(x-0.5, y+0.5, z+0.5, u,v+uvsize, l);

                        self.vertex(x-0.5, y-0.5, z+0.5, u,v, l);
                        self.vertex(x+0.5, y-0.5, z+0.5, u+uvsize,v, l);
                        self.vertex(x+0.5, y+0.5, z+0.5, u+uvsize,v+uvsize, l);
                    }
                    if !is_blocked(x, y, z, chunk, chunks, 0., 0., -1.) {
                        l = 0.1;
                        self.vertex(x-0.5, y-0.5, z-0.5, u+uvsize,v, l);
                        self.vertex(x-0.5, y+0.5, z-0.5, u+uvsize,v+uvsize, l);
                        self.vertex(x+0.5, y+0.5, z-0.5, u,v+uvsize, l);

                        self.vertex(x-0.5, y-0.5, z-0.5, u+uvsize,v, l);
                        self.vertex(x+0.5, y+0.5, z-0.5, u,v+uvsize, l);
                        self.vertex(x+0.5, y-0.5, z-0.5, u,v, l);
                    }
                }
            }
        }
        Mesh::new(display, &self.buffer).unwrap()
    }


    pub fn vertex(&mut self, x: f64, y: f64, z: f64, u: f64, v: f64, l: f64) {
        self.buffer.push((x, y, z, u, v, l));
    }
}