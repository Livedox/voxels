mod input;
mod event;
mod graphics;
mod camera;
mod voxels;
mod program;
mod mesh;

#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;


use std::{io::Cursor};

use glium::{glutin::dpi::{PhysicalPosition}, uniforms::{MinifySamplerFilter, MagnifySamplerFilter}};

use crate::{program::Program, mesh::{Mesh, CrosshairMesh}, voxels::{chunk::{Chunk, self}, chunks::{Chunks, self}}, graphics::voxel_renderer, input::{InputEvent, State}};


fn main() {
    let mut input_event = input::InputEvent::new();
    #[allow(unused_imports)]
    use glium::{glutin, Surface};
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);;
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    display.gl_window().window().set_cursor_grab(glutin::window::CursorGrabMode::Confined).unwrap();
    display.gl_window().window().set_cursor_visible(false);

    let mut camera = camera::Camera::new(glm::vec3(0., 0., -1.), 1.2);
    let mut cam_x = 0.;
    let mut cam_y = 0.;

    let image = image::load(Cursor::new(&include_bytes!("../textures/atlas.png")),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
    let behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };

    let mut voxel_renderer = voxel_renderer::VoxelRenderer::new();
    let mut chunks = Chunks::new(1, 1, 1);
    let mut meshes: Vec<Mesh> = vec![];
    
    let crosshair_mesh = CrosshairMesh::new(&display, &vec![(-0.01, -0.01),(0.01, 0.01), (-0.01, 0.01), (0.01, -0.01)])
        .unwrap();

    let shader = graphics::shader::Shader::new("./shaders/vertex.glslv", "./shaders/fragment.glslf")
        .expect("Failed to open file");

    let crosshair_shader = graphics::shader::Shader::new("./shaders/crosshair.glslv", "./shaders/crosshair.glslf")
        .expect("Failed to open file");

    let program = Program::new(&display, &shader);
    let crosshair_program = Program::new(&display, &crosshair_shader);

    let mut fps: usize = 0;
    let mut now = std::time::Instant::now();
    let one_second = std::time::Duration::new(1, 0);
    let mut is_cursor = true;

    let speed: f32 = 5.0;
    let instant = std::time::Instant::now();
    let mut last_time = instant.elapsed().as_secs_f32();
    event_loop.run(move |event, _, control_flow| {
        let current_time = instant.elapsed().as_secs_f32();
        let delta_time = current_time - last_time;
        last_time = current_time;

        let wh_tuple = display.get_framebuffer_dimensions();
        let (width, height): (f32, f32) = (wh_tuple.0 as f32, wh_tuple.1 as f32);

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        

        input_event.update();
        if let Some(control) = event::match_event(event, &mut input_event) {
            *control_flow = control;
        }

        if input_event.keyboard[glutin::event::VirtualKeyCode::W as usize] == input::State::Pressed {
            camera.position += camera.front * speed * delta_time;
        }
        if input_event.keyboard[glutin::event::VirtualKeyCode::S as usize] == input::State::Pressed {
            camera.position -= camera.front * speed * delta_time;
        }
        if input_event.keyboard[glutin::event::VirtualKeyCode::D as usize] == input::State::Pressed {
            camera.position += camera.right * speed * delta_time;
        }
        if input_event.keyboard[glutin::event::VirtualKeyCode::A as usize] == input::State::Pressed {
            camera.position -= camera.right * speed * delta_time;
        }
        if input_event.keyboard[glutin::event::VirtualKeyCode::Escape as usize] == input::State::Jpressed {
            use glutin::window::CursorGrabMode;
            let mode = if is_cursor {CursorGrabMode::None} else {CursorGrabMode::Confined};
            display.gl_window().window().set_cursor_grab(mode).unwrap();
            display.gl_window().window().set_cursor_visible(is_cursor);
            is_cursor = !is_cursor;
        }
        if is_cursor {
            camera.rotation = glm::diagonal4x4(&glm::vec4(1., 1., 1., 1.));
            cam_x += -input_event.delta_x*speed/height;
            cam_y += -input_event.delta_y*speed/height;
            if cam_y > 1.56905099754 {cam_y = 1.56905099754}
            if cam_y < -1.56905099754 {cam_y = -1.56905099754}
            camera.rotate(cam_y, cam_x, 0.);
            let position = PhysicalPosition::new(width/2., height/2.);
            display.gl_window().window().set_cursor_position(position).unwrap();
        };

        if now.elapsed() >= one_second {
            println!("{}", fps);
            fps = 0;
            now = std::time::Instant::now();
        }
        fps += 1;

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let projview: [[f32; 4]; 4] = (camera.get_projection(width, height)*camera.get_view()).into();
        

        {
            let mut end: glm::TVec3<f64> = glm::vec3(0., 0., 0.);
            let mut norm: glm::TVec3<f64> = glm::vec3(0., 0., 0.);
            let mut iend: glm::TVec3<f64> = glm::vec3(0., 0., 0.);
            let rotation: glm::TVec3<f64> = glm::vec3(camera.rotation[0] as f64, camera.rotation[5] as f64, camera.rotation[10] as f64);
            let front: glm::TVec3<f64> = glm::vec3(camera.front[0] as f64, camera.front[1] as f64, camera.front[2] as f64);

            if fps > 100 {println!("{:?} {:?}", camera.rotation, camera.front)}

            let voxel = chunks.ray_cast(rotation, front, 10.0, &mut end, &mut norm, &mut iend);
            println!("{:?}", voxel);
            if voxel.is_some() {
                if input_event.mouse[0] == State::Pressed {
                    chunks.set(iend.x as i64, iend.y as i64, iend.z as i64, 0);
                }
                if input_event.mouse[1] == State::Pressed {
                    chunks.set((iend.x+norm.x) as i64, (iend.y+norm.y) as i64, (iend.z+norm.z) as i64, 2);
                }
            }
        }

        for i in 0..chunks.chunks.len() {
            if !chunks.chunks[i].modified { continue; };
            chunks.chunks[i].modified = false;

            let mut clossest: [Option<&Chunk>; 7] = [None, None, None, None, None, None, None];
            for j in 0..chunks.chunks.len() {
                let ox: i64 = chunks.chunks[j].x - chunks.chunks[i].x;
                let oy: i64 = chunks.chunks[j].y - chunks.chunks[i].y;
                let oz: i64 = chunks.chunks[j].z - chunks.chunks[i].z;
                if (ox.abs() == 1 && oy == 0 && oz == 0) ||
                (oy.abs() == 1 && ox == 0 && oz == 0) ||
                (oz.abs() == 1 && ox == 0 && oy == 0) ||
                (ox == 0 && oy ==0 && oz == 0)
                {
                    // Exemple oy = -1 ox = 0 oz = 0
                    // 3-(-1*3+0*2+0) => 3--3 => 6
                    clossest[(3-(oy*3+oz*2+ox)).abs() as usize] = Some(&chunks.chunks[j]);
                }
            }
            if i >= meshes.len() {
                meshes.push(voxel_renderer.render(&chunks.chunks[i], &clossest, &display));
            } else {
                meshes[i] = voxel_renderer.render(&chunks.chunks[i], &clossest, &display);
            } 
        }

        
        let iter = chunks.chunks.iter().zip(&meshes);
        for (chunk, mesh) in iter {
            let model: [[f32; 4]; 4] = glm::translate(
                &glm::make_mat4(&[
                    1.,0.,0.,0.,
                    0.,1.,0.,0.,
                    0.,0.,1.,0.,
                    0.,0.,0.,1.]),
                &glm::vec3(
                    (chunk.x*chunk::CHUNK_WIDTH as i64) as f32+0.5,
                    (chunk.y*chunk::CHUNK_HEIGHT as i64) as f32+0.5,
                    (chunk.z*chunk::CHUNK_DEPTH as i64) as f32+0.5))
                .into();
            let uniforms = uniform! {
                model: model,
                projview: projview,
                u_texture0: glium::uniforms::Sampler(&texture, behavior),
                a_color: [1.0, 1.0, 1.0, 1.0] as [f32; 4],
            };
            mesh.draw(&mut target, &program, &uniforms);
        }
        crosshair_mesh.draw(&mut target, &crosshair_program, &uniform! {});

        target.finish().unwrap();
    });
}