use glfw::{Action, Key, WindowEvent};

use crate::renderer::{primitives, renderer::State, world::World};

pub fn start(state: &mut State, world: &mut World, events: &glfw::GlfwReceiver<(f64, WindowEvent)>) {
    let cube = primitives::make_cube(&state.device, &mut state.materials, glm::vec3(1.0, 1.0, 1.0));
    world.load_model(state, cube.clone(), glm::vec3(0.0, 0.0, 0.0));
}

pub fn run(state: &mut State, world: &mut World, events: &glfw::GlfwReceiver<(f64, WindowEvent)>) {
        read_input(state, world, events);
}

fn read_input(state: &mut State, world: &mut World, events: &glfw::GlfwReceiver<(f64, WindowEvent)>) {
    for(_, event) in glfw::flush_messages(&events) {
        state.handle_glfw_event(&event);
        match event {

            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                state.window.set_should_close(true);
            }
            glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                world.keys.insert(glfw::Key::W, true);
            }
            glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => {
                world.keys.insert(glfw::Key::W, false);
            }
            // A
            glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                world.keys.insert(glfw::Key::A, true);
            }
            glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
                world.keys.insert(glfw::Key::A, false);
            }
            // S
            glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                world.keys.insert(glfw::Key::S, true);
            }
            glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
                world.keys.insert(glfw::Key::S, false);
            }
            // D
            glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                world.keys.insert(glfw::Key::D, true);
            }
            glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
                world.keys.insert(glfw::Key::D, false);
            }

            glfw::WindowEvent::FramebufferSize(width, height) => {
                state.update_surface();
                state.resize((width, height));
            }
            glfw::WindowEvent::Pos(..) => {
                state.update_surface();
                state.resize(state.size);
            }
            glfw::WindowEvent::MouseButton(button, action, _) => {
                println!("mouse button event: {button:?} {action:?}");
            }

            // default handling
            _ => {
                // println!("{:?}", event);
            }
        }
    }
}
