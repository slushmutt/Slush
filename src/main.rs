use egui::Key::O;
use glfw::{Context, Action, Key, fail_on_errors};

mod renderer;
use renderer::renderer::*;
use renderer::world::*;

mod model;

mod utility;
use crate::application::testing;
use crate::utility::logging;

mod engine;
mod application;



async fn run() {
    // initialize glfw, have it crash when an error occurs.
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    
    // does not just return pointer to window
    // also gives a "glfw reciever" which seperates event handling object from the window object
    let (mut window, events) = glfw.create_window(2560, 1440, "WGPU", glfw::WindowMode::Windowed).unwrap();
    
    // create state
    // tells the window to poll specific events
    window.set_key_polling(true);
    window.set_size_polling(true);
    window.set_pos_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    if window.glfw.supports_raw_motion() {
        window.set_raw_mouse_motion(true);
    }

    let mut state = State::new(&mut window).await;
    let mut world = World::new();

    testing::start(&mut state, &mut world, &events);
    state.load_assets();
    state.build_ubos_for_objects(world.models.len() + 1);
    // initialize keys
    world.keys.insert(glfw::Key::W, false);
    world.keys.insert(glfw::Key::A, false);
    world.keys.insert(glfw::Key::S, false);
    world.keys.insert(glfw::Key::D, false);

    while !state.window.should_close() {
        // call poll events to stop the buffer of event objects from stacking
        world.update(16.67, state.window);
        glfw.poll_events();
        testing::run(&mut state, &mut world, &events);
        state.render(&world.primitives, &world.models, &world.camera);
         
    }
}
fn main() {
    logging::initialize();
    pollster::block_on(run());

}
