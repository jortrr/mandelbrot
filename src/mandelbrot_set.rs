use crate::complex::Complex;

#[derive(Clone)]
pub struct MandelbrotSet {
    pub max_iterations: u32,
    ///If z remains within the orbit_radius in max_iterations, we assume c does not tend to infinity
    pub orbit_radius: f64,
}

impl MandelbrotSet {
    pub fn new(max_iterations: u32, orbit_radius: f64) -> MandelbrotSet {
        MandelbrotSet {
            max_iterations,
            orbit_radius,
        }
    }

    /// Run the Mandelbrot set algorithm for a single Complex number
    /// Returns the amount of iterations needed before Zn escapes to infinity
    pub fn iterate(&self, c: &Complex) -> u32 {
        let mut z = Complex::new(0.0, 0.0);
        let mut iterations: u32 = 0;
        let orbit_radius_squared = self.orbit_radius * self.orbit_radius;
        for _ in 0..self.max_iterations {
            z = z.squared().add(c);

            if (z.x * z.x + z.y * z.y) > orbit_radius_squared {
                //Optimization: square both sides of the Mandelbrot set function, saves us taking the square root
                break;
            }
            iterations += 1;
        }
        iterations
    }
}

impl std::fmt::Debug for MandelbrotSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "max_iterations = {}, orbit_radius = {}", self.max_iterations, self.orbit_radius)
    }
}
