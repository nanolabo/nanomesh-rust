use nanomesh::base::Vector3;
use nanoview::{Camera, Renderer, Scene, PointLight, wgpu};
use nanoview::ultraviolet::{Vec3};
use std::mem::size_of;
use image::{ImageBuffer, Bgra};
use img_hash::{HasherConfig};
use std::time::{Instant};

static SURFACE_CONFIG: wgpu::SurfaceConfiguration = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: wgpu::TextureFormat::Bgra8UnormSrgb,
    width: 1024,
    height: 1024,
    present_mode: wgpu::PresentMode::Fifo,
};

static T: f64 = 1.61803398874989;
static ICOSPHERE_POSITIONS: [Vector3; 12] =
[
    Vector3 { x: -1., y: T, z: 0. },
    Vector3 { x: 1., y: T, z: 0. },
    Vector3 { x: -1., y: -T, z: 0. },
    Vector3 { x: 1., y: -T, z: 0. },
    Vector3 { x: 0., y: -1., z: T },
    Vector3 { x: 0., y: 1., z: T },
    Vector3 { x: 0., y: -1., z: -T },
    Vector3 { x: 0., y: 1., z: -T },
    Vector3 { x: T, y: 0., z: -1. },
    Vector3 { x: T, y: 0., z: 1. },
    Vector3 { x: -T, y: 0., z: -1. },
    Vector3 { x: -T, y: 0., z: 1. }
];

fn main()
{
    let now = Instant::now();

    nanoview::futures::executor::block_on(run_all_async());

    println!("done in: {} ms", now.elapsed().as_millis());    
}

async fn run_all_async()
{
    let hash1 = run_async("cases/helmet/helmet_original.glb").await.unwrap();
    let hash2 = run_async("cases/helmet/helmet_90p.glb").await.unwrap();

    let dist = hamming_distance(&hash1, &hash2);
    println!("distance: {}", dist);
}

async fn run_async(file_path: &str) -> Result<Vec<u8>, ()>
{
    let mut total_hash = Vec::new();

    let instance = wgpu::Instance::new(wgpu::Backends::all());

    let needed_extensions = wgpu::Features::empty();

    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
            force_fallback_adapter: false,
        },
    ).await.unwrap();
    let adapter_features = adapter.features();

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: adapter_features & needed_extensions,
            limits: wgpu::Limits::default(),
        },
        None,
    ).await.unwrap();

    let camera = Camera::new(SURFACE_CONFIG.width as f32 / SURFACE_CONFIG.height as f32);

    let mut scene = Scene::new(camera);
    let mut renderer = Renderer::new(&SURFACE_CONFIG, device, queue);

    let mesh = renderer.mesh_from_file(file_path, true);
    let bbox = mesh.bbox;
    let mesh_id = scene.add_mesh(mesh);

    // Unnecessary but perhaps educational?
    scene.mesh(mesh_id).position = Vec3::zero();
    scene.mesh(mesh_id).scale = Vec3::broadcast(1.0);

    // We'll position these lights down in the render loop
    scene.add_point_light(PointLight {
        pos: [0.0, 10.0, 10.0],
        color: [1.0, 1.0, 1.0],
        intensity: 1000.0,
    });

    scene.add_point_light(PointLight {
        pos: [10.0, 10.0, 10.0],
        color: [1.0, 1.0, 1.0],
        intensity: 750.0,
    });

    scene.add_point_light(PointLight {
        pos: [-5.0, 10.0, -5.0],
        color: [1.0, 1.0, 1.0],
        intensity: 500.0,
    });

    let camera_distance: f64 = 1.3 * bbox.diagonal();

    for i in 0..ICOSPHERE_POSITIONS.len() {

        // Update camera
        let camera_position = &ICOSPHERE_POSITIONS[i].normalized() * camera_distance;

        let cam_offset = Vec3::new(camera_position.x as f32, camera_position.y as f32, camera_position.z as f32);
        scene.camera.look_at(
            cam_offset,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        // Render scene
        let mut encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: SURFACE_CONFIG.width,
                height: SURFACE_CONFIG.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        };

        let buffer_dimensions = BufferDimensions::new(SURFACE_CONFIG.width as usize, SURFACE_CONFIG.height as usize);

        let output_buffer_desc = wgpu::BufferDescriptor {
            size: (buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };
        let output_buffer = renderer.device.create_buffer(&output_buffer_desc);

        // Render to framebuffer
        let fb_texture = renderer.device.create_texture(&texture_desc);
        let fb_view = fb_texture.create_view(&wgpu::TextureViewDescriptor::default());
        renderer.render(&fb_view, &mut encoder, &scene);

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                    texture: &fb_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(buffer_dimensions.padded_bytes_per_row as u32),
                    rows_per_image: std::num::NonZeroU32::new(SURFACE_CONFIG.height),
                },
            },
            texture_desc.size,
        );

        renderer.queue.submit(Some(encoder.finish()));

        {
            let bytes = get_buffer_bytes(&renderer, &output_buffer, &buffer_dimensions).await.unwrap();
            let image_buffer = get_image_buffer(bytes);
            
            //image_buffer.save(format!("image{}.jpg", i)).unwrap();

            let hasher = HasherConfig::new().to_hasher();
            let hash = hasher.hash_image(&image_buffer);
            total_hash.extend_from_slice(&hash.as_bytes());
        }

        output_buffer.unmap();
    }

    return Ok(total_hash);
}

async fn get_buffer_bytes(renderer: &Renderer, buffer: &wgpu::Buffer, buffer_dimensions: &BufferDimensions) -> Result<Vec<u8>, ()>
{
    let buffer_slice: wgpu::BufferSlice = buffer.slice(..);

    let mapping = buffer_slice.map_async(wgpu::MapMode::Read);
    renderer.device.poll(wgpu::Maintain::Wait);
    mapping.await.unwrap();

    let data: wgpu::BufferView = buffer_slice.get_mapped_range();

    use std::io::{Cursor, Read, Seek, SeekFrom, Write};

    let mut c = Cursor::new(Vec::new());

    for chunk in data.chunks(buffer_dimensions.padded_bytes_per_row) {
        c.write_all(&chunk[..buffer_dimensions.unpadded_bytes_per_row]).unwrap();
    }
    c.seek(SeekFrom::Start(0)).unwrap();

    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();

    return Ok(out);
}

fn get_image_buffer(bytes: Vec<u8>) -> ImageBuffer<Bgra<u8>, Vec<u8>>
{
    let image_buffer: ImageBuffer<Bgra<u8>, Vec<u8>> = ImageBuffer::<Bgra<u8>, Vec<u8>>::from_raw(SURFACE_CONFIG.width as u32, SURFACE_CONFIG.height as u32, bytes).unwrap();
    return image_buffer;
}

fn hamming_distance(a: &Vec<u8>, b: &Vec<u8>) -> u32 {
    a.as_slice().iter().zip(b.as_slice()).map(|(l, r)| (l ^ r).count_ones()).sum()
}

struct BufferDimensions {
    width: usize,
    height: usize,
    unpadded_bytes_per_row: usize,
    padded_bytes_per_row: usize,
}

impl BufferDimensions {
    fn new(width: usize, height: usize) -> Self {
        let bytes_per_pixel = size_of::<u32>();
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Self {
            width,
            height,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_from_estimator() {
        let a = Vector3 { x: 1., y: 2., z: 3. };
        let b = Vector3 { x: 4., y: 5., z: 6. };
        let c = Vector3 { x: 5., y: 7., z: 9. };
        assert_eq!(&a + &b, c);
        assert_ne!(&a + &b, a);
    }
}
