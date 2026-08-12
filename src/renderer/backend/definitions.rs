use std::ops::Sub;

use glm::{Vec2, Vec3, Vec4};

use crate::renderer::backend::texture;

#[derive(Eq, Hash, PartialEq)]
pub enum PipelineType{
    ColoredModel,
    TexturedModel,
    Simple,
}

#[derive(Eq, Hash, PartialEq)]
pub enum BindScope{
    Color,
    Texture,
    UBO
}
pub struct Material{
    pub pipeline_type: PipelineType,
    pub color: Option<Vec4>,
    pub filename: Option<String>,
    pub texture: Option<wgpu::BindGroup>,
}
impl Material{
    pub fn new() -> Self {
        Material {
            pipeline_type: PipelineType::Simple,
            color: None,
            filename: None,
            texture: None
        }
    }
}

pub struct Mesh {
    // one buffer for both indicies and verticies because the gpu prefers one big batch of data vs 2
    // small ones
    pub buffer: wgpu::Buffer,
    pub offset: u64
}
#[derive(Clone, Copy)]
pub struct SubMesh {
    pub first_index: i32,
    pub index_count: u32,
    pub material_id: usize,
}
pub struct Model {
    pub buffer: wgpu::Buffer,
    pub ebo_offset: u64,
    pub submeshes: Vec<SubMesh>,
    pub name: Option<String>,
}
#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3
}
impl Vertex {

    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
        wgpu::VertexBufferLayout {array_stride: std::mem::size_of::<Vertex>() as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &ATTRIBUTES}
    }
    
}

#[repr(C)]
#[derive(PartialEq)]
pub struct ModelVertex {
    pub position: Vec3,
    pub tex_coord: Vec2,
    pub normal: Vec3
}
impl ModelVertex {

    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        
        const ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x3];
        wgpu::VertexBufferLayout {array_stride: std::mem::size_of::<ModelVertex>() as u64, step_mode: wgpu::VertexStepMode::Vertex, attributes: &ATTRIBUTES}
    }
    
}
