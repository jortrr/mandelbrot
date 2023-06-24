use angular_units::Deg;
use num::traits::Pow;
use prisma::{Hsv, Rgb, FromColor};

#[derive(Clone, Copy)]
pub struct TrueColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl TrueColor {
    pub fn new(red: u8, green: u8, blue: u8) -> TrueColor { 
        TrueColor { red, green, blue } 
    }

    /// Creates a 32-bit color. The encoding for each pixel is `0RGB`:
    /// The upper 8-bits are ignored, the next 8-bits are for the red channel, the next 8-bits
    /// afterwards for the green channel, and the lower 8-bits for the blue channel.
    pub fn to_32_bit(&self) -> u32 {
        let (r, g, b) = (self.red as u32, self.green as u32, self.blue as u32);
        (r << 16) | (g << 8) | b
    }

    /// Creates a 24-bit truecolor struct from a normalized t ∈ [0, 1) </br>
    /// Maps a normalized value t to a continous 3D color space
    /// ![img](https://solarianprogrammer.com/images/2013/02/28/rgb_smooth.png)</br>
    /// Source: [Bernstein polynomial coloring](https://solarianprogrammer.com/2013/02/28/mandelbrot-set-cpp-11/)
    pub fn new_from_bernstein_polynomials(t: f64) -> TrueColor {
        let t = t.abs().min(0.999);
        let one_minus_t = 1.0-t;
        let red: f64 = 9.0 * one_minus_t * t.pow(3) * 255.0;
        let green: f64 = 15.0 * one_minus_t * t.pow(2) * 255.0;
        let blue: f64 = 8.5 * one_minus_t * t * 255.0;
        TrueColor { red: red as u8, green: green as u8, blue: blue as u8 }
    }

    pub fn new_from_hsv_colors(iterations: u32, max_iterations: u32) -> TrueColor {
        let hue = 0.3 * iterations as f64;
        let saturation = 0.8;
        let value: f64 = if iterations < max_iterations {1.0} else {0.0};
        let hue_degree = Deg(hue % 359.999);
        let hsv = Hsv::new(hue_degree,saturation,value);
        let rgb = Rgb::from_color(&hsv);
        let red = normalized_to_byte(rgb.red());
        let green = normalized_to_byte(rgb.green());
        let blue = normalized_to_byte(rgb.blue());
        TrueColor { red, green, blue }
    }

    ///Computes the average color of the given colors
    ///Can handle at most 2^24 colors
    pub fn average(colors: Vec<TrueColor>) -> TrueColor {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        for color in &colors {
            red += color.red as u32;
            green += color.green as u32;
            blue += color.blue as u32;
        }
        let divisor = colors.len().max(1) as u32;
        red /= divisor;
        green /= divisor;
        blue /= divisor;
        let red = red as u8;
        let blue = blue as u8;
        let green = green as u8;
        TrueColor { red, green, blue }
    }

}

///Maps a number t ∈ [0.0, 1.0] to a byte b ∈ [0, 255]
fn normalized_to_byte(t: f64) -> u8 {
    let t = t.abs().min(1.0);
    (t * 255.0) as u8 
}
