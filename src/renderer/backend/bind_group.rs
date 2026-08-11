pub struct Builder<'a> {
    entries: Vec<wgpu::BindGroupEntry<'a>>,
    layout: Option<&'a wgpu::BindGroupLayout>,
    device: &'a wgpu::Device,
}

impl<'a> Builder<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        Builder {
            entries: Vec::new(),
            layout: None,
            device: device,
        }
    }

    fn reset(&mut self) {
        self.entries.clear();
    }

    pub fn set_layout(&mut self, layout: &'a wgpu::BindGroupLayout) {
        self.layout = Some(layout);
    }

    pub fn add_material(&mut self, view: &'a wgpu::TextureView, sampler: &'a wgpu::Sampler){
        // push a texutre onto the binding group
        self.entries.push(wgpu::BindGroupEntry{
            binding: self.entries.len() as u32,
            resource: wgpu::BindingResource::TextureView(view)
        });
        // push a sampler onto the binding group, a sampler tells the shader how to read the data
        // from a texture and how to use it
        self.entries.push(wgpu::BindGroupEntry{
            binding: self.entries.len() as u32,
            resource: wgpu::BindingResource::Sampler(sampler)
        });
    }

    pub fn add_buffer(&mut self, buffer: &'a wgpu::Buffer, offset: u64){
        self.entries.push(wgpu::BindGroupEntry{
            binding: self.entries.len() as u32,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding{
                buffer: buffer,
                offset: offset,
                size: None
            })
        });
    }
    pub fn build(&mut self, label: &str) -> wgpu::BindGroup {
        let descriptor = wgpu::BindGroupDescriptor {
            label: Some(label),
            layout: self.layout.unwrap(),
            entries: &self.entries
        };
        let bind_group = self.device.create_bind_group(&descriptor);
        self.reset();

        bind_group
    }
}
