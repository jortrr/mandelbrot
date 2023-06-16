use std::error::Error;
use std::{time::Instant, thread, sync::{Mutex, Arc}};

use angular_units::Deg;
use complex::Complex;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use prisma::{Hsv, Rgb, FromColor};
use num_cpus;

use crate::complex_plane::ComplexPlane;
use crate::pixel_buffer::PixelBuffer;
use crate::pixel_buffer::pixel_plane::PixelPlane;

mod complex_plane;
mod complex;
mod pixel_buffer;

pub struct Config {
    // Window dimensions in pixels
    pub width: usize,
    pub height: usize,
    // Mandelbrot set parameters
    pub max_iterations: u32,
    pub orbit_radius: f64,      //If z remains within the orbit_radius in max_iterations, we assume c does not tend to infinity
}


impl Config {
    /// Parse the command line arguments from e.g. env::args() in the following format
    /// ```
    /// cargo run -- width height max_iterations
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        args.next(); //Skip the first argument as it is the name of the executable

        let width = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("no width argument given")),
        };

        let width = match width.parse::<usize>() {
            Ok(val) => val,
            Err(err) => return Err(err.to_string() + &String::from(" for width")),
        };

        let height = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("no height argument given")),
        };

        let height = match height.parse::<usize>() {
            Ok(val) => val,
            Err(err) => return Err(err.to_string() + &String::from(" for height")),
        };

        let max_iterations = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("no max_iterations argument given")),
        };

        let max_iterations = match max_iterations.parse::<u32>() {
            Ok(val) => val,
            Err(err) => return Err(err.to_string() + &String::from(" for max_iterations")),
        };

        Ok(Config {width, height, max_iterations, orbit_radius: 2.0})
    }
}

///Holds all the logic currently in the main function that isn't involved with setting up configuration or handling errors, to make `main` concise and
///easy to verify by inspection
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Complex plane dimensions and increments
    let mut c = ComplexPlane::new(config.width, config.height);
    // Pixel plane and buffer
    let mut p = PixelBuffer::new(PixelPlane::new(config.width, config.height));
    // User interaction variables
    let mut mouse_down: bool = false; //Variable needed for mouse single-click behavior
    let mut translation_amount: u8 = 10; //Variable determining the amount of rows and columns are translated by pressing the 4 arrow keys
    let mut scale_numerator: f64 = 9.0; //Variable denoting the user scaling speed; the lower this value, the more aggressive the zooming will become
    let scale_denominator: f64 = 10.0;
    //Mandelbrot coloring variables
    //let mut hue_offset: f64 = 0.0;
    //Multithreading variables
    let amount_of_threads = num_cpus::get(); //Amount of CPU threads to use

    p.pixel_plane.print();
    c.print();
    println!("Mandelbrot set parameters: max. iterations is {} and orbit radius is {}", config.max_iterations, config.orbit_radius);
    println!("Amount of CPU threads that will be used for rendering: {}", amount_of_threads);

    // Create a new window
    let mut window = Window::new(
        "Mandelbrot set viewer",
        config.width,
        config.height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    render_complex_plane_into_buffer(&mut p, &c, config.orbit_radius, config.max_iterations);
    println!();

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer to black
        //buffer.iter_mut().for_each(|pixel| *pixel = from_u8_rgb(r, g, b));
        
        // Update the window with the new buffer
        window.update_with_buffer(&p.buffer, config.width, config.height).unwrap();
        //change_hue_of_buffer(&mut buffer, 1.0);

        // Handle any window events
        // Handle any key events
        for key in window.get_keys_pressed(minifb::KeyRepeat::No) {
            println!("Key pressed: {:?}", key);
            match key {
                Key::Up => {c.translate(0.0, -c.increment_y * translation_amount as f64); translate_and_render_complex_plane_buffer(&mut p, &c, translation_amount as i128, 0, config.orbit_radius, config.max_iterations)},
                Key::Down => {c.translate(0.0, c.increment_y  * translation_amount as f64); translate_and_render_complex_plane_buffer(&mut p, &c,  -(translation_amount as i128), 0, config.orbit_radius, config.max_iterations)},
                Key::Left => {c.translate(-c.increment_x  * translation_amount as f64, 0.0); translate_and_render_complex_plane_buffer(&mut p, &c,  0, translation_amount as i128, config.orbit_radius, config.max_iterations);},
                Key::Right => {c.translate(c.increment_x  * translation_amount as f64, 0.0); translate_and_render_complex_plane_buffer(&mut p, &c,  0, -(translation_amount as i128), config.orbit_radius, config.max_iterations);},
                Key::R => c.reset(),
                Key::NumPadPlus => if translation_amount < u8::MAX { translation_amount += 1;},
                Key::NumPadMinus => if translation_amount > 1 { translation_amount -= 1; },
                Key::NumPadSlash => if scale_numerator > 1.0 { scale_numerator -= 1.0;},
                Key::NumPadAsterisk => if scale_numerator < 9.0 {scale_numerator += 1.0;},
                Key::LeftBracket => c.scale(scale_numerator/scale_denominator),
                Key::RightBracket => c.scale(scale_denominator/scale_numerator),
                Key::C => println!("Center: {:?}, scale: {:?}", c.center(), c.get_scale()),
                Key::Key1 => c.set_view(-0.6604166666666667, 0.4437500000000001, 0.1),
                Key::Key2 => c.set_view(-1.0591666666666668, 0.2629166666666668, 0.01),
                Key::Key3 => c.set_view(-0.4624999999999999, 0.55, 0.1),
                Key::Key4 => c.set_view(-0.46395833333333325, 0.5531250000000001, 0.03),
                Key::Key5 => c.set_view(-0.4375218333333333, 0.5632133750000003, 0.00002000000000000002),
                Key::Key6 => c.set_view(-0.7498100000000001, -0.020300000000000054, 0.00006400000000000002),
                _ => (),
            }
            if vec![Key::NumPadPlus, Key::NumPadMinus].contains(&key) {
                println!("translation_amount: {}", translation_amount);
            }
            if vec![Key::NumPadSlash,Key::NumPadAsterisk].contains(&key) {
                println!("scale factor: {}/{}",scale_numerator,scale_denominator);
            }
            if vec![Key::R,Key::Key1, Key::Key2, Key::Key3, Key::Key4, Key::Key5, Key::Key6].contains(&key) {
                render_complex_plane_into_buffer(&mut p, &c, config.orbit_radius, config.max_iterations);
                c.print();
            }
            if vec![Key::LeftBracket, Key::RightBracket].contains(&key) {
                render_complex_plane_into_buffer(&mut p, &c, config.orbit_radius, config.max_iterations);
                c.print();
            }
            if vec![Key::Up, Key::Down, Key::Left, Key::Right].contains(&key) {
                c.print();
             }
            println!();
        }
        //Handle any mouse events
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            let x: usize = x as usize;
            let y: usize = y as usize;
            let mouse_down_now = window.get_mouse_down(MouseButton::Left);
            if mouse_down_now && !mouse_down {
                println!("({x}, {y})");
                let iterations = p.iterations_at_point(x, y, config.max_iterations);
                let complex = c.complex_from_pixel_plane(x, y);
                println!("{:?}", complex);
                println!("iterations: {}", iterations);
                println!();
            }
            if window.get_mouse_down(MouseButton::Right) {
                println!("({x}, {y})");
                let complex = c.complex_from_pixel_plane(x, y);
                println!("{:?}", complex);
                c.set_center(complex);
                println!("Center: {:?}", c.center());
                //translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, 0, -(t.a/c.increment_x) as i128, orbit_radius, max_iterations);
                //translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, -(t.b/c.increment_y) as i128, 0, orbit_radius, max_iterations);
                render_complex_plane_into_buffer(&mut p, &c, config.orbit_radius, config.max_iterations);
                c.print();
                println!();
            }
            mouse_down = mouse_down_now;
        }
    }

    Ok(())
}


/// Run the Mandelbrot set algorithm for a single Complex number
/// Returns the amount of iterations needed before Zn escapes to infinity
fn iterate(c: Complex, orbit_radius: f64, max_iterations: u32) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    let mut iterations: u32 = 0;
    let orbit_radius_squared = orbit_radius*orbit_radius;
    for _ in 0..max_iterations {
        z = z.squared().add(&c);

        if (z.x * z.x) + (z.y * z.y) > orbit_radius_squared { //Optimization: square both sides of the Mandelbrot set function, saves us taking the square root
            break;
        }
        iterations += 1;
    }
    iterations
}

/// Render the Complex plane c into the 32-bit pixel buffer by applying the Mandelbrot formula iteratively to every Complex point mapped to a pixel in the buffer. 
/// The buffer should have a size of width*height.
/// orbit_radius determines when Zn is considered to have gone to infinity.
/// max_iterations concerns the maximum amount of times the Mandelbrot formula will be applied to each Complex number.
/// Note: This function is computationally intensive, and should not be used for translations
fn render_complex_plane_into_buffer(p: &mut PixelBuffer, c: &ComplexPlane, orbit_radius: f64, max_iterations: u32) {
    render_box_render_complex_plane_into_buffer(p, c, orbit_radius, max_iterations, 0, p.pixel_plane.width, 0, p.pixel_plane.height);
}

/// Render the Complex plane c into the 32-bit pixel buffer by applying the Mandelbrot formula iteratively to every Complex point mapped to a pixel in the buffer. 
/// The buffer should have a size of width*height.
/// Only renders Pixels inside the render box denoted by render_min_x, render_max_x, render_min_y, render_max_y
/// orbit_radius determines when Zn is considered to have gone to infinity.
/// max_iterations concerns the maximum amount of times the Mandelbrot formula will be applied to each Complex number.
/// Note: This function is computationally intensive, and should not be used for translations
/// Note: This function is multithreaded
fn render_box_render_complex_plane_into_buffer(p: &mut PixelBuffer, c: &ComplexPlane, orbit_radius: f64, max_iterations: u32, render_min_x: usize, render_max_x: usize, render_min_y: usize, render_max_y: usize) {
    let time = benchmark_start();
    println!("render_box: ({},{}) -> ({},{}) {{{} pixels}}",render_min_x,render_min_y,render_max_x,render_max_y ,(render_max_x-render_min_x)*(render_max_y-render_min_y));
    let chunk_size = p.buffer.len()/p.pixel_plane.height;
    let chunks: Vec<Vec<u32>> = p.buffer.chunks(chunk_size).map(|c| c.to_owned()).collect();
    let chunks_len = chunks.len();
    println!("chunks.len(): {}", chunks.len());
    let mut handles = Vec::new();
    let amount_of_threads = num_cpus::get(); //Amount of CPU threads to use
    let global_mutex = Arc::new(Mutex::new(0));

    for thread_id in 0..amount_of_threads {
        let plane = (*c).clone();
        let buf = chunks.clone();
        let thread_mutex = Arc::clone(&global_mutex);
        let pixel_buffer = (*p).clone();

        let handle = thread::spawn(move || {
            let mut thread_chunks = Vec::new();

            loop {
                let mut data = thread_mutex.lock().unwrap();
                let current_chunk = *data;
                *data+=1;
                drop(data);
                if current_chunk >= chunks_len {
                    return thread_chunks;
                }
                println!("Thread[{}] takes chunk[{}]", thread_id, current_chunk);
            
                let chunk_start = chunk_size * current_chunk;
                let mut chunk = buf[current_chunk].clone();

                
                for (i, pixel) in chunk.iter_mut().enumerate() {
                    let point = pixel_buffer.index_to_point(i + chunk_start);
                    if point.0 < render_min_x || point.0 > render_max_x || point.1 < render_min_y || point.1 > render_max_y {
                        continue; //Do not render Pixel points outside of the render box
                    }
                    //println!("i: {i}");
                    //println!("Pixel: {:?}", point);
                    //let complex = Complex::new(0.0,0.0);//
                    let complex = plane.complex_from_pixel_plane(point.0, point.1);
                    //println!("C: {:?}", c);
                    let iterations = iterate(complex, orbit_radius, max_iterations);
                    //println!("iterations: {}", iterations);
                    //println!();
                    let hue: f64 = 359.0 * (iterations as f64 / max_iterations as f64);
                    let value: f64 = if iterations < max_iterations {1.0} else {0.0};
                    let hsv = Hsv::new(Deg(hue % 359.0),1.0,value);
                    let rgb = Rgb::from_color(&hsv);
                    //println!("rgb: {:?}", rgb);
                    *pixel = from_u8_rgb((rgb.red() * 255.0) as u8, (rgb.green() * 255.0) as u8, (rgb.blue() * 255.0) as u8);
                }
                thread_chunks.push((current_chunk, chunk.clone()));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let thread_chunks = handle.join().unwrap();
        for (i, chunk) in thread_chunks{
            let mut index = i*p.pixel_plane.width;
            for pixel in chunk {
                p.buffer[index] = pixel;
                index+=1;
            }
        }
    }
    benchmark("render_box_render_complex_plane_into_buffer()", time);
}

/// Translate the complex plane in the `buffer` `rows` to the right and `columns` up.
/// This operation is significantly less expensive than the render_box_render_complex_plane_into_buffer() function, as it does not rerender anything in the complex plane, it simply
/// get rids of `rows.abs()` rows and `columns.abs()` columns, and moves the image rows to the right and columns up.
/// Note: The removed rows and columns should be rerendered by the render_box_render_complex_plane_into_buffer() function.
fn translate_complex_plane_buffer(p: &mut PixelBuffer, rows: i128, columns: i128) {
    //Iterate over the correct y's in the correct order
    let y_range : Vec<usize> = if rows > 0 {((rows as usize)..p.pixel_plane.height).rev().into_iter().collect()} else {(0..((p.pixel_plane.height as i128 + rows) as usize)).into_iter().collect()};
    //Iterate over the correct x's in the correct order
    let x_range : Vec<usize> = if columns > 0 {((columns as usize)..p.pixel_plane.width).rev().into_iter().collect()} else {(0..((p.pixel_plane.width as i128 + columns) as usize)).into_iter().collect()};

    for y in y_range {
        let other_y = (y as i128-rows) as usize;
        //println!("y: {y} and other_y: {other_y}");
        for x in &x_range {
            let other_x = (*x as i128 - columns) as usize;
            //println!("x: {} and other_x: {other_x}",*x);
            let index = p.point_to_index(*x, y);
            let other_index = p.point_to_index(other_x, other_y);
            p.buffer[index] = p.buffer[other_index];
        }
    }
}

fn translate_and_render_complex_plane_buffer(p: &mut PixelBuffer, c: &ComplexPlane, rows: i128, columns: i128, orbit_radius: f64, max_iterations: u32) {
    println!("rows: {}, columns: {}",rows, columns);
    let max_x: usize = if columns > 0 {columns as usize} else {p.pixel_plane.width-1};
    let max_y: usize = if rows > 0 {rows as usize} else {p.pixel_plane.height-1};
    translate_complex_plane_buffer(p, rows, columns);
    if rows == 0 {
        render_box_render_complex_plane_into_buffer(p, c, orbit_radius, max_iterations, (max_x as i128-columns.abs()) as usize, max_x, 0, p.pixel_plane.height);
    }
    else if columns == 0 {
        render_box_render_complex_plane_into_buffer(p, c, orbit_radius, max_iterations, 0, p.pixel_plane.width, (max_y as i128 -rows.abs()) as usize, max_y);
    } else {
        println!("ERROR: translate_and_render_complex_plane_buffer() requires that rows == 0 || columns == 0");
    }
}

fn benchmark_start() -> Instant {
    Instant::now()
}

fn benchmark(function: &str,time: Instant) {
    println!("[Benchmark] {}: {:.2?}",function, time.elapsed());
}


/// Creates a 32-bit color. The encoding for each pixel is `0RGB`:
/// The upper 8-bits are ignored, the next 8-bits are for the red channel, the next 8-bits
/// afterwards for the green channel, and the lower 8-bits for the blue channel.
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

/// Get the amount of Mandelbrot iterations from a HSV colored pixel
fn iterations_from_hsv_pixel(pixel: u32, max_iterations: u32) -> u32 {
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
    let iterations: u32 = (max_iterations as f64 * (hue.0 / 359.0) as f64) as u32; 
    iterations
}