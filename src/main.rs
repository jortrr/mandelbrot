mod complex_plane;
mod complex;

use complex::Complex;
use minifb::{Key, Window, WindowOptions, MouseMode, MouseButton};

use crate::complex_plane::ComplexPlane;

/// Creates a 32-bit color. The encoding for each pixel is `0RGB`:
/// The upper 8-bits are ignored, the next 8-bits are for the red channel, the next 8-bits
/// afterwards for the green channel, and the lower 8-bits for the blue channel.
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

/// Converts a buffer index to a screen coordinate
fn index_to_point(index: usize, width: usize, height: usize) -> (usize, usize) {
    (index % width,  index / width)
}

/// Converts a screen coordinate to a buffer index
fn point_to_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn main() {
    // Window dimensions in pixels
    let width: usize = 1600;
    let height: usize = 1200;
    let aspect_ratio_w_h: f64 = width as f64 / height as f64;
    let aspect_ratio_h_w: f64 = height as f64 / width as f64;
    // Complex plane dimensions and increments
    let c = ComplexPlane::new(width,height);
    // Mandelbrot set parameters
    let max_iterations = 100;
    let orbit_radius = 2.0; //If z remains within the orbit_radius in max_iterations, we assume c does not tend to infinity
    // User interaction variables
    let mut mouse_down: bool = false; //Variable needed for mouse single-click behavior

    let gcd = num::integer::gcd(width,height); //Needed to compute the aspect ratio of the pixel plane
    println!("Pixel plane: size is {width}x{height} and aspect ratio is {}:{}",width/gcd,height/gcd);
    println!("Complex plane: R ∈ [{},{}] and C ∈ [{},{}]", c.min_x, c.max_x, c.min_y, c.max_y);

    // Create a new window
    let mut window = Window::new(
        "Mandelbrot set viewer",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Create a buffer to store pixel data
    let mut buffer: Vec<u32> = vec![0; width * height];

    for (i, pixel) in buffer.iter_mut().enumerate() {
        let point = index_to_point(i, width, height);
        //println!("i: {i}");
        //println!("Pixel: {:?}", point);
        let complex = c.complex_from_pixel_plane(point.0, point.1);
        //println!("C: {:?}", c);
        let iterations = iterate(complex, orbit_radius, max_iterations);
        //println!("iterations: {}", iterations);
        //println!();
        *pixel = from_u8_rgb(iterations, iterations, iterations);
    }

    let mut r: u8 = 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer to black
        //buffer.iter_mut().for_each(|pixel| *pixel = from_u8_rgb(r, g, b));

        // Draw a red pixel at (100, 100)
        //let pixel_index = 100 + 100 * width;
        //buffer[pixel_index] = 0xFF0000;

        // Update the window with the new buffer
        window.update_with_buffer(&buffer, width, height).unwrap();

        // Handle any window events
        //Handle any key events
        for key in window.get_keys_pressed(minifb::KeyRepeat::Yes) {
            println!("Key pressed: {:?}", key);
            match key {
                Key::Q => r = u8::wrapping_add(r, 1),
                Key::A => r = u8::wrapping_sub(r, 1),
                Key::W => g = u8::wrapping_add(g, 1),
                Key::S => g = u8::wrapping_sub(g, 1),
                Key::E => b = u8::wrapping_add(b, 1),
                Key::D => b = u8::wrapping_sub(b, 1),
                _ => (),
            }
            if vec![Key::Q, Key::A, Key::W, Key::S, Key::E, Key::D].contains(&key) {
                println!("(r: {r:0>3}, g: {g:0>3}, b: {b:0>3})");
            }
        }
        //Handle any mouse events
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            let mouse_down_now = window.get_mouse_down(MouseButton::Left);
            if mouse_down_now && !mouse_down {
                println!("({x}, {y})");
                let index = point_to_index(x as usize, y as usize, width);
                let iterations = buffer[index] & 0xFF;
                let complex = c.complex_from_pixel_plane(x as usize, y as usize);
                println!("{:?}",complex);
                println!("iterations: {}",iterations);
                println!();
               // buffer[screen_pos] = 0x00ffffff;
            }
            mouse_down = mouse_down_now;
        }
    }
}

/// Run the Mandelbrot set algorithm for a single Complex number
/// Returns the amount of iterations needed before Zn escapes to infinity
fn iterate(c: Complex, orbit_radius: f64, max_iterations: u8) -> u8 {
    let mut z = Complex::new(0.0, 0.0);
    let mut iterations: u8 = 0;
    for _ in 0..max_iterations {
        z = z.squared().add(&c);

        if z.abs() > orbit_radius {
            break;
        }
        iterations += 1;
        if iterations == u8::MAX {
            break;
        }
    }
    iterations
}
