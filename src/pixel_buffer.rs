use self::pixel_plane::PixelPlane;

pub mod pixel_plane;

#[derive(Clone)]
pub struct PixelBuffer {
    pub pixel_plane: PixelPlane,
    pub buffer: Vec<u32>,
}

impl PixelBuffer {
    pub fn new(pixel_plane: PixelPlane) -> PixelBuffer {
        // Create a buffer to store pixel data
        let buffer: Vec<u32> = vec![0; pixel_plane.width * pixel_plane.height];
        PixelBuffer { pixel_plane, buffer}
    }

    /// Converts a buffer index to a screen coordinate
    pub fn index_to_point(&self, index: usize) -> (usize, usize) {
        (index % self.pixel_plane.width, index / self.pixel_plane.width)
    }

    /// Converts a screen coordinate to a buffer index
    pub fn point_to_index(&self, x: usize, y: usize) -> usize {
        y * self.pixel_plane.width + x
    }

    /// Returns the amount of Mandelbrot iterations at a given point inside the pixel plane
    pub fn iterations_at_point(&self, x: usize, y: usize, max_iterations: u32) -> u32 {
        let index = self.point_to_index(x, y);
        let pixel = self.buffer[index];
        let iterations = crate::iterations_from_hsv_pixel(pixel, max_iterations);
        iterations
    } 
}
