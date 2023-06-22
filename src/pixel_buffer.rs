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

    /// Translate the complex plane in the `buffer` `rows` up and `columns` to the right.
    /// This operation is significantly less expensive than the render_box_render_complex_plane_into_buffer() function, as it does not rerender anything in the complex plane, it simply
    /// get rids of `rows.abs()` rows and `columns.abs()` columns, and moves the image rows to the right and columns up.
    /// Note: The removed rows and columns should be rerendered by the render_box_render_complex_plane_into_buffer() function.
    pub fn translate_buffer(&mut self, rows_up: i128, columns_right: i128) {
        //Iterate over the correct y's in the correct order
        let y_range : Vec<usize> = if rows_up > 0 {((rows_up as usize)..self.pixel_plane.height).rev().into_iter().collect()} else {(0..((self.pixel_plane.height as i128 + rows_up) as usize)).into_iter().collect()};
        //Iterate over the correct x's in the correct order
        let x_range : Vec<usize> = if columns_right > 0 {((columns_right as usize)..self.pixel_plane.width).rev().into_iter().collect()} else {(0..((self.pixel_plane.width as i128 + columns_right) as usize)).into_iter().collect()};

        for y in y_range {
            let other_y = (y as i128-rows_up) as usize;
            //println!("y: {y} and other_y: {other_y}");
            for x in &x_range {
                let other_x = (*x as i128 - columns_right) as usize;
                //println!("x: {} and other_x: {other_x}",*x);
                let index = self.point_to_index(*x, y);
                let other_index = self.point_to_index(other_x, other_y);
                self.buffer[index] = self.buffer[other_index];
            }
        }
    }
}
