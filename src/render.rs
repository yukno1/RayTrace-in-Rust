use std::sync::Arc;

pub struct PathTracer {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl PathTracer {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> PathTracer {
        device.on_uncaptured_error(Arc::new(|error| {
            panic!("Aborting due to an error: {}", error);
        }));

        // TODO: initialize GPU resources

        PathTracer { device, queue }
    }
}
