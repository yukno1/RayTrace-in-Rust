mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod quad;
mod ray;
mod rtw_image;
mod sphere;
mod texture;
mod transform;
mod utils;
mod vec3;
// mod render;

// cpu
use crate::{
    bvh::BVHNode,
    camera::Camera,
    color::Color,
    constant_medium::ConstantMedium,
    hittable_list::HittableList,
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    quad::{Quad, cube},
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture},
    transform::{RotateY, Translate},
    utils::{rand_f64, rand_f64_range},
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

/*
// gpu
use {
    anyhow::{Context, Result},
    winit::{
        application::ApplicationHandler,
        event::{Event, WindowEvent},
        event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
        window::{Window, WindowAttributes, WindowId},
    },
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct App {
    #[allow(unused)]
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    config: wgpu::SurfaceConfiguration,
}

#[derive(Default)]
struct AppHandler {
    app: Option<App>,
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // 恢复事件
        if self.app.is_some() {
            return;
        }

        let window_size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
        let window_attrs = WindowAttributes::default()
            .with_inner_size(window_size)
            .with_resizable(false)
            .with_title("GPU Path Tracer".to_string());
        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());

        let (device, queue, surface, config) =
            pollster::block_on(connect_to_gpu(Arc::clone(&window))).unwrap();

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let renderer = render::PathTracer::new(device.clone(), queue.clone());

        self.app = Some(App {
            window,
            surface,
            device,
            queue,
            config,
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.app.as_ref().unwrap().window.request_redraw();
            }
            _ => (),
        }
    }
}

async fn connect_to_gpu(
    window: Arc<Window>,
) -> Result<(
    wgpu::Device,
    wgpu::Queue,
    wgpu::Surface<'static>,
    wgpu::SurfaceConfiguration,
)> {
    use wgpu::TextureFormat::{Bgra8Unorm, Rgba8Unorm};

    // Create an "instance" of wgpu. This is the entry-point to the API.
    let instance = wgpu::Instance::default();

    // Create a drawable "surface" that is associated with the window.
    let surface = instance.create_surface(window.clone())?;

    // Request a GPU that is compatible with the surface. If the system has multiple GPUs then
    // pick the high performance one.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .context("failed to find a compatible adapter")?;

    // Connect to the GPU. "device" represents the connection to the GPU and allows us to create
    // resources like buffers, textures, and pipelines. "queue" represents the command queue that
    // we use to submit commands to the GPU.
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .context("failed to connect to the GPU")?;

    // Configure the texture memory backing the surface. Our renderer will draw to a surface
    // texture every frame.
    let caps = surface.get_capabilities(&adapter);
    let format = caps
        .formats
        .into_iter()
        .find(|it| matches!(it, Rgba8Unorm | Bgra8Unorm))
        .context("could not find preferred texture format (Rgba8Unorm or Bgra8Unorm)")?;
    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 3,
    };
    surface.configure(&device, &config);

    Ok((device, queue, surface, config))
}

gpu
fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut app = AppHandler::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
*/

// cpu
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
        Arc::new(Lambertian::from_tex(checker)),
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
                    let mat = Arc::new(Lambertian::new(albedo));
                    let centre2 = centre + Vec3::new(0.0, rand_f64_range(0.0, 0.5), 0.0);
                    world.add(Sphere::new_moving(centre, centre2, 0.2, mat));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::rand(0.5, 1.0);
                    let fuzz = rand_f64_range(0.0, 0.5);
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(centre, 0.2, mat));
                } else {
                    // glass
                    let mat = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(centre, 0.2, mat));
                };
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));

    let world = BVHNode::from_hittable_list(world);
    // world = HittableList::new(world_bvh);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.7, 0.8, 1.0);

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

    let lam_checker = Arc::new(Lambertian::from_tex(checker.clone()));
    world.add(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        lam_checker.clone(),
    ));
    world.add(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, lam_checker));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.7, 0.8, 1.0);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    // camera.focus_dist = 10.0;
    camera.init();

    camera.render(&world);
}

fn earth() {
    let mut world: HittableList = HittableList::default();

    let earth_tex = ImageTexture::new("earthmap.jpg");
    let earth_surface = Lambertian::from_tex(Arc::new(earth_tex));
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, Arc::new(earth_surface));
    world.add(globe);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.7, 0.8, 1.0);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    // camera.focus_dist = 10.0;
    camera.init();

    camera.render(&world);
}

fn perlin_spheres() {
    let mut world: HittableList = HittableList::default();

    let pertex = Arc::new(NoiseTexture::new(4.0));
    let lam_pertex = Arc::new(Lambertian::from_tex(pertex));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        lam_pertex.clone(),
    ));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, lam_pertex));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.7, 0.8, 1.0);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    // camera.focus_dist = 10.0;
    camera.init();

    camera.render(&world);
}

fn quads() {
    let mut world: HittableList = HittableList::default();

    // Materials
    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    world.add(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.7, 0.8, 1.0);

    camera.vfov = 80.0;
    camera.lookfrom = Point3::new(0.0, 0.0, 9.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    camera.init();

    camera.render(&world);
}

fn simple_light() {
    let mut world: HittableList = HittableList::default();

    let pertex = Arc::new(NoiseTexture::new(4.0));
    let lam_pertex = Arc::new(Lambertian::from_tex(pertex.clone()));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        lam_pertex.clone(),
    ));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, lam_pertex));

    let difflight: Arc<dyn Material> =
        Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    ));
    world.add(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, difflight));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;

    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(26.0, 3.0, 6.0);
    camera.lookat = Point3::new(0.0, 2.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;
    // camera.focus_dist = 10.0;
    camera.init();

    camera.render(&world);
}

fn cornell_box() {
    let mut world: HittableList = HittableList::default();

    // Materials
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    // Quads
    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    let box1 = quad::cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::rotate_y(box1, 15.0);
    let box1 = Translate::translate(box1, Vec3::new(265.0, 0.0, 295.0));
    world.add(box1);

    let box2 = quad::cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = RotateY::rotate_y(box2, -18.0);
    let box2 = Translate::translate(box2, Vec3::new(130.0, 0.0, 65.0));
    world.add(box2);

    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 200;
    camera.max_depth = 50;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.vfov = 40.0;
    camera.lookfrom = Point3::new(278.0, 278.0, -800.0);
    camera.lookat = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    camera.init();

    camera.render(&world);
}

fn cornell_smoke() {
    let mut world: HittableList = HittableList::default();

    // Materials
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));

    // Quads
    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    let box1 = quad::cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::rotate_y(box1, 15.0);
    let box1 = Translate::translate(box1, Vec3::new(265.0, 0.0, 295.0));
    world.add(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    ));

    let box2 = quad::cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = RotateY::rotate_y(box2, -18.0);
    let box2 = Translate::translate(box2, Vec3::new(130.0, 0.0, 65.0));
    world.add(ConstantMedium::from_color(
        box2,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    ));

    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = 600;
    camera.samples_per_pixel = 200;
    camera.max_depth = 50;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.vfov = 40.0;
    camera.lookfrom = Point3::new(278.0, 278.0, -800.0);
    camera.lookat = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    camera.init();

    camera.render(&world);
}

fn final_sccene(image_width: usize, samples_per_pixel: usize, max_depth: usize) {
    let mut boxes1 = HittableList::default();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand_f64_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(cube(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world: HittableList = HittableList::default();
    world.add(BVHNode::from_hittable_list(boxes1));

    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    ));

    let centre1 = Point3::new(400.0, 400.0, 400.0);
    let centre2 = centre1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_mat = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Sphere::new_moving(centre1, centre2, 50.0, sphere_mat));

    world.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    ));

    let boundary = Sphere::new(
        Point3::new(360.0, 150.0, 45.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    );
    world.add(boundary);
    let boundary = Sphere::new(
        Point3::new(360.0, 150.0, 45.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    );
    world.add(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));
    let boundary = Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    );
    world.add(ConstantMedium::from_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    ));

    let emat = Lambertian::from_tex(Arc::new(ImageTexture::new("earthmap.jpg")));
    world.add(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        Arc::new(emat),
    ));
    let pertex = Arc::new(NoiseTexture::new(0.2));
    world.add(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::from_tex(pertex)),
    ));

    let mut boxes2 = HittableList::default();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Sphere::new(
            Point3::rand_vec3_range(0.0, 165.0),
            10.0,
            white.clone(),
        ));
    }
    world.add(Translate::translate(
        RotateY::rotate_y(BVHNode::from_hittable_list(boxes2), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    let mut camera = Camera::new();

    camera.aspect_ratio = 1.0;
    camera.image_width = image_width;
    camera.samples_per_pixel = samples_per_pixel;
    camera.max_depth = max_depth;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.vfov = 40.0;
    camera.lookfrom = Point3::new(478.0, 278.0, -600.0);
    camera.lookat = Point3::new(278.0, 278.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    camera.init();

    camera.render_para(&world);
}

fn main() {
    match 9 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_sccene(800, 10000, 40),
        _ => final_sccene(400, 250, 4),
    }
}
