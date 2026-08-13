use glm::{Vec2, Vec3};
use image::codecs::png::FilterType::Sub;
use wgpu::util::DeviceExt;

use crate::renderer::backend::{definitions::{Material, Mesh, Model, ModelVertex, SubMesh, Vertex}, mesh_builder::{ObjLoader, any_as_u8_slice}};

pub fn make_cube(device: &wgpu::Device, materials: &mut Vec<Material>, scale: glm::Vec3) -> Model {
    let c0 = glm::Vec4::new(scale.x, 0.0, 0.0, 0.0);
    let c1 = glm::Vec4::new(0.0, scale.y, 0.0, 0.0);
    let c2 = glm::Vec4::new(0.0, 0.0, scale.z, 0.0);
    let c3 = glm::Vec4::new(0.0, 0.0, 0.0, 1.0);
    let pre_transform = glm::Matrix4::new(c0,c1,c2,c3);

    let mut loader = ObjLoader::new();
    loader.load(materials, "Cube.obj", device, &pre_transform)
    // let verticies: [ModelVertex; 8] = [
    //     ModelVertex  {position: Vec3::new(1.0, -1.0, -1.0), tex_coord: Vec2::new(0.0, 0.0), normal: Vec3::new(0.0, -1.0, 0.0)},
    //     ModelVertex  {position: Vec3::new(1.0, -1.0, 1.0), tex_coord: Vec2::new(1.0, 0.0), normal: Vec3::new(0.0, 1.0, 0.0)},
    //     ModelVertex  {position: Vec3::new(-1.0, -1.0, 1.0), tex_coord: Vec2::new(1.0, 1.0), normal: Vec3::new(1.0, 0.0, 0.0)},
    //     ModelVertex  {position: Vec3::new(-1.0, -1.0, -1.0), tex_coord: Vec2::new(0.0, 1.0), normal: Vec3::new(0.0, 0.0, 1.0)},
    //     ModelVertex  {position: Vec3::new(1.0, 1.0, -1.0), tex_coord: Vec2::new(0.0, 0.0), normal: Vec3::new(-1.0, 0.0, 0.0)},
    //     ModelVertex  {position: Vec3::new(1.0, 1.0, 1.0), tex_coord: Vec2::new(0.0, 0.0), normal: Vec3::new(0.0, 0.0, -1.0)},
    //     ModelVertex  {position: Vec3::new(-1.0, 1.0, 1.0), tex_coord: Vec2::new(0.0, 0.0), normal: Vec3::new(1.0, 0.0, 0.0)},
    //     ModelVertex  {position: Vec3::new(-1.0, 1.0, -1.0), tex_coord: Vec2::new(0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0)},
    // ];
    //
    // let indices: [u16; 36] = [
    //     // bottom
    //     0, 2, 1,
    //     0, 3, 2,
    //
    //     // top
    //     4, 5, 6,
    //     4, 6, 7,
    //
    //     // front
    //     1, 6, 5,
    //     1, 2, 6,
    //
    //     // back
    //     0, 4, 7,
    //     0, 7, 3,
    //
    //     // left
    //     2, 7, 6,
    //     2, 3, 7,
    //
    //     // right
    //     0, 1, 5,
    //     0, 5, 4,
    // ];
    //
    // let bytes_1 = unsafe {
    //     any_as_u8_slice(&verticies)
    // };
    // let bytes_2 = unsafe {
    //     any_as_u8_slice(&indices)
    // };
    // let bytes_merged: Vec<u8> = [bytes_1, bytes_2].concat();
    //
    // let buffer_descriptor = wgpu::util::BufferInitDescriptor {
    //     label: Some("Quad Vertex & Index Buffer"),
    //     contents: &bytes_merged, 
    //     usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::VERTEX
    // };
    //
    // let buffer = device.create_buffer_init(&buffer_descriptor);
    // let offset: u64 = bytes_1.len() as u64;
    // // let submeshes: Vec<SubMesh> = vec![SubMesh {first_index: 0, index_count: indices.len() as u32, material_id: 0}];
    // let mut submeshes = Vec::new();
    // submeshes.push(SubMesh{ first_index: 0, index_count: 6, material_id: 0});
    // submeshes.push(SubMesh{ first_index: 6, index_count: 6, material_id: 0});
    // submeshes.push(SubMesh{ first_index: 12, index_count: 6, material_id: 0});
    // submeshes.push(SubMesh{ first_index: 18, index_count: 6, material_id: 0});
    // submeshes.push(SubMesh{ first_index: 24, index_count: 6, material_id: 0});
    // submeshes.push(SubMesh{ first_index: 30, index_count: 6, material_id: 0});
    //
    //
    // Model{buffer: buffer, ebo_offset: offset / 2, submeshes, name: Some("Cube".to_string())}
}
