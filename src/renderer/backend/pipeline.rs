use std::env::current_dir;
use std::fs;

pub struct Builder<'a> {
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
    vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'static>>,
    bind_group_layouts: Vec<&'a wgpu::BindGroupLayout>,
    device: &'a wgpu::Device,
}

impl<'a> Builder<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        Builder {
            shader_filename: "dummy".to_string(),
            vertex_entry: "dummy".to_string(),
            fragment_entry: "dummy".to_string(),
            pixel_format: wgpu::TextureFormat::Rgba8Unorm,
            vertex_buffer_layouts: Vec::new(),
            bind_group_layouts: Vec::new(),
            device: device
        }
    }
    pub fn reset(&mut self) {
        self.bind_group_layouts.clear();
        self.vertex_buffer_layouts.clear();
    }

    pub fn add_vertex_buffer_layout(&mut self, layout: wgpu::VertexBufferLayout<'static>) {
        self.vertex_buffer_layouts.push(layout);
    }

    pub fn add_bind_group_layout(&mut self, layout: &'a wgpu::BindGroupLayout) {
        self.bind_group_layouts.push(layout);
    }
    // set the shader that you wanna use, the name of the vertex main function, and the name of the
    // fragment main function
    pub fn set_shader_module(&mut self, shader_filename: &str, vertex_entry: &str, fragment_entry: &str) {
        self.shader_filename = shader_filename.to_string();  
        self.vertex_entry = vertex_entry.to_string();  
        self.fragment_entry = fragment_entry.to_string();  
    }
    pub fn set_pixel_format(&mut self, pixel_format: wgpu::TextureFormat) {
        self.pixel_format = pixel_format;
    }

    // build the pipeline using the device that was assigned during the state creation
    pub fn build(&mut self, label: &str) -> wgpu::RenderPipeline {
        let mut filepath = current_dir().unwrap();
        filepath.push("src/");
        filepath.push(self.shader_filename.as_str());
        let filepath = filepath.into_os_string().into_string().unwrap();
        let source_code = fs::read_to_string(filepath).expect("Can't read shaders soruce code!");
        
        // pass in shader source code
        let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source_code.into()),
        };
        // wgpu then translates it from wgsl to whatever is needed
        let shader_module = &self.device.create_shader_module(shader_module_descriptor);
        

        let bind_group_layouts_wrapped: Vec<Option<&wgpu::BindGroupLayout>> = self.bind_group_layouts
            .iter()
            .cloned()
            .map(Some)
            .collect();

        let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some(label),
            bind_group_layouts: &bind_group_layouts_wrapped,
            ..Default::default()
        };
        let pipeline_layout = &self.device.create_pipeline_layout(&pipeline_layout_descriptor);

        // assign render tragets which is a memory buffer, or essentiallya digital canvas where the
        // gpu draws pixels before they are displayed, it specifically draws onto the back buffer
        // and when the window swap buffers is called it swaps the back buffer that was created in
        // the background to the front buffer which is the one that you are able to see
        let render_targets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let vertex_layouts_wrapped: Vec<Option<wgpu::VertexBufferLayout>> = self.vertex_buffer_layouts.clone()
            .into_iter()
            .map(Some)
            .collect();

        // create the render pipeline,which is a series of steps of turning 3d scene data into 2d
        // points to render it to the screen
        let depth_stencil = wgpu::DepthStencilState{
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Less),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        };
        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&pipeline_layout),

            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some(&self.vertex_entry),
                buffers: &vertex_layouts_wrapped,
                compilation_options: Default::default(),
            },
            // vertex shader transforms corners of the triangle, then it is put with the primitive
            // assembly, that puts it into shapes which goes into the rasterizer which builds it
            // into shapes, which interpolates everything and gets it down to per pixel basis, and
            // dispatches to fragment shader
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some(&self.fragment_entry),
                targets: &render_targets,
                compilation_options: Default::default(),
            }),

            depth_stencil: Some(depth_stencil),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None
        };
        let render_pipeline  = self.device.create_render_pipeline(&render_pipeline_descriptor);

        self.reset();

        render_pipeline
    }
}
