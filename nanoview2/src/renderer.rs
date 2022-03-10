use super::scene::Scene;
use super::scene::SceneObject;
use slotmap::DenseSlotMap;

pub type ObjectId = slotmap::DefaultKey;

pub struct Renderer {
    // Scene
    pub objects: DenseSlotMap<ObjectId, Box<dyn SceneObject>>,
    // Rendering
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new<'a>(
        instance: &wgpu::Instance,
        surface: wgpu::Surface,
        surface_config: &wgpu::SurfaceConfiguration
    ) -> Renderer {

        let needed_extensions = wgpu::Features::empty();
    
        let adapter_fut = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface), // None for FBO
                force_fallback_adapter: false,
            },
        );
        let adapter = futures::executor::block_on(adapter_fut).unwrap();
    
        let adapter_features = adapter.features();
    
        let device_fut = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: adapter_features & needed_extensions,
                limits: wgpu::Limits::default(),
            },
            None,
        );
        let (device, queue) = futures::executor::block_on(device_fut).unwrap();

        surface.configure(&device, &surface_config);


        // Render pipeline

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });
    
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
    
        let vertex_desc = wgpu::VertexBufferLayout {
            array_stride: 3 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3],
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex_desc],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        return Renderer {
            objects: DenseSlotMap::new(),
            surface: surface,
            device: device,
            queue: queue,
            render_pipeline: render_pipeline,
        };
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what [[location(0)]] in the fragment shader targets
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }
                            ),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });
    
            // NEW!
            render_pass.set_pipeline(&self.render_pipeline);

            for object in self.objects.iter() {
                object.1.render(&mut render_pass);
            }

            render_pass.draw(0..3, 0..1);
        }
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}

impl Scene for Renderer {

    fn add<T: 'static + SceneObject>(&mut self, scene_object: T) -> ObjectId {
        self.objects.insert(Box::new(scene_object))
    }

    fn update(&self) {
        for object in self.objects.iter() {
            object.1.update();
        }
    }

    fn get_internal(&self, id: ObjectId) -> Option<&Box<dyn SceneObject>> {
        self.objects.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SURFACE_CONFIG: wgpu::SurfaceConfiguration = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: 1024,
        height: 1024,
        present_mode: wgpu::PresentMode::Fifo,
    };

    #[test]
    fn wgpu_can_start() {


    }
}
