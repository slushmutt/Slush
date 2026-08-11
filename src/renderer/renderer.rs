use std::collections::HashMap;

use glfw::Key::P;
use glm::dot;
use glm::radians;
use raw_window_handle::HasWindowHandle;
use raw_window_handle::HasDisplayHandle;
use glfw::Window;
use glm::ext;

use crate::renderer::backend::mesh_builder::ObjLoader;
use crate::renderer::backend::pipeline::Builder;
use crate::renderer::backend::texture::*;
use crate::renderer::backend::uniform_buffer_object::UBOGroup;
use crate::renderer::backend::mesh_builder;
use crate::renderer::backend::definitions::*;
use crate::model::game_object::Object;
use crate::model::game_object::Camera;
use crate::renderer::backend::bind_group_layout;
use crate::renderer::backend::definitions::Vertex;
use crate::renderer::backend::uniform_buffer_object::UBO;

// the area that represents and manages all of my game objects
pub struct World {
    pub quads: Vec<Object>,
    pub tris: Vec<Object>,
    pub camera: Camera,
    pub keys: HashMap<glfw::Key, bool>,
}

impl World{
    pub fn new() -> Self{
        World {quads: Vec::new(), tris: Vec::new(), camera: Camera::new(), keys: HashMap::new()  }
    }

    pub fn update(&mut self, dt: f32, window: &mut Window) {
        for i in 0..self.tris.len() { self.tris[i].angle = self.tris[i].angle + 0.001 * dt;
            if self.tris[i].angle > 360.0 {
                self.tris[i].angle -= 360.0;
            }
        }
        
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

pub struct State<'a> {
    // an instance is a handle to the wgpu context
    instance: wgpu::Instance,
    // a surface is what we present our rendering too, in this case it is targetting the glfw window
    surface: wgpu::Surface<'a>,
    // a device is an abstracted version of the gpu being used to render
    device: wgpu::Device,
    // a queue is where work is submitted
    queue: wgpu::Queue,
    // settings for the surface
    config: wgpu::SurfaceConfiguration,
    // surface size/res
    pub size: (i32,i32),
    // a mutable reference to the glfw window itself
    pub window: &'a mut Window,

    render_pipelines: HashMap<PipelineType,wgpu::RenderPipeline>,
    
    triangle_mesh: wgpu::Buffer,

    quad_mesh: Mesh,

    triangle_material: wgpu::BindGroup,

    quad_material: wgpu::BindGroup,

    ubo_group: Option<UBOGroup>,

    projection_ubo: UBO,

    bind_group_layouts: HashMap<BindScope, wgpu::BindGroupLayout>,
    materials: Vec<Material>,
    pub models: Vec<Model>,
    depth_buffer: Texture,
}
impl<'a> State<'a> {
    // constructor
    pub async fn new(window: &'a mut Window) -> Self {
        let size = window.get_framebuffer_size();

        let instance = wgpu::Instance::default();

        // abstracting the window and making a target to render to the window
        let target = 
            wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_window_handle: window.window_handle().unwrap().as_raw(),
                raw_display_handle: Some(window.display_handle().unwrap().as_raw()),
            
        };
        // then creating a window with that target, this makes it now when we render to the surface
        // it is bound to the window
        let surface = unsafe { instance.create_surface_unsafe(target) }.unwrap();

        let adapter_descriptor = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            ..Default::default()
        };
        // the gpu itself, we create a device with it so that it can abstract the info
        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
            ..Default::default()
        };
        // the device is an abstraction which a app will use to access certain gpu capabilities
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats.iter()
            .copied().filter(|f| f.is_srgb())
            .next().unwrap_or(surface_capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };
        surface.configure(&device, &config);

        let triangle_mesh = mesh_builder::make_triangle(&device);
        let quad_mesh = mesh_builder::make_quad(&device);

        let bind_group_layouts = State::build_bind_group_layouts(&device);

        let render_pipelines = State::build_pipelines(&device,&config,&bind_group_layouts);
        let quad_material = new_texture("resources/bald.png", &device, &queue, &bind_group_layouts[&BindScope::Texture]);
        let triangle_material = new_texture("resources/background.jpg", &device, &queue, &bind_group_layouts[&BindScope::Texture]);

        let projection_ubo = UBO::new(&device, &bind_group_layouts[&BindScope::UBO]);

        let depth_buffer = new_depth_texture(&device, &config, "Depth Buffer");
        Self{
            instance,
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipelines,
            triangle_mesh,
            quad_mesh,
            triangle_material,
            quad_material,
            ubo_group: None,
            projection_ubo: projection_ubo,
            bind_group_layouts,
            materials: Vec::new(),
            models: Vec::new(),
            depth_buffer,
        }
    }

    pub fn build_ubos_for_objects(&mut self, object_count: usize) {
        self.ubo_group = Some(UBOGroup::new(&self.device, object_count, &self.bind_group_layouts[&BindScope::UBO]));
    }

    fn update_projection(&mut self, camera: &Camera) {

        let c0 = glm::Vec4::new(camera.right.x, camera.up.x, -camera.forward.x, 0.0);
        let c1 = glm::Vec4::new(camera.right.y, camera.up.y, -camera.forward.y, 0.0);
        let c2 = glm::Vec4::new(camera.right.z, camera.up.z, -camera.forward.z, 0.0);

        let a: f32 = -dot(camera.right, camera.position);
        let b: f32 = -dot(camera.up, camera.position);
        let c: f32 = dot(camera.forward, camera.position);

        let c3 = glm::Vec4::new(a, b, c, 1.0);
        let view = glm::Matrix4::new(c0,c1,c2,c3);

        let fov_y: f32 = radians(90.0);
        let aspect: f32 = 16.0 / 9.0;
        let z_near: f32 = 0.1;
        let z_far: f32 = 100.0;
        let projection = ext::perspective(fov_y, aspect, z_near, z_far);

        /*
        we have a world with a meaningless origin
        we have objects out in the world
        we also have a camera in the world
        we are viewing all of this from the cameras perspective
        the cameras x axis is left and right of the screen
        the camers y axis is top and bottom
        the cameras z is the depth

        so how do we get objects in the world into the cameras frame of reference
        we need to apply a transformation matrix to the camera to put it into the world
        so if we apply the inverse of the cameras model matrix to all objects in the world
        it puts the objects into the camera frame of reference
        so how do we do that
        we can do a geometric inverse
        theres a class of matricies called geometry matricies
        
        the bottom row is the identity
        the upper has a 3x3,
        and there is a collumn on the right
        the upper 3x3 is rotation
        the right vector is the translation
        the cameras object has a frame of reference
        which means it has a position
        and it has a collection of vectors which is the cameras directions

        in the cameras frame of reference the right vector of the geometric matrix corresponds to the x axis (the upper 3x3 first collumn)
        in the cameras frame of reference the up vector of the geometric matrix corresponds to the y axis (the upper 3x3 second collumn)
        in the cameras frame of reference the forward vector of the geometric matrix corresponds to the z axis (the upper 3x3 third collumn)
        in other words the right direction of the camera is the image of the cameras local x axis
        after it gets transformed into the world
        which is saying how to map the screen space into the world space
        the upper 3x3 of the matrix is a set of orthonormal vectors
        and we know how to invert them because the transpose of those vectors is its inverse

        the goal is to find matrix B such that BA = I (identity)
        so we take the upper 3x3 and transpose it
        which makes the collumns into rows
        eg
        right   (r,r,r,a)   (r,u,f,a)   (1,0,0,0)
        up      (u,u,u,b) * (r,u,f,b) = (0,1,0,0)
        forward (f,f,f,c) * (r,u,f,c) = (0,0,1,0)
                (0,0,0,1)   (0,0,0,1)   (0,0,0,1)
        
        when you multiply them by the original transformation matrix it will come out to the identitiy
        any dot product with themselves comes out to one because they are orthonormal
        the coieffeicent dosent matter because it gets multiplies by 0

        <r,p> + a = 0
        <u,p> + b = 0
        <f,p> + c = 0
            
        a = -<r,p>
        b = -<u,p>
        c = -<f,p>

        the view matrix is the following:
        (r,r,r, -<r,p>)
        (u,u,u, -<u,p>)
        (p,p,p, -<f,p>)
        (0,0,0,    1  )

        so how would we make the camera the center of the cordinate system
        we would subtract the cams position
        then we perform the opposite of whatever rotation the camera is doing
        imagine looking forward and you spin your head but you are the frame of reference
        you are staying still the world is spinning around you
        furthermore imagine walking forward but you are standing still and the world is moving towards you

        what the dot products are doing is projecting the position into the frame of reference of the camera
        you are essentially rotating first then subtracting the position in a way

        */

        

        let view_proj = projection * view;
        self.projection_ubo.upload(&view_proj, &self.queue);
    }
    fn update_transforms(&mut self, quads: &Vec<Object>, tris: &Vec<Object>) {
        let mut offset: u64 = 0;

        // loop through quads and apply ubo with matrix
        for i in 0..quads.len() {
            let c0 = glm::Vec4::new(1.0, 0.0, 0.0, 0.0);
            let c1 = glm::Vec4::new(0.0, 1.0, 0.0, 0.0);
            let c2 = glm::Vec4::new(0.0, 0.0, 1.0, 0.0);
            let c3 = glm::Vec4::new(0.0, 0.0, 0.0, 1.0);
            let m1 = glm::Matrix4::new(c0, c1, c2, c3);
            let m2 = glm::Matrix4::new(c0, c1, c2, c3);

            let matrix = 
                ext::rotate(&m1, quads[i].angle, glm::Vec3::new(0.0, 0.0, 1.0))
                * ext::translate(&m2, quads[i].position);
            
            self.ubo_group.as_mut().unwrap().upload(offset + i as u64, &matrix, &self.queue);
        }
        // because the ubo has one buffer we need to specifiy the offset between the types,
        // in this case tris and quads, so when it goes to tri operations,
        // you must add the amount of quads that were in the buffer to the index 
        // so that it gets to the tris properly
        offset += quads.len() as u64;
        // loop through tris and apply ubo with matrix
        for i in 0..tris.len() {
            // construct identity matrix
            let c0 = glm::Vec4::new(1.0, 0.0, 0.0, 0.0);
            let c1 = glm::Vec4::new(0.0, 1.0, 0.0, 0.0);
            let c2 = glm::Vec4::new(0.0, 0.0, 1.0, 0.0);
            let c3 = glm::Vec4::new(0.0, 0.0, 0.0, 1.0);
            let m1 = glm::Matrix4::new(c0, c1, c2, c3);
            let m2 = glm::Matrix4::new(c0, c1, c2, c3);

            let matrix = 
                ext::rotate(&m1, tris[i].angle,glm::Vec3::new(0.0, 0.0, 1.0))
                * ext::translate(&m2, tris[i].position);
            
            self.ubo_group.as_mut().unwrap().upload(offset + i as u64, &matrix, &self.queue);
        }
    }

    fn render_model(&self, model: &Model, renderpass: &mut wgpu::RenderPass, offset: usize) {
        // bind vertex and index buffer
        renderpass.set_vertex_buffer(0, 
            model.buffer.slice(0..model.ebo_offset));
        renderpass.set_index_buffer(model.buffer.slice(model.ebo_offset..), 
            wgpu::IndexFormat::Uint32);
        
        // transforms
        renderpass.set_bind_group(
            1, 
            &(self.ubo_group.as_ref().unwrap()).bind_groups[offset], 
            &[]);
        // renderpass.set_bind_group(2, &self.projection_ubo.bind_group, &[]);
        
        for submesh in &model.submeshes {
            // select pipeline
            let material = &self.materials[submesh.material_id];
            renderpass.set_pipeline(&self.render_pipelines[&material.pipeline_type]);
            renderpass.set_bind_group(0, 
                (material.texture).as_ref().unwrap(), &[]);
                
            let start = submesh.first_index as u32;
            renderpass.draw_indexed(start..(start + submesh.index_count), 0, 0..1);
        }
    }
    pub fn render(&mut self, quads: &Vec<Object>, tris: &Vec<Object>, camera: &Camera) {
        self.update_projection(camera);
        self.update_transforms(quads, tris);


        // wait for the queue to submit the tris and quads before going ahead
        let event = self.queue.submit([]);
        let maintain = wgpu::PollType::wait_indefinitely();
        self.device.poll(maintain);

        // grab the current surface texture that we will render too
        let drawable = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            _ => {
                panic!("Surface texture validation failed");
            }
        };

        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        // create an image view using that so we can see what is being rendered
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        // encodes draw commands
        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        let mut command_encoder = self.device.create_command_encoder(&command_encoder_descriptor);

        // setting up the clear color for the screen
        let color_attachment = wgpu::RenderPassColorAttachment{
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations { 
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.25,
                    g: 0.0,
                    b: 0.5,
                    a: 0.0
                }), 
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        };  
        let depth_stencil_attachement = wgpu::RenderPassDepthStencilAttachment{
            view: &self.depth_buffer.view,
            depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
            stencil_ops: None,
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor{
            label: Some("Renderpass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: Some(depth_stencil_attachement),
            occlusion_query_set: None,
            timestamp_writes: None,
            ..Default::default()
        };
        
        // starting render pass which flushes the surface with the clear color
        // now also loads shader using custom render pipeline
        {
            let mut renderpass = command_encoder.begin_render_pass(&render_pass_descriptor);
            renderpass.set_pipeline(&self.render_pipelines[&PipelineType::Simple]);
           
            // quads
            renderpass.set_bind_group(0, &self.quad_material, &[]);
            renderpass.set_bind_group(2, &self.projection_ubo.bind_group, &[]);
            // use new single buffer for both index and vertex buffer
            renderpass.set_vertex_buffer(0, self.quad_mesh.buffer.slice(0..self.quad_mesh.offset));
            renderpass.set_index_buffer(self.quad_mesh.buffer.slice(self.quad_mesh.offset..), wgpu::IndexFormat::Uint16);

            // bind the bind groups for the quads
            // draw all of the quads
            let mut offset: usize = 0;
            for i in 0..quads.len() {
                renderpass.set_bind_group(1, &self.ubo_group.as_ref().unwrap().bind_groups[offset + i], &[]);
                renderpass.draw_indexed(0..6, 0, 0..1);
            }

            // tris
            renderpass.set_bind_group(0, &self.triangle_material, &[]);
            renderpass.set_vertex_buffer(0, self.triangle_mesh.slice(..));
            // bind the bind groups for the tris
            // draw all of the tris
            offset += quads.len();
            for i in 0..tris.len() {
                renderpass.set_bind_group(1, &self.ubo_group.as_ref().unwrap().bind_groups[offset + i], &[]);
                renderpass.draw_indexed(0..6, 0, 0..1);
                renderpass.draw(0..3, 0..1);
            }
            self.render_model(&self.models[0], &mut renderpass, offset);
        }
        self.queue.submit(std::iter::once(command_encoder.finish()));

        // put it on the screen
        self.queue.present(drawable);
    }
    fn build_pipelines(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, bind_group_layouts: &HashMap<BindScope, wgpu::BindGroupLayout>) -> HashMap<PipelineType, wgpu::RenderPipeline> { 
        let mut pipelines: HashMap<PipelineType, wgpu::RenderPipeline> = HashMap::new();
        let mut pipeline_type: PipelineType;
        let mut pipeline: wgpu::RenderPipeline;
        let mut builder = Builder::new(device);
        pipeline_type = PipelineType::Simple;
        builder.set_shader_module("shaders/shader.wgsl", 
            "vs_main", "fs_main");
        builder.set_pixel_format(config.format);
        builder.add_vertex_buffer_layout(Vertex::get_layout());
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::Texture]);
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::UBO]);
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::UBO]);
        pipeline = builder.build("Simple Pipeline");
        pipelines.insert(pipeline_type, pipeline);

        pipeline_type = PipelineType::ColoredModel;
        builder.set_shader_module("shaders/colored_model_shader.wgsl", 
            "vs_main", "fs_main");
        builder.set_pixel_format(config.format);
        builder.add_vertex_buffer_layout(ModelVertex::get_layout());
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::Color]);
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::UBO]);
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::UBO]);
        pipeline = builder.build("Colored Model Pipeline");
        pipelines.insert(pipeline_type, pipeline);

        pipeline_type = PipelineType::TexturedModel;
        builder.set_shader_module("shaders/textured_model_shader.wgsl", 
            "vs_main", "fs_main");
        builder.set_pixel_format(config.format);
        builder.add_vertex_buffer_layout(ModelVertex::get_layout());
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::Texture]);
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::UBO]);
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::UBO]);
        pipeline = builder.build("Textured Model Pipeline");
        pipelines.insert(pipeline_type, pipeline);

        pipelines
    }
    fn build_bind_group_layouts(device: &wgpu::Device) -> HashMap<BindScope, wgpu::BindGroupLayout> {        
        let mut layouts: HashMap<BindScope, wgpu::BindGroupLayout> = HashMap::new();
        let mut layout: wgpu::BindGroupLayout;
        let mut scope = BindScope::Texture;
        let mut builder = bind_group_layout::Builder::new(device);
        builder.add_texture();
        layout = builder.build("Texture Bind Group Layout");
        layouts.insert(scope, layout);

        builder.add_vec4();
        scope = BindScope::Color;
        layout = builder.build("Color Bind Group Layout");
        layouts.insert(scope, layout);

        builder.add_mat4();
        scope = BindScope::UBO;
        layout = builder.build("UBO Bind Group Layout");
        layouts.insert(scope, layout);
        layouts
    }

    pub fn load_assets(&mut self) {
        let scale = 0.1;
        let c0 = glm::Vec4::new(scale, 0.0, 0.0, 0.0);
        let c1 = glm::Vec4::new(0.0, scale, 0.0, 0.0);
        let c2 = glm::Vec4::new(0.0, 0.0, scale, 0.0);
        let c3 = glm::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let pre_transform = glm::Matrix4::new(c0,c1,c2,c3);

        let mut loader = ObjLoader::new();
        self.models.push(loader.load(&mut self.materials, "Testing.obj", &self.device, &pre_transform));

        for material in &mut self.materials {
            material.texture = match material.pipeline_type {
                PipelineType::ColoredModel => {
                    Some(new_color(&(material.color.unwrap()),
                    &self.device,
                    "Color",
                    &self.bind_group_layouts[&BindScope::Color]))
                }
                PipelineType::TexturedModel => {
                    Some(new_texture(material.filename.as_ref().unwrap().as_str(),
                    &self.device,
                    &self.queue,
                    &self.bind_group_layouts[&BindScope::Texture]))
                }
                _ => {
                    None
                }
            }
        }
    }

    pub fn resize(&mut self, new_size: (i32,i32)){
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            // apply new size to surface
            self.surface.configure(&self.device, &self.config);

            self.depth_buffer.texture.destroy();
            self.depth_buffer = new_depth_texture(&self.device, &self.config, "Depth Buffer");
        }
    }

    pub fn update_surface(&mut self) {
        let target = unsafe { wgpu::SurfaceTargetUnsafe::from_window(&self.window) }.unwrap();
        self.surface = unsafe { self.instance.create_surface_unsafe(target) }.unwrap();
    }
}

