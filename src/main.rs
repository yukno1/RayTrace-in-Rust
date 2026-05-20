mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
// mod render;
mod sphere;
mod texture;
mod utils;
mod vec3;

// cpu
use crate::{
    bvh::BVHNode,
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    texture::{CheckerTexture, Texture},
    utils::{rand_f64, rand_f64_range},
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

// gpu
// use {
//     anyhow::{Context, Result},
//     winit::{
//         application::ApplicationHandler,
//         event::{Event, WindowEvent},
//         event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
//         window::{Window, WindowAttributes, WindowId},
//     },
// };

// const WIDTH: u32 = 800;
// const HEIGHT: u32 = 600;

// struct App {
//     #[allow(unused)]
//     window: Arc<Window>,
//     surface: wgpu::Surface<'static>,
//     device: Arc<wgpu::Device>,
//     queue: Arc<wgpu::Queue>,
//     config: wgpu::SurfaceConfiguration,
// }

// #[derive(Default)]
// struct AppHandler {
//     app: Option<App>,
// }

// impl ApplicationHandler for AppHandler {
//     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//         // 恢复事件
//         if self.app.is_some() {
//             return;
//         }

//         let window_size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
//         let window_attrs = WindowAttributes::default()
//             .with_inner_size(window_size)
//             .with_resizable(false)
//             .with_title("GPU Path Tracer".to_string());
//         let window = Arc::new(event_loop.create_window(window_attrs).unwrap());

//         let (device, queue, surface, config) =
//             pollster::block_on(connect_to_gpu(Arc::clone(&window))).unwrap();

//         let device = Arc::new(device);
//         let queue = Arc::new(queue);

//         let renderer = render::PathTracer::new(device.clone(), queue.clone());

//         self.app = Some(App {
//             window,
//             surface,
//             device,
//             queue,
//             config,
//         });
//     }

//     fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
//         match event {
//             WindowEvent::CloseRequested => {
//                 println!("The close button was pressed; stopping");
//                 event_loop.exit();
//             }
//             WindowEvent::RedrawRequested => {
//                 // Redraw the application.
//                 //
//                 // It's preferable for applications that do not render continuously to render in
//                 // this event rather than in AboutToWait, since rendering in here allows
//                 // the program to gracefully handle redraws requested by the OS.

//                 // Draw.

//                 // Queue a RedrawRequested event.
//                 //
//                 // You only need to call this if you've determined that you need to redraw in
//                 // applications which do not always need to. Applications that redraw continuously
//                 // can render here instead.
//                 self.app.as_ref().unwrap().window.request_redraw();
//             }
//             _ => (),
//         }
//     }
// }

// async fn connect_to_gpu(
//     window: Arc<Window>,
// ) -> Result<(
//     wgpu::Device,
//     wgpu::Queue,
//     wgpu::Surface<'static>,
//     wgpu::SurfaceConfiguration,
// )> {
//     use wgpu::TextureFormat::{Bgra8Unorm, Rgba8Unorm};

//     // Create an "instance" of wgpu. This is the entry-point to the API.
//     let instance = wgpu::Instance::default();

//     // Create a drawable "surface" that is associated with the window.
//     let surface = instance.create_surface(window.clone())?;

//     // Request a GPU that is compatible with the surface. If the system has multiple GPUs then
//     // pick the high performance one.
//     let adapter = instance
//         .request_adapter(&wgpu::RequestAdapterOptions {
//             power_preference: wgpu::PowerPreference::HighPerformance,
//             force_fallback_adapter: false,
//             compatible_surface: Some(&surface),
//         })
//         .await
//         .context("failed to find a compatible adapter")?;

//     // Connect to the GPU. "device" represents the connection to the GPU and allows us to create
//     // resources like buffers, textures, and pipelines. "queue" represents the command queue that
//     // we use to submit commands to the GPU.
//     let (device, queue) = adapter
//         .request_device(&wgpu::DeviceDescriptor::default())
//         .await
//         .context("failed to connect to the GPU")?;

//     // Configure the texture memory backing the surface. Our renderer will draw to a surface
//     // texture every frame.
//     let caps = surface.get_capabilities(&adapter);
//     let format = caps
//         .formats
//         .into_iter()
//         .find(|it| matches!(it, Rgba8Unorm | Bgra8Unorm))
//         .context("could not find preferred texture format (Rgba8Unorm or Bgra8Unorm)")?;
//     let size = window.inner_size();
//     let config = wgpu::SurfaceConfiguration {
//         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//         format,
//         width: size.width,
//         height: size.height,
//         present_mode: wgpu::PresentMode::AutoVsync,
//         alpha_mode: caps.alpha_modes[0],
//         view_formats: vec![],
//         desired_maximum_frame_latency: 3,
//     };
//     surface.configure(&device, &config);

//     Ok((device, queue, surface, config))
// }

// gpu
// fn main() -> Result<()> {
//     let event_loop = EventLoop::new()?;

//     let mut app = AppHandler::default();
//     event_loop.run_app(&mut app)?;

//     Ok(())
// }

// cpu
fn main() {
    match 2 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        _ => (),
    }
}

fn bouncing_spheres() {
    // world
    let mut world: HittableList = HittableList::default();

    let checker = Arc::new(CheckerTexture::from_color(
        0.32,
        Color {
            r: 0.2,
            g: 0.3,
            b: 0.1,
        },
        Color {
            r: 0.9,
            g: 0.9,
            b: 0.9,
        },
    ));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::from_tex(checker),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f64();
            let centre = Point3::new(
                a as f64 + 0.9 * rand_f64(),
                0.2,
                b as f64 + 0.9 * rand_f64(),
            );

            if (centre - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::from(Vec3::rand_vec3()) * Color::from(Vec3::rand_vec3());
                    let mat = Lambertian::new(albedo);
                    let centre2 = centre + Vec3::new(0.0, rand_f64_range(0.0, 0.5), 0.0);
                    world.add(Sphere::new_moving(centre, centre2, 0.2, mat));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::rand(0.5, 1.0);
                    let fuzz = rand_f64_range(0.0, 0.5);
                    let mat = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(centre, 0.2, mat));
                } else {
                    // glass
                    let mat = Dielectric::new(1.5);
                    world.add(Sphere::new(centre, 0.2, mat));
                };
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));

    let world = BVHNode::from_hittable_list(world);
    // world = HittableList::new(world_bvh);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    camera.init();

    camera.render(&world);
}

fn checkered_spheres() {
    let mut world: HittableList = HittableList::default();

    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::from_color(
        0.32,
        Color {
            r: 0.2,
            g: 0.3,
            b: 0.1,
        },
        Color {
            r: 0.9,
            g: 0.9,
            b: 0.9,
        },
    ));

    world.add(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::from_tex(Arc::clone(&checker)),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::from_tex(Arc::clone(&checker)),
    ));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    // camera.focus_dist = 10.0;
    camera.init();

    camera.render(&world);
}
