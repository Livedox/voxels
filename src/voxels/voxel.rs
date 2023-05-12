#[derive(Clone, Copy, Debug)]
pub struct Voxel {
    pub id: u64,
}

impl Voxel {
    pub fn new(id: u64) -> Voxel {
        Voxel { id }
    }
}