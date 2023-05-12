use glium::{program, Display};

use crate::graphics::shader::Shader;



pub struct Program {
    pub program: glium::program::Program,
}

impl Program {
    pub fn new(display: &Display, shader: &Shader) -> Program {
        Program {
            program: glium::program::Program::from_source(display, &shader.vertex, &shader.fragment, None)
                .unwrap(),
        }
    }
}