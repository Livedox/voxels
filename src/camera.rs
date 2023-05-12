extern crate nalgebra_glm as glm;

pub struct Camera {
    pub fov: f32,
    pub position: glm::TVec3<f32>,
    pub front: glm::TVec3<f32>,
    pub up: glm::TVec3<f32>,
    pub right: glm::TVec3<f32>,
    pub rotation: glm::TMat4<f32>,
}


impl Camera {
    pub fn new(position: glm::TVec3<f32>, fov: f32) -> Camera {
        Camera {
            fov,
            position,
            rotation: glm::diagonal4x4(&glm::vec4(1., 1., 1., 1.)),
            front: glm::vec4_to_vec3(&(glm::diagonal4x4(&glm::vec4(1., 1., 1., 1.)) * glm::vec4(0., 0., -1., 1.0))),
            right: glm::vec4_to_vec3(&(glm::diagonal4x4(&glm::vec4(1., 1., 1., 1.)) * glm::vec4(1., 0., 0., 1.0))),
            up: glm::vec4_to_vec3(&(glm::diagonal4x4(&glm::vec4(1., 1., 1., 1.)) * glm::vec4(0., 1., 0., 1.0))),
        }
    }


    pub fn rotate(&mut self, x: f32, y: f32, z:f32) {
        self.rotation = glm::rotate_z(&self.rotation, z);
        self.rotation = glm::rotate_y(&self.rotation, y);
        self.rotation = glm::rotate_x(&self.rotation, x);

        self.update_vectors();
    }


    fn update_vectors(&mut self) {
        self.front = glm::vec4_to_vec3(&(self.rotation * glm::vec4(0., 0., -1., 1.0)));
        self.right = glm::vec4_to_vec3(&(self.rotation * glm::vec4(1., 0., 0., 1.0)));
        self.up = glm::vec4_to_vec3(&(self.rotation * glm::vec4(0., 1., 0., 1.0)));
    }


    pub fn get_projection(&self, width: f32, height: f32) -> glm::TMat4<f32> {
        glm::perspective(width/height, self.fov, 0.1, 100.)
    }


    pub fn get_view(&self) -> glm::TMat4<f32> {
        glm::look_at(&self.position, &(self.position+self.front), &self.up)
    }
}