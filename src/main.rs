mod complex;
mod complex_plane;

use angular_units::Deg;
use complex::Complex;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use prisma::{Hsv, Rgb, FromColor, channel::AngularChannel};

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
    (index % width, index / width)
}

/// Converts a screen coordinate to a buffer index
fn point_to_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

/// Get the amount of Mandelbrot iterations from a HSV colored pixel
fn iterations_from_hsv_pixel(pixel: u32, max_iterations: u8) -> u8 {
    let r = (pixel >> 16) & 0xFF;
    let g = (pixel >> 8) & 0xFF;
    let b = pixel & 0xFF;
    let rgb = Rgb::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
    let hsv: Hsv<f64, _> = Hsv::from_color(&rgb);
    let hue: Deg<f64> = hsv.hue();
    let value = hsv.value();
    if value == 0.0 { //If value == 0.0, the pixel is an element of the Mandelbrot set
        return max_iterations;
    }
    let iterations: u8 = (max_iterations as f64 * (hue.0 / 359.0) as f64) as u8; 
    iterations
}

fn main() {
    // Window dimensions in pixels
    let width: usize = 1600;
    let height: usize = 1200;
    let aspect_ratio_w_h: f64 = width as f64 / height as f64;
    let aspect_ratio_h_w: f64 = height as f64 / width as f64;
    // Complex plane dimensions and increments
    let mut c = ComplexPlane::new(width, height);
    // Mandelbrot set parameters
    let max_iterations = 255;
    let orbit_radius = 2.0; //If z remains within the orbit_radius in max_iterations, we assume c does not tend to infinity
    // User interaction variables
    let mut mouse_down: bool = false; //Variable needed for mouse single-click behavior
    let mut translation_amount: u8 = 1; //Variable determining the amount of rows and columns are translated by pressing the 4 arrow keys

    let gcd = num::integer::gcd(width, height); //Needed to compute the aspect ratio of the pixel plane
    println!("Pixel plane: size is {width}x{height} and aspect ratio is {}:{}",width / gcd,height / gcd);
    c.print();
    println!("Mandelbrot set parameters: max. iterations is {} and orbit radius is {}", max_iterations, orbit_radius);
    println!();

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

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    render_complex_plane_into_buffer(&mut buffer, &c, width, height, orbit_radius, max_iterations);

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
                Key::Up => {c.translate(0.0, c.increment_y * translation_amount as f64); translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, -(translation_amount as i128), 0, orbit_radius, max_iterations)},
                Key::Down => {c.translate(0.0, -c.increment_y  * translation_amount as f64); translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, translation_amount as i128, 0, orbit_radius, max_iterations)},
                Key::Left => {c.translate(c.increment_x  * translation_amount as f64, 0.0); translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, 0, -(translation_amount as i128), orbit_radius, max_iterations);},
                Key::Right => {c.translate(-c.increment_x  * translation_amount as f64, 0.0); translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, 0, translation_amount as i128, orbit_radius, max_iterations);},
                Key::R => {c.reset_translation();render_complex_plane_into_buffer(&mut buffer, &c, width, height, orbit_radius, max_iterations);},
                Key::NumPadPlus => if translation_amount < u8::MAX { translation_amount += 1;},
                Key::NumPadMinus => if translation_amount > 1 { translation_amount -= 1; },
                _ => (),
            }
            if vec![Key::Q, Key::A, Key::W, Key::S, Key::E, Key::D].contains(&key) {
                println!("(r: {r:0>3}, g: {g:0>3}, b: {b:0>3})");
            }
            if vec![Key::Up, Key::Down, Key::Left, Key::Right, Key::R].contains(&key) {
               //render_complex_plane_into_buffer(&mut buffer, &c, width, height, orbit_radius, max_iterations);
               c.print();
            }
            if vec![Key::NumPadPlus, Key::NumPadMinus].contains(&key) {
                println!("translation_amount: {}", translation_amount);
            }
            println!();
        }
        //Handle any mouse events
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            let mouse_down_now = window.get_mouse_down(MouseButton::Left);
            if mouse_down_now && !mouse_down {
                println!("({x}, {y})");
                let index = point_to_index(x as usize, y as usize, width);
                let pixel = buffer[index];
                let iterations = iterations_from_hsv_pixel(pixel, max_iterations);
                let complex = c.complex_from_pixel_plane(x as usize, y as usize);
                println!("{:?}", complex);
                println!("iterations: {}", iterations);
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

/// Render the Complex plane c into the 32-bit pixel buffer by applying the Mandelbrot formula iteratively to every Complex point mapped to a pixel in the buffer. 
/// The buffer should have a size of width*height.
/// orbit_radius determines when Zn is considered to have gone to infinity.
/// max_iterations concerns the maximum amount of times the Mandelbrot formula will be applied to each Complex number.
/// Note: This function is computationally intensive, and should not be used for translations
fn render_complex_plane_into_buffer(buffer: &mut Vec<u32>, c: &ComplexPlane, width: usize, height: usize, orbit_radius: f64, max_iterations: u8) {
    render_box_render_complex_plane_into_buffer(buffer, c, width, height, orbit_radius, max_iterations, 0, width, 0, height);
}

/// Render the Complex plane c into the 32-bit pixel buffer by applying the Mandelbrot formula iteratively to every Complex point mapped to a pixel in the buffer. 
/// The buffer should have a size of width*height.
/// Only renders Pixels inside the render box denoted by render_min_x, render_max_x, render_min_y, render_max_y
/// orbit_radius determines when Zn is considered to have gone to infinity.
/// max_iterations concerns the maximum amount of times the Mandelbrot formula will be applied to each Complex number.
/// Note: This function is computationally intensive, and should not be used for translations
fn render_box_render_complex_plane_into_buffer(buffer: &mut Vec<u32>, c: &ComplexPlane, width: usize, height: usize, orbit_radius: f64, max_iterations: u8, render_min_x: usize, render_max_x: usize, render_min_y: usize, render_max_y: usize) {
    println!("render_box: ({},{}) -> ({},{})",render_min_x,render_min_y,render_max_x,render_max_y);
    for (i, pixel) in buffer.iter_mut().enumerate() {
        let point = index_to_point(i, width, height);
        if point.0 < render_min_x || point.0 > render_max_x || point.1 < render_min_y || point.1 > render_max_y {
            continue; //Do not render Pixel points outside of the render box
        }
        //println!("i: {i}");
        //println!("Pixel: {:?}", point);
        let complex = c.complex_from_pixel_plane(point.0, point.1);
        //println!("C: {:?}", c);
        let iterations = iterate(complex, orbit_radius, max_iterations);
        //println!("iterations: {}", iterations);
        //println!();
        let hue: f64 = 359.0 * (iterations as f64 / max_iterations as f64);
        let value: f64 = if iterations < max_iterations {1.0} else {0.0};
        let hsv = Hsv::new(Deg(hue),1.0,value);
        let rgb = Rgb::from_color(&hsv);
        //println!("rgb: {:?}", rgb);
        *pixel = from_u8_rgb((rgb.red() * 255.0) as u8, (rgb.green() * 255.0) as u8, (rgb.blue() * 255.0) as u8);
    }
}

/// Translate the complex plane in the `buffer` `rows` to the right and `columns` up.
/// This operation is significantly less expensive than the render_box_render_complex_plane_into_buffer() function, as it does not rerender anything in the complex plane, it simply
/// get rids of `rows.abs()` rows and `columns.abs()` columns, and moves the image rows to the right and columns up.
/// Note: The removed rows and columns should be rerendered by the render_box_render_complex_plane_into_buffer() function.
fn translate_complex_plane_buffer(buffer: &mut Vec<u32>, width: usize, height: usize, rows: i128, columns: i128) {
    //Iterate over the correct y's in the correct order
    let y_range : Vec<usize> = if rows > 0 {((rows as usize)..height).rev().into_iter().collect()} else {(0..((height as i128 + rows) as usize)).into_iter().collect()};
    //Iterate over the correct x's in the correct order
    let x_range : Vec<usize> = if columns > 0 {((columns as usize)..width).rev().into_iter().collect()} else {(0..((width as i128 + columns) as usize)).into_iter().collect()};

    for y in y_range {
        let other_y = (y as i128-rows) as usize;
        //println!("y: {y} and other_y: {other_y}");
        for x in &x_range {
            let other_x = (*x as i128 - columns) as usize;
            //println!("x: {} and other_x: {other_x}",*x);
            buffer[point_to_index(*x, y, width)] = buffer[point_to_index(other_x, other_y, width)];
        }
    }
}

fn translate_and_render_complex_plane_buffer(buffer: &mut Vec<u32>, c: &ComplexPlane, width: usize, height: usize, rows: i128, columns: i128, orbit_radius: f64, max_iterations: u8) {
    let max_x: usize = if columns > 0 {columns as usize} else {width-1};
    let max_y: usize = if rows > 0 {rows as usize} else {height-1};
    translate_complex_plane_buffer(buffer, width, height, rows, columns);
    if rows == 0 {
        render_box_render_complex_plane_into_buffer(buffer, c, width, height, orbit_radius, max_iterations, (max_x as i128-columns.abs()) as usize, max_x, 0, height);
    }
    else if columns == 0 {
        render_box_render_complex_plane_into_buffer(buffer, c, width, height, orbit_radius, max_iterations, 0, width, (max_y as i128 -rows.abs()) as usize, max_y);
    } else {
        println!("ERROR: translate_and_render_complex_plane_buffer() requires that rows == 0 || columns == 0");
    }
}