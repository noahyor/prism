use std::sync::Arc;

use crate::{bvh::BVHNode, camera::Camera, hit::Hittable};

pub struct Scene {
    pub camera: Arc<Camera>,
    pub root: Arc<BVHNode>,
}

impl Scene {
    pub fn new(camera: Camera, root: BVHNode) -> Self {
        Scene { camera: Arc::new(camera), root: Arc::new(root) }
    }

    pub fn objects(&self) -> usize {
        self.root.objects()
    }

    #[cfg(debug_assertions)]
    pub fn est_time(&self) -> usize {
        (0.0000002
            * self.camera.total_pixels() as f64
            * self.camera.samples_per_pixel as f64
            * self.camera.max_depth as f64) as usize
    }

    #[cfg(not(debug_assertions))]
    pub fn est_time(&self) -> usize {
        (0.00000008
            * self.camera.total_pixels() as f64
            * self.camera.samples_per_pixel as f64
            * self.camera.max_depth as f64) as usize
    }
}