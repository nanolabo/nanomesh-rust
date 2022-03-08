#![allow(dead_code)]
#![allow(incomplete_include)]

#[path = "../src/lib.rs"]
mod nanoview2;

static SURFACE_CONFIG: wgpu::SurfaceConfiguration = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: wgpu::TextureFormat::Bgra8UnormSrgb,
    width: 1024,
    height: 1024,
    present_mode: wgpu::PresentMode::Fifo,
};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap(); // No need for FBO

    // SETUP
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) }; // No need for FBO

    let mut renderer = nanoview2::renderer::Renderer::new(&instance, surface, &SURFACE_CONFIG);

    // EVENT LOOP

    event_loop.run(move |event, _, control_flow| {
        match event {
            // ...
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                match renderer.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => {},
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}