use glium::{VertexBuffer, index::NoIndices, Display, vertex::BufferCreationError, Frame, uniforms, Surface};

use crate::program::{Program};


pub trait MeshTrait {
    fn draw<U>(&self, target: &mut Frame, program: &Program, uniforms: &U)
    where U: uniforms::Uniforms;
}


#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f64; 3],
    tex_coords: [f64; 2],
    v_light: f64
}

impl Vertex {
    pub fn new(coords: (f64, f64, f64, f64, f64, f64)) -> Vertex {
        Vertex { position: [coords.0, coords.1, coords.2], tex_coords: [coords.3, coords.4], v_light: coords.5 }
    }
}
implement_vertex!(Vertex, position, tex_coords, v_light);


#[derive(Copy, Clone, Debug)]
pub struct VertexCrosshair {
    position: [f64; 2]
}

impl VertexCrosshair {
    pub fn new(coords: (f64, f64)) -> VertexCrosshair {
        VertexCrosshair { position: [coords.0, coords.1] }
    }
}
implement_vertex!(VertexCrosshair, position);




#[derive(Debug)]

pub struct Mesh {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub indices: NoIndices,
}


impl Mesh {
    pub fn new(display: &Display, coords: &Vec<(f64, f64, f64, f64, f64, f64)>) -> Result<Mesh, BufferCreationError> {
        let shape: Vec<Vertex> = coords.iter().map(|&item| Vertex::new(item)).collect();
        let vertex_buffer = glium::VertexBuffer::new(display, &shape)?;
        Ok(Mesh { 
            vertex_buffer,
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
        })
    }
}


impl Mesh {
    pub fn draw<U>(&self, target: &mut Frame, program: &Program, uniforms: &U)
    where U: uniforms::Uniforms {
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        target.draw(&self.vertex_buffer, &self.indices, &program.program, uniforms, &params)
            .unwrap()
    }
}


pub struct CrosshairMesh {
    pub vertex_buffer: VertexBuffer<VertexCrosshair>,
    pub indices: NoIndices,
}


impl CrosshairMesh {
    pub fn new(display: &Display, coords: &Vec<(f64, f64)>) -> Result<CrosshairMesh, BufferCreationError> {
        let shape: Vec<VertexCrosshair> = coords.iter().map(|&item| VertexCrosshair::new(item)).collect();
        let vertex_buffer = glium::VertexBuffer::new(display, &shape)?;
        Ok(CrosshairMesh {
            vertex_buffer,
            indices: glium::index::NoIndices(glium::index::PrimitiveType::LinesList)
        })
    }
}


impl CrosshairMesh {
    pub fn draw<U>(&self, target: &mut Frame, program: &Program, uniforms: &U)
    where U: uniforms::Uniforms {
        target.draw(&self.vertex_buffer, &self.indices, &program.program, uniforms, &Default::default())
            .unwrap()
    }
}