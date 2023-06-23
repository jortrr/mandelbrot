use num::traits::Pow;

pub struct TrueColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

/// Creates a 24-bit truecolor struct from a normalized t ∈ [0, 1) </br>
/// Maps a normalized value t to a continous 3D color space
/// ![img](https://solarianprogrammer.com/images/2013/02/28/rgb_smooth.png)</br>
/// Source: [Bernstein polynomial coloring](https://solarianprogrammer.com/2013/02/28/mandelbrot-set-cpp-11/)
pub fn create_color_from_continuous_bernstein_polynomials(t: f64) -> TrueColor {
    let one_minus_t = 1.0-t;
    let red: f64 = 9.0 * one_minus_t * t.pow(3) * 255.0;
    let green: f64 = 15.0 * one_minus_t * t.pow(2) * 255.0;
    let blue: f64 = 8.5 * one_minus_t * t * 255.0;
    TrueColor { red: red as u8, green: green as u8, blue: blue as u8 }
}