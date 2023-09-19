#[derive(Clone)]
pub struct PixelPlane {
    pub width: usize,
    pub height: usize,
    aspect_ratio_width: usize,
    aspect_ratio_height: usize,
}

impl PixelPlane {
    pub fn new(width: usize, height: usize) -> PixelPlane {
        let gcd = num::integer::gcd(width, height); //Needed to compute the aspect ratio of the pixel plane
        PixelPlane {
            width,
            height,
            aspect_ratio_width: width / gcd,
            aspect_ratio_height: height / gcd,
        }
    }

    /// Prints: "Pixel plane: size is {}x{} and aspect ratio is {}:{}", `self.width`, `self.height`, `self.aspect_ratio_width`, `self.aspect_ratio_height`)
    pub fn print(&self) {
        println!(
            "Pixel plane: size is {}x{} and aspect ratio is {}:{}",
            self.width, self.height, self.aspect_ratio_width, self.aspect_ratio_height
        );
    }
}
