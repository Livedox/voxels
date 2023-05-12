use std::{fs::File, io::{self, Read, BufReader}};


pub struct Shader {
    pub vertex: String,
    pub fragment: String,
}


impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Shader, io::Error> {
        let mut shader = Shader {vertex: String::new(), fragment: String::new()};
        let f = File::open(vertex_path)?;
        let mut buf_reader = BufReader::new(f);
        buf_reader.read_to_string(&mut shader.vertex)?;

        let f = File::open(fragment_path)?;
        let mut buf_reader = BufReader::new(f);
        buf_reader.read_to_string(&mut shader.fragment)?;

        Ok(shader)
    }
}