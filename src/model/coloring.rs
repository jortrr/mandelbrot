use std::{fmt, str::FromStr};

use angular_units::Deg;
use num::traits::Pow;
use prisma::{FromColor, Hsv, Rgb};

#[derive(Debug, Clone, Copy)]
///A mapping from ColorChannelMapping -> RGB, the first character denotes the new red channel, the second character the new green channel,
/// the third character the new blue channel. </br>
/// E.g: Ok(ColorChannelMapping::BGR) means that red will get the value of blue, green the value of green, and blue the value of red:
/// B => R, G => G, R => B.
pub enum ColorChannelMapping {
    BBB,
    BBG,
    BBR,
    BGB,
    BGG,
    BGR,
    BRB,
    BRG,
    BRR,
    GBB,
    GBG,
    GBR,
    GGB,
    GGG,
    GGR,
    GRB,
    GRG,
    GRR,
    RBB,
    RBG,
    RBR,
    RGG,
    RGR,
    RRB,
    RRG,
    RRR,
    RGB,
}

impl ColorChannelMapping {
    pub fn new(r_g_b: &str) -> Result<ColorChannelMapping, String> {
        match &r_g_b.to_uppercase()[..] {
            "BBB" => Ok(ColorChannelMapping::BBB),
            "BBG" => Ok(ColorChannelMapping::BBG),
            "BBR" => Ok(ColorChannelMapping::BBR),
            "BGB" => Ok(ColorChannelMapping::BGB),
            "BGG" => Ok(ColorChannelMapping::BGG),
            "BGR" => Ok(ColorChannelMapping::BGR),
            "BRB" => Ok(ColorChannelMapping::BRB),
            "BRG" => Ok(ColorChannelMapping::BRG),
            "BRR" => Ok(ColorChannelMapping::BRR),
            "GBB" => Ok(ColorChannelMapping::GBB),
            "GBG" => Ok(ColorChannelMapping::GBG),
            "GBR" => Ok(ColorChannelMapping::GBR),
            "GGB" => Ok(ColorChannelMapping::GGB),
            "GGG" => Ok(ColorChannelMapping::GGG),
            "GGR" => Ok(ColorChannelMapping::GGR),
            "GRB" => Ok(ColorChannelMapping::GRB),
            "GRG" => Ok(ColorChannelMapping::GRG),
            "GRR" => Ok(ColorChannelMapping::GRR),
            "RBB" => Ok(ColorChannelMapping::RBB),
            "RBG" => Ok(ColorChannelMapping::RBG),
            "RBR" => Ok(ColorChannelMapping::RBR),
            "RGG" => Ok(ColorChannelMapping::RGG),
            "RGR" => Ok(ColorChannelMapping::RGR),
            "RRB" => Ok(ColorChannelMapping::RRB),
            "RRG" => Ok(ColorChannelMapping::RRG),
            "RRR" => Ok(ColorChannelMapping::RRR),
            "RGB" => Ok(ColorChannelMapping::RGB),
            _ => Err(String::from(
                "Invalid r_g_b string, should be a string of length three with characters 'R', 'G' or 'B'",
            )),
        }
    }
    ///Returns (x,y,z) where x,y,z ∈ {'R', 'G', 'B'}
    pub fn get_r_g_b_mapping(&self) -> (char, char, char) {
        let r_g_b = self.to_string().chars().collect::<Vec<char>>();
        (r_g_b[0], r_g_b[1], r_g_b[2])
    }
}

impl fmt::Display for ColorChannelMapping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl FromStr for ColorChannelMapping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ColorChannelMapping::new(s)
    }
}

#[derive(Clone, Copy)]
pub struct TrueColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl TrueColor {
    pub fn new(red: u8, green: u8, blue: u8) -> TrueColor {
        TrueColor { red, green, blue }
    }

    /// Creates a 32-bit color. The encoding for each pixel is `0RGB`:
    /// The upper 8-bits are ignored, the next 8-bits are for the red channel, the next 8-bits
    /// afterwards for the green channel, and the lower 8-bits for the blue channel.
    pub fn to_32_bit(&self, mapping: &ColorChannelMapping) -> u32 {
        let (r_map, g_map, b_map) = mapping.get_r_g_b_mapping();
        let (r, g, b) = (
            u32::from(self.get_color(r_map)),
            u32::from(self.get_color(g_map)),
            u32::from(self.get_color(b_map)),
        );
        (r << 16) | (g << 8) | b
    }

    pub fn get_color(&self, color: char) -> u8 {
        assert!(
            color == 'R' || color == 'G' || color == 'B',
            "Error: color should be equal to R, G, or B, color = {}",
            color
        );
        match color {
            'R' => self.red,
            'G' => self.green,
            'B' => self.blue,
            _ => 0,
        }
    }

    /// Creates a 24-bit truecolor struct from a normalized t ∈ [0, 1) </br>
    /// Maps a normalized value t to a continous 3D color space
    /// ![img](https://solarianprogrammer.com/images/2013/02/28/rgb_smooth.png)</br>
    /// Source: [Bernstein polynomial coloring](https://solarianprogrammer.com/2013/02/28/mandelbrot-set-cpp-11/)
    fn new_from_bernstein_polynomials_normalized(t: f64) -> TrueColor {
        let t = t.abs().min(0.999);
        let one_minus_t = 1.0 - t;
        let red: f64 = 9.0 * one_minus_t * t.pow(3) * 255.0;
        let green: f64 = 15.0 * one_minus_t * t.pow(2) * 255.0;
        let blue: f64 = 8.5 * one_minus_t * t * 255.0;
        TrueColor {
            red: red as u8,
            green: green as u8,
            blue: blue as u8,
        }
    }

    ///A `coloring_function`
    /// Creates a 24-bit truecolor struct from a normalized t = `iterations`/`max_iterations` ∈ [0, 1) </br>
    /// Maps a normalized value t to a continous 3D color space
    /// ![img](https://solarianprogrammer.com/images/2013/02/28/rgb_smooth.png)</br>
    /// Source: [Bernstein polynomial coloring](https://solarianprogrammer.com/2013/02/28/mandelbrot-set-cpp-11/)
    pub fn new_from_bernstein_polynomials(iterations: u32, max_iterations: u32) -> TrueColor {
        let t: f64 = f64::from(iterations) / f64::from(max_iterations);
        TrueColor::new_from_bernstein_polynomials_normalized(t)
    }

    ///A `coloring_function`
    pub fn new_from_hsv_colors(iterations: u32, max_iterations: u32) -> TrueColor {
        let hue = 0.3 * f64::from(iterations);
        let saturation = 1.0; //0.8;
        let value: f64 = if iterations < max_iterations { 1.0 } else { 0.0 };
        let hue_degree = Deg(hue % 359.999);
        let hsv = Hsv::new(hue_degree, saturation, value);
        let rgb = Rgb::from_color(&hsv);
        let red = normalized_to_byte(rgb.red());
        let green = normalized_to_byte(rgb.green());
        let blue = normalized_to_byte(rgb.blue());
        TrueColor { red, green, blue }
    }

    ///Computes the average color of the given colors
    ///Can handle at most 2^24 colors
    pub fn average(colors: &Vec<TrueColor>) -> TrueColor {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        for color in colors {
            red += u32::from(color.red);
            green += u32::from(color.green);
            blue += u32::from(color.blue);
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
    let byte = (t * 255.0) as i16;
    byte.unsigned_abs() as u8
}
