use crate::complex::Complex;

pub struct ComplexPlane {
    // Complex plane dimensions
    pub min_x: f64,
    pub max_x: f64,
    pub length_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub length_y: f64,
    // Complex plane increments
    pub increment_x: f64,
    pub increment_y: f64, 
    //Complex plane translations
    pub translate_x: f64,
    pub translate_y: f64,
}

impl ComplexPlane {
    /// Creates the dimensions of a Complex plane for viewing the Mandelbrot set based on the width and height of a screen in pixels.
    /// By default: Complex plane: R ∈ [-2,0.5] and C will be calculated to preserve proportionality.
    pub fn new(width: usize, height: usize) -> ComplexPlane {
        // Complex plane dimensions
        let min_x: f64 = -2.0;
        let max_x: f64 = 1.0 / 2.0;
        let length_x: f64 = max_x - min_x;
        let aspect_ratio_h_w: f64 = height as f64 / width as f64;
        let min_y: f64 = -(length_x * aspect_ratio_h_w / 2.0);
        let max_y: f64 = -min_y;
        let length_y: f64 = max_y - min_y;
        // Complex plane increments
        let increment_x: f64 = length_x / width as f64;
        let increment_y: f64 = length_y / height as f64;
        ComplexPlane { min_x, max_x, length_x, min_y, max_y, length_y, increment_x, increment_y, translate_x: 0.0, translate_y: 0.0}
    }

    /// Translate the Complex plane by adding x to min_x and max_x, and y to min_y and max_y
    pub fn translate(&mut self, x: f64, y: f64) {
        self.min_x += x;
        self.max_x += x;
        self.min_y += y;
        self.max_y += y;
        self.translate_x += x;
        self.translate_y += y;
    }

    /// Convert the point (x,y) in the pixel plane to the complex number a+bi in the complex plane
    pub fn complex_from_pixel_plane(&self, x: usize, y: usize) -> Complex {
        let x = self.min_x + x as f64 * self.increment_x;
        let y = -(self.min_y + y as f64 * self.increment_y); //Negate because math plane is bottom-top, and screen plane is top-bottom 
        let c = Complex::new(x,y);
        c
    }

    /// Prints: "Complex plane: R ∈ [{},{}] and C ∈ [{},{}]",c.min_x, c.max_x, c.min_y, c.max_y
    pub fn print(&self) {
        println!("Complex plane: R ∈ [{},{}] and C ∈ [{},{}]",self.min_x, self.max_x, self.min_y, self.max_y);
    }

    /// Resets the total translation applied to the Complex plane by the translate() function
    pub fn reset_translation(&mut self) {
        self.min_x -= self.translate_x;
        self.max_x -= self.translate_x;
        self.min_y -= self.translate_y;
        self.max_y -= self.translate_y;
    }
}