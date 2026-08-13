use std::collections::HashMap;
use glfw::Window;

use crate::{model::game_object::{Camera, Object}, renderer::{backend::{definitions::Model, mesh_builder::ObjLoader}, primitives, renderer::State}};


// the area that represents and manages all of my game objects
pub struct World {
    pub primitives: Vec<Object>,
    pub models: Vec<Object>,
    pub camera: Camera,
    pub keys: HashMap<glfw::Key, bool>,
}

impl World{
    pub fn new() -> Self{
        World {primitives: Vec::new(),  models: Vec::new(), camera: Camera::new(), keys: HashMap::new()  }
    }

    pub fn load_model_from_file(&mut self, state: &mut State, model_name: &str, position: glm::Vec3, scale: glm::Vec3) -> &Object {
        let c0 = glm::Vec4::new(scale.x, 0.0, 0.0, 0.0);
        let c1 = glm::Vec4::new(0.0, scale.y, 0.0, 0.0);
        let c2 = glm::Vec4::new(0.0, 0.0, scale.z, 0.0);
        let c3 = glm::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let pre_transform = glm::Matrix4::new(c0,c1,c2,c3);

        let mut loader = ObjLoader::new();
        state.models.push(loader.load(&mut state.materials, model_name, &state.device, &pre_transform));

        let _ = &self.models.push(Object {position: position, scale, angle: 0.0});
        return self.models.last().unwrap();
    }

    pub fn load_model<'a>(&'a mut self, state: &mut State, model: Model, position: glm::Vec3) -> usize {
        state.models.push(model);
        let scale = glm::vec3(1.0, 1.0, 1.0);
        let _ = &self.models.push(Object {position: position, scale, angle: 0.0});
        return self.models.len() - 1;
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
