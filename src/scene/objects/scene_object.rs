use std::any::Any;
use crate::render::Renderer;

pub trait SceneObject {
    fn render<'a>(&'a mut self, renderer: &wgpu::Queue, pass: &mut wgpu::RenderPass<'a>) {} // Todo: move to rendering layer
    fn update(&self) {}
    fn load(&mut self, device: &wgpu::Device) {}
    fn unload(&self) {}
    fn as_any(&self) -> &dyn Any;
}