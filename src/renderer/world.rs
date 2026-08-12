use std::collections::HashMap;

use glfw::Window;

use crate::model::game_object::{Camera, Object};


// the area that represents and manages all of my game objects
pub struct World {
    pub quads: Vec<Object>,
    pub tris: Vec<Object>,
    pub models: Vec<Object>,
    pub camera: Camera,
    pub keys: HashMap<glfw::Key, bool>,
}

impl World{
    pub fn new() -> Self{
        World {quads: Vec::new(), tris: Vec::new(), models: Vec::new(), camera: Camera::new(), keys: HashMap::new()  }
    }

    pub fn update(&mut self, dt: f32, window: &mut Window) {
        // for i in 0..self.models.len() { self.models[i].angle = self.models[i].angle + 0.00001 * dt;
        //     if self.models[i].angle > 360.0 {
        //         self.models[i].angle -= 360.0;
        //     }
        // }
        
        let (mx, my) = window.get_cursor_pos();
        let (dx_raw, dy_raw) = (mx as f32 - self.camera.last_mouse.0, my as f32 - self.camera.last_mouse.1);
        self.camera.last_mouse = (mx as f32, my as f32);

        let dx = (-0.05 * dx_raw) as f32; 
        let dy = (-0.05 * dy_raw) as f32;
        self.camera.rotate(dx, dy);

        let mut d_right: f32 = 0.0;
        let mut d_forward: f32 = 0.0;

        let mut speed: f32 = 0.01;
        
        if self.keys[&glfw::Key::W] {
            d_forward = d_forward + speed; 
        }

        if self.keys[&glfw::Key::A] {
            d_right = d_right - speed; 
        }

        if self.keys[&glfw::Key::S] {
            d_forward = d_forward - speed; 
        }

        if self.keys[&glfw::Key::D] {
            d_right = d_right + speed; 
        }
        self.camera.walk(d_right, d_forward);
    } 

}
