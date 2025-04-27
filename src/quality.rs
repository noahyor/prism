pub struct QualityOptions {
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub img_width: usize,
}

impl QualityOptions {
    pub fn new(samples: usize, max_depth: usize, img_width: usize) -> Self {
        Self {
            samples_per_pixel: samples,
            max_depth,
            img_width,
        }
    }

    pub const DEFAULT: QualityOptions = QualityOptions {samples_per_pixel: 100, max_depth: 50, img_width: 400};
}