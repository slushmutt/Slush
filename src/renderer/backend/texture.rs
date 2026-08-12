use std::{env::current_dir, fs};
use glm::Vec4;
use image::GenericImageView;
use wgpu::util::DeviceExt;

use crate::renderer::backend::mesh_builder::any_as_u8_slice;

use super::bind_group;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

    
pub fn new_depth_texture(device: &wgpu::Device, 
    config: &wgpu::SurfaceConfiguration, label: &str) -> Texture {

    let size = wgpu::Extent3d {
        width: config.width.max(1),
        height: config.height.max(1),
        depth_or_array_layers: 1,
    };

    let descriptor = wgpu::TextureDescriptor {
        label: Some(label),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    };
    let texture = device.create_texture(&descriptor);

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    Texture { texture, view }
}
pub fn new_texture(filename: &str, device: &wgpu::Device, queue: &wgpu::Queue, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
    let mut filepath = current_dir().unwrap();
    filepath.push(filename);
    let filepath = filepath.into_os_string().into_string().unwrap();
    let bytes = fs::read(filepath).unwrap();

    let loaded_image = image::load_from_memory(&bytes).unwrap();
    let converted = loaded_image.to_rgba8();
    let size = loaded_image.dimensions();
    
    let texture_size = wgpu::Extent3d {
        width: size.0,
        height: size.1,
        depth_or_array_layers: 1
    };

    let texture_descriptor = wgpu::TextureDescriptor {
        label: Some(filename),
        mip_level_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        size: texture_size,
        sample_count: 1,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
    };
    let texture = device.create_texture(&texture_descriptor);
    queue.write_texture(wgpu::TexelCopyTextureInfo{
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All
    },
        &converted,
        wgpu::TexelCopyBufferLayout{
            offset: 0,
            // the amout of bytes taken up per pixel, the images width x 4 because there is 4 values
            // per pixel (rgba)
            bytes_per_row: Some(size.0 * 4),
            rows_per_image: Some(size.1),
        }, texture_size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let sampler_descriptor = wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        address_mode_w: wgpu::AddressMode::Repeat,
        min_filter: wgpu::FilterMode::Nearest,
        mag_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    };

    let sampler = device.create_sampler(&sampler_descriptor);

    let mut builder = bind_group::Builder::new(device);
    builder.set_layout(layout);
    builder.add_material(&view, &sampler);
    let bind_group = builder.build(filename);
    bind_group

}

pub fn new_color(color: &Vec4, device: &wgpu::Device, 
    label: &str, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {

    let bytes: &[u8] = unsafe { any_as_u8_slice(color) };

    let buffer_descriptor = wgpu::util::BufferInitDescriptor { 
        label: Some("Model vertex & index buffer"), 
        contents: bytes,
        usage: wgpu::BufferUsages::UNIFORM };

    let buffer = device.create_buffer_init(&buffer_descriptor);

    // build bind groups
    let bind_group: wgpu::BindGroup;
    {
        let mut builder = bind_group::Builder::new(device);
        builder.set_layout(layout);
        builder.add_buffer(&buffer, 0);
        bind_group = builder.build(label);
    }

    bind_group
}
