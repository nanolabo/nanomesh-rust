use wgpu::util::DeviceExt;
use std::any::Any;
use crate::render::Renderer;
use crate::objects::SceneObject;

pub type ObjectId = slotmap::DefaultKey;

pub struct Mesh {
    buffer: wgpu::Buffer,
}

impl Mesh {

    pub fn new(renderer: &Renderer) -> Self {
        const VERTICES: &[f32] = &[
            0.0, 0.5, 0.0,
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0
        ];

        let buffer = renderer.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        Mesh { 
            buffer: buffer,
        }
    }
}

impl SceneObject for Mesh {

    fn render<'a>(&'a mut self, queue: &wgpu::Queue, pass: &mut wgpu::RenderPass<'a>)
    {
        pass.set_vertex_buffer(0, self.buffer.slice(..));
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}