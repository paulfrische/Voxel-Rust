use game::config;
use glfw::{Action, Context, Key};
use std::{os::raw::*, mem};

mod ogl;
mod game;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(1280, 720, "Voxel", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_resizable(false);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|s| glfw.get_proc_address_raw(s));
    gl::Viewport::load_with(|s| glfw.get_proc_address_raw(s));

    let vao = ogl::vao::VAO::new();
    vao.bind();

    let mut shader = ogl::shader::Shader::new("shader/vertex.glsl", "shader/fragment.glsl");
    shader.bind();
    shader.create_uniform("view");
    shader.create_uniform("projection");
    shader.create_uniform("model");

    let mut camera = game::camera::Camera::new(cgmath::point3(0f32, 0f32, 1f32), 0f32, -90f32, cgmath::vec3(0f32, 1f32, 0f32), config::PLAYER_SPEED, 0.01f32, 45f32);
    camera.set_position(((config::WORLD_SIZE_X_CHUNKS / 2) * config::CHUNK_WIDTH) as f32, 140f32, ((config::WORLD_SIZE_Y_CHUNKS / 2) * config::CHUNK_DEPTH) as f32);

    let game_world = game::world::World::new(&shader);

    unsafe {
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (9 * mem::size_of::<f32>()) as i32, 0 as *const c_void);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (9 * mem::size_of::<f32>()) as i32, (3 * mem::size_of::<f32>()) as *const c_void);
        gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, (9 * mem::size_of::<f32>()) as i32, (6 * mem::size_of::<f32>()) as *const c_void);
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        gl::EnableVertexAttribArray(2);
    }

    let mut current_frame: f64;
    let mut last_frame = 0f64;

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    while !window.should_close() {
        current_frame = glfw.get_time();
        let delta = (current_frame - last_frame) as f32;
        last_frame = current_frame;

        let mouse_pos = window.get_cursor_pos();
        camera.update(&mut shader, &window, delta, mouse_pos.0 as f32, mouse_pos.1 as f32);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.1f32, 0.1f32, 0.1f32, 1.0f32);
        }

        game_world.render(&camera, &shader);

        unsafe {
            if gl::GetError() != gl::NO_ERROR {
                println!("ERROR");
            }
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {},
            }
        }
    }
}
