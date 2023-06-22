use crate::complex::Complex;

/// # Complex plane
/// In mathematics, the complex plane is the plane formed by the complex numbers, with a Cartesian coordinate system such that the x-axis, called the real axis, is formed by the real numbers, and the y-axis, called the imaginary axis, is formed by the imaginary numbers.
/// \
/// The complex plane allows a geometric interpretation of complex numbers. Under addition, they add like vectors. The multiplication of two complex numbers can be expressed more easily in polar coordinates—the magnitude or modulus of the product is the product of the two
///  absolute values, or moduli, and the angle or argument of the product is the sum of the two angles, or arguments. In particular, multiplication by a complex number of modulus 1 acts as a rotation.
/// ## Notational conventions
/// ### Complex numbers
/// In complex analysis, the complex numbers are customarily represented by the symbol z, which can be separated into its real (x) and imaginary (y) parts:
/// \
/// ```
/// z = x + iy
/// ```
/// for example: z = 4 + 5i, where x and y are real numbers, and i is the imaginary unit. In this customary notation the complex number z corresponds to the point (x, y) in the Cartesian plane.
/// \
/// In the Cartesian plane the point (x, y) can also be represented in polar coordinates as
/// ```
/// (x, y) = (rcosθ, rsinθ)  (r, θ)= (√(x^2 + y^2), arctan(y/x))
/// ```
/// ### Complex plane notation ℂ
/// Complex plane is denoted as ℂ.
#[derive(Clone)]
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
    // Pixel plane width and height
    width: usize,
    height: usize,
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
        ComplexPlane {
            min_x,
            max_x,
            length_x,
            min_y,
            max_y,
            length_y,
            increment_x,
            increment_y,
            width,
            height,
        }
    }

    /// Translate the Complex plane by adding x to min_x and max_x, and y to min_y and max_y
    pub fn translate(&mut self, x: f64, y: f64) {
        self.min_x += x;
        self.max_x += x;
        self.min_y += y;
        self.max_y += y;
    }

    /// Convert the point (x,y) in the pixel plane to the complex number a+bi in the complex plane
    pub fn complex_from_pixel_plane(&self, x: usize, y: usize) -> Complex {
        let x = self.min_x + x as f64 * self.increment_x;
        let y = -(self.min_y + y as f64 * self.increment_y); //Negate because math plane is bottom-top, and screen plane is top-bottom
        let c = Complex::new(x, y);
        c
    }

    /// Prints: "Complex plane: R ∈ [{},{}] and C ∈ [{},{}]",c.min_x, c.max_x, c.min_y, c.max_y
    pub fn print(&self) {
        println!("Complex plane:\tR ∈ [{},{}]", self.min_x, self.max_x);
        println!("\t\tC ∈ [{},{}]", self.min_y, self.max_y);
        println!(
            "\t\tCenter is {:?} and scale is {}",
            self.center(),
            self.get_scale()
        );
    }

    /// Resets the total translation and scaling applied to the Complex plane by the translate() and scale() functions
    pub fn reset(&mut self) {
        *self = ComplexPlane::new(self.width, self.height);
    }

    //Returns the total scale applied to the Complex plane
    pub fn get_scale(&self) -> f64 {
        let s = self.length_x / 2.5;
        return s;
    }

    /// Scale the complex plane, by multiplying the complex plane dimensions and increments by factor.
    /// If factor > 1.0: zoom out
    /// If factor < 1.0: zoom in
    pub fn scale(&mut self, factor: f64) {
        let center = self.center();
        self.min_x *= factor;
        self.max_x *= factor;
        self.min_y *= factor;
        self.max_y *= factor;
        self.length_x *= factor;
        self.length_y *= factor;
        self.increment_x *= factor;
        self.increment_y *= factor;
        self.set_center(center);
    }

    /// Returns the center of the Complex plane bounded by min_x, min_y, max_x, max_y
    pub fn center(&self) -> Complex {
        let x = self.min_x + (self.max_x - self.min_x) / 2.0;
        let y = -(self.min_y + (self.max_y - self.min_y) / 2.0); //Negate because math plane is bottom-top, and screen plane is top-bottom
        Complex::new(x, y)
    }

    /// Translate min_x,max_x,min_y,max_y so that center becomes the center of the Complex plane
    /// Returns the translation
    pub fn set_center(&mut self, center: Complex) -> Complex {
        let old = self.center();
        let mut translation = center.subtract(&old);
        translation.y = -translation.y; //Negate because the Complex plane and pixel plane are flipped
        println!("DEBUG set_center():");
        println!("\tcenter: {:?}", center);
        println!("\told: {:?}", old);
        println!("\ttranslation: {:?}", translation);
        self.translate(translation.x, translation.y);
        translation
    }

    /// Set the Complex plane at Center (x,y) at the given scale, where scale == 1 => max_x-min_x=2.5
    pub fn set_view(&mut self, x: f64, y: f64, scale: f64) {
        self.reset();
        self.set_center(Complex::new(x, y));
        self.scale(scale);
    }
}
