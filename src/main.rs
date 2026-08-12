use glfw::{Context, Action, Key, fail_on_errors};

mod renderer;
use renderer::renderer::*;
use renderer::world::*;

mod model;
use model::game_object::Object;

mod utility;
use crate::utility::logging;

mod engine;



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

    // window.set_cursor_mode(glfw::CursorMode::Disabled);

    // if window.glfw.supports_raw_motion() {
    //     window.set_raw_mouse_motion(true);
    // }

    let mut state = State::new(&mut window).await;
    let mut world = World::new();
    world.models.push(Object {position: glm::Vec3::new(0.0, 0.0, 0.0), angle: 0.0});
    state.load_assets();
    state.build_ubos_for_objects(world.quads.len() + world.tris.len() + state.models.len());
    // initialize keys
    world.keys.insert(glfw::Key::W, false);
    world.keys.insert(glfw::Key::A, false);
    world.keys.insert(glfw::Key::S, false);
    world.keys.insert(glfw::Key::D, false);

    while !state.window.should_close() {
        // call poll events to stop the buffer of event objects from stacking
        world.update(16.67, state.window);
        glfw.poll_events();

        for(_, event) in glfw::flush_messages(&events) {
            state.handle_glfw_event(&event);
            match event {
                // key checking
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    state.window.set_should_close(true);
                }

                // W
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
        state.render(&world.quads, &world.tris, &world.models, &world.camera);
         
    }
}
fn main() {
    logging::initialize();
    pollster::block_on(run());

}
