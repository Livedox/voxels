use std::f64::INFINITY;

use super::{chunk::{Chunk, CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_DEPTH}, voxel::Voxel};


pub struct Chunks {
    pub chunks: Vec<Chunk>,
    pub volume: i64,
    pub width: i64,
    pub height: i64,
    pub depth: i64,
}

impl Chunks {
    pub fn new(width: i64, height: i64, depth: i64) -> Chunks {
        let volume = width*height*depth;
        let mut chunks: Vec<Chunk> = vec![];

        for y in 0..height {
            for z in 0..depth {
                for x in 0..width {
                    chunks.push(Chunk::new(x, y, z));
                }
            }
        }

        Chunks { chunks, volume, width, height, depth }
    }
    

    pub fn get(&self, x: i64, y: i64, z: i64) -> Option<&Voxel> {
        let mut cx = x/CHUNK_WIDTH as i64;
        let mut cy = y/CHUNK_HEIGHT as i64;
        let mut cz = z/CHUNK_DEPTH as i64;
        if x < 0 { cx -= 1 };
        if y < 0 { cy -= 1 };
        if z < 0 { cz -= 1 };
        if cx < 0 || cy < 0 || cz < 0 || cx >= CHUNK_WIDTH as i64 || cy >= CHUNK_HEIGHT as i64 || cz >= CHUNK_DEPTH as i64 {
            return None;
        }
        let chunk: &Chunk = &self.chunks[((cy * self.depth + cz) * self.width + cx) as usize];
        let lx = x - cx * CHUNK_WIDTH as i64;
        let ly= y - cy * CHUNK_HEIGHT as i64;
        let lz = z - cz * CHUNK_DEPTH as i64;
        Some(&chunk.voxels[((ly*CHUNK_DEPTH as i64+lz)*CHUNK_WIDTH as i64+lx) as usize])
    }
    

    pub fn get_chunk(&mut self, x: i64, y: i64, z: i64) -> Option<&mut Chunk> {
        if x < 0 || y < 0 || z < 0 || x >= self.width || y >= self.height || z >= self.depth {
            return None;
        }
        Some(&mut self.chunks[((y * self.depth + z) * self.width + x) as usize])
    }


    pub fn set(&mut self, x: i64, y: i64, z: i64, id: u64) {
        let mut cx = x / CHUNK_WIDTH as i64;
        let mut cy = y / CHUNK_HEIGHT as i64;
        let mut cz = z / CHUNK_DEPTH as i64;
        if x < 0 { cx -= 1 };
        if y < 0 { cy -= 1 };
        if z < 0 { cz -= 1 };
        if cx < 0 || cy < 0 || cz < 0 || cx >= self.width || cy >= self.height || cz >= self.depth {
            return;
        }
        println!("Work0");
        
        if self.chunks.get(((y * self.depth + z) * self.width + x) as usize).is_none() { return; };

        let mut chunk = &mut self.chunks[((y * self.depth + z) * self.width + x) as usize];
        let lx = x - cx * CHUNK_WIDTH as i64;
        let ly= y - cy * CHUNK_HEIGHT as i64;
        let lz = z - cz * CHUNK_DEPTH as i64;
        println!("Work");
        chunk.voxels[((ly*CHUNK_DEPTH as i64+lz)*CHUNK_WIDTH as i64+lx) as usize].id = id;
        chunk.modified = true;
        
        if lx == 0 {
            if let Some(chunk) = self.get_chunk(cx-1, cy, cz) {chunk.modified = true };
            if let Some(chunk) = self.get_chunk(cx+1, cy, cz) {chunk.modified = true };
        }
        if ly == 0 {
            if let Some(chunk) = self.get_chunk(cx, cy-1, cz) {chunk.modified = true };
            if let Some(chunk) = self.get_chunk(cx, cy+1, cz) {chunk.modified = true };
        }
        if lz == 0 {
            if let Some(chunk) = self.get_chunk(cx, cy, cz-1) {chunk.modified = true };
            if let Some(chunk) = self.get_chunk(cx, cy, cz+1) {chunk.modified = true };
        }
    }


    pub fn ray_cast(
        &self, a: glm::TVec3<f64>, dir: glm::TVec3<f64>, max_dist: f64,
        end: &mut glm::TVec3<f64>, norm: &mut glm::TVec3<f64>, iend: &mut glm::TVec3<f64>
    ) -> Option<&Voxel> {
        let px = a.x as f64;
        let py = a.y as f64;
        let pz = a.z as f64;

        let dx = dir.x as f64;
        let dy = dir.y as f64;
        let dz = dir.z as f64;

        let mut t: f64 = 0.0;
        let mut ix = px.floor() as i64;
        let mut iy = py.floor() as i64;
        let mut iz = pz.floor() as i64;

        let stepx = if dx > 0.0 {1.0} else{-1.0};
        let stepy = if dy > 0.0 {1.0} else{-1.0};
        let stepz = if dz > 0.0 {1.0} else{-1.0};

        let infinity = INFINITY;

        let tx_delta = if dx == 0.0 {infinity} else {(1.0/dx).abs()};
        let ty_delta = if dy == 0.0 {infinity} else {(1.0/dz).abs()};
        let tz_delta = if dz == 0.0 {infinity} else {(1.0/dz).abs()};

        let xdist = if stepx > 0.0 {ix as f64 + 1.0 - px} else {px - ix as f64};
        let ydist = if stepy > 0.0 {iy as f64 + 1.0 - py} else {py - iy as f64};
        let zdist = if stepz > 0.0 {iz as f64 + 1.0 - pz} else {pz - iz as f64};

        let mut tx_max = if tx_delta < infinity {tx_delta*xdist} else {infinity};
        let mut ty_max = if ty_delta < infinity {ty_delta*ydist} else {infinity};
        let mut tz_max = if tz_delta < infinity {tz_delta*zdist} else {infinity};

        let mut stepped_index: i64 = -1;

        while t < max_dist {
            let voxel = self.get(ix, iz, iz);
            let mut id = 0;
            let is_voxel = voxel.is_some();
            if let Some(voxel) = voxel {id = voxel.id};
            if !is_voxel || id != 0 {
                end.x = px + t * dx;
                end.y = py + t * dy;
                end.z = pz + t * dz;

                iend.x = ix as f64;
                iend.y = iy as f64;
                iend.z = iz as f64;

                (norm.x, norm.y, norm.z) = (0.0, 0.0, 0.0);
                if stepped_index == 0 {norm.x = -stepx};
                if stepped_index == 1 {norm.y = -stepy};
                if stepped_index == 2 {norm.z = -stepz};
                return voxel;
            }
            if tx_max < ty_max {
                if tx_max < tz_max {
                    ix += stepx as i64;
                    t = tx_max;
                    tx_max += tx_delta;
                    stepped_index = 0;
                } else {
                    iz += stepz as i64;
                    t = tz_max;
                    tz_max += tz_delta;
                    stepped_index = 2;
                }
            } else {
                if ty_max < tz_max {
                    iy += stepy as i64;
                    t = ty_max;
                    ty_max += ty_delta;
                    stepped_index = 1;
                } else {
                    iz += stepz as i64;
                    t = tz_max;
                    tz_max += tz_delta;
                    stepped_index = 2;
                }
            }    
        }
        iend.x = ix as f64;
        iend.y = iy as f64;
        iend.z = iz as f64;
    
        end.x = px + t * dx;
        end.y = py + t * dy;
        end.z = pz + t * dz;
        (norm.x, norm.y, norm.z) = (0.0, 0.0, 0.0);
        None
    }
}