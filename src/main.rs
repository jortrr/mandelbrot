mod complex;
mod complex_plane;

use std::{time::Instant, slice::Chunks, thread, sync::{Mutex, Arc}};
use std::env;
use std::process;


use angular_units::Deg;
use complex::Complex;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use prisma::{Hsv, Rgb, FromColor, channel::AngularChannel};
use num_cpus;

use crate::complex_plane::ComplexPlane;
use mandelbrot::{self, Config};

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
fn iterations_from_hsv_pixel(pixel: u32, max_iterations: u16) -> u16 {
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
    let iterations: u16 = (max_iterations as f64 * (hue.0 / 359.0) as f64) as u16; 
    iterations
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Window dimensions in pixels
    let width: usize = 1200*2;
    let height: usize = 800*2;
    // Complex plane dimensions and increments
    let mut c = ComplexPlane::new(width, height);
    // Mandelbrot set parameters
    let max_iterations = 10000;
    let orbit_radius = 2.0; //If z remains within the orbit_radius in max_iterations, we assume c does not tend to infinity
    // User interaction variables
    let mut mouse_down: bool = false; //Variable needed for mouse single-click behavior
    let mut translation_amount: u8 = 1; //Variable determining the amount of rows and columns are translated by pressing the 4 arrow keys
    let mut scale_numerator: f64 = 9.0; //Variable denoting the user scaling speed; the lower this value, the more aggressive the zooming will become
    let scale_denominator: f64 = 10.0;
    //Mandelbrot coloring variables
    let mut hue_offset: f64 = 0.0;
    //Multithreading variables
    let amount_of_threads = num_cpus::get(); //Amount of CPU threads to use

    let gcd = num::integer::gcd(width, height); //Needed to compute the aspect ratio of the pixel plane
    println!("Pixel plane: size is {width}x{height} and aspect ratio is {}:{}",width / gcd,height / gcd);
    c.print();
    println!("Mandelbrot set parameters: max. iterations is {} and orbit radius is {}", max_iterations, orbit_radius);
    println!("Amount of CPU threads that will be used for rendering: {}", amount_of_threads);

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

    render_complex_plane_into_buffer(&mut buffer, &c, width, height, orbit_radius, max_iterations);
    println!();

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer to black
        //buffer.iter_mut().for_each(|pixel| *pixel = from_u8_rgb(r, g, b));
        
        // Update the window with the new buffer
        window.update_with_buffer(&buffer, width, height).unwrap();
        //change_hue_of_buffer(&mut buffer, 1.0);

        // Handle any window events
        // Handle any key events
        for key in window.get_keys_pressed(minifb::KeyRepeat::No) {
            println!("Key pressed: {:?}", key);
            match key {
                Key::Up => {c.translate(0.0, -c.increment_y * translation_amount as f64); translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, translation_amount as i128, 0, orbit_radius, max_iterations)},
                Key::Down => {c.translate(0.0, c.increment_y  * translation_amount as f64); translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, -(translation_amount as i128), 0, orbit_radius, max_iterations)},
                Key::Left => {c.translate(-c.increment_x  * translation_amount as f64, 0.0); translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, 0, translation_amount as i128, orbit_radius, max_iterations);},
                Key::Right => {c.translate(c.increment_x  * translation_amount as f64, 0.0); translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, 0, -(translation_amount as i128), orbit_radius, max_iterations);},
                Key::R => c.reset(),
                Key::NumPadPlus => if translation_amount < u8::MAX { translation_amount += 1;},
                Key::NumPadMinus => if translation_amount > 1 { translation_amount -= 1; },
                Key::NumPadSlash => if scale_numerator > 1.0 { scale_numerator -= 1.0;},
                Key::NumPadAsterisk => if scale_numerator < 9.0 {scale_numerator += 1.0;},
                Key::LeftBracket => c.scale(scale_numerator/scale_denominator),
                Key::RightBracket => c.scale(scale_denominator/scale_numerator),
                Key::C => println!("Center: {:?}, scale: {:?}", c.center(), c.get_scale()),
                Key::N => hue_offset += 10.0,
                Key::M => hue_offset -= 10.0,
                Key::H => change_hue_of_buffer(&mut buffer, hue_offset),
                Key::Key1 => c.set_view(-0.6604166666666667, 0.4437500000000001, 0.1),
                Key::Key2 => c.set_view(-1.0591666666666668, 0.2629166666666668, 0.01),
                Key::Key3 => c.set_view(-0.4624999999999999, 0.55, 0.1),
                Key::Key4 => c.set_view(-0.46395833333333325, 0.5531250000000001, 0.03),
                Key::Key5 => c.set_view(-0.4375218333333333, 0.5632133750000003, 0.00002000000000000002),
                Key::Key6 => c.set_view(-0.7498100000000001, -0.020300000000000054, 0.00006400000000000002),
                /*Key::N => {
                    let mut input_string = String::new();
                    io::stdin().read_line(&mut input_string).unwrap(); // Get the stdin from the user, and put it in read_string
                }*/
                _ => (),
            }
            if vec![Key::NumPadPlus, Key::NumPadMinus].contains(&key) {
                println!("translation_amount: {}", translation_amount);
            }
            if vec![Key::NumPadSlash,Key::NumPadAsterisk].contains(&key) {
                println!("scale factor: {}/{}",scale_numerator,scale_denominator);
            }
            if vec![Key::N, Key::M].contains(&key) {
                println!("hue offset: {}", hue_offset)
            }
            if vec![Key::R,Key::Key1, Key::Key2, Key::Key3, Key::Key4, Key::Key5, Key::Key6].contains(&key) {
                render_complex_plane_into_buffer(&mut buffer, &c, width, height, orbit_radius, max_iterations);
                c.print();
            }
            if vec![Key::LeftBracket, Key::RightBracket].contains(&key) {
                render_complex_plane_into_buffer(&mut buffer, &c, width, height, orbit_radius, max_iterations);
                c.print();
            }
            if vec![Key::Up, Key::Down, Key::Left, Key::Right].contains(&key) {
                c.print();
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
            }
            if window.get_mouse_down(MouseButton::Right) {
                println!("({x}, {y})");
                let complex = c.complex_from_pixel_plane(x as usize, y as usize);
                println!("{:?}", complex);
                c.set_center(complex);
                println!("Center: {:?}", c.center());
                //translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, 0, -(t.a/c.increment_x) as i128, orbit_radius, max_iterations);
                //translate_and_render_complex_plane_buffer(&mut buffer, &c, width, height, -(t.b/c.increment_y) as i128, 0, orbit_radius, max_iterations);
                render_complex_plane_into_buffer(&mut buffer, &c, width, height, orbit_radius, max_iterations);
                c.print();
                println!();
            }
            mouse_down = mouse_down_now;
        }
    }
}

/// Run the Mandelbrot set algorithm for a single Complex number
/// Returns the amount of iterations needed before Zn escapes to infinity
fn iterate(c: Complex, orbit_radius: f64, max_iterations: u16) -> u16 {
    let mut z = Complex::new(0.0, 0.0);
    let mut iterations: u16 = 0;
    let orbit_radius_squared = orbit_radius*orbit_radius;
    for _ in 0..max_iterations {
        z = z.squared().add(&c);

        if (z.a * z.a) + (z.b * z.b) > orbit_radius_squared { //Optimization: square both sides of the Mandelbrot set function, saves us taking the square root
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
fn render_complex_plane_into_buffer(buffer: &mut Vec<u32>, c: &ComplexPlane, width: usize, height: usize, orbit_radius: f64, max_iterations: u16) {
    render_box_render_complex_plane_into_buffer(buffer, c, width, height, orbit_radius, max_iterations, 0, width, 0, height);
}

/// Render the Complex plane c into the 32-bit pixel buffer by applying the Mandelbrot formula iteratively to every Complex point mapped to a pixel in the buffer. 
/// The buffer should have a size of width*height.
/// Only renders Pixels inside the render box denoted by render_min_x, render_max_x, render_min_y, render_max_y
/// orbit_radius determines when Zn is considered to have gone to infinity.
/// max_iterations concerns the maximum amount of times the Mandelbrot formula will be applied to each Complex number.
/// Note: This function is computationally intensive, and should not be used for translations
/// Note: This function is multithreaded
fn render_box_render_complex_plane_into_buffer(buffer: &mut Vec<u32>, c: &ComplexPlane, width: usize, height: usize, orbit_radius: f64, max_iterations: u16, render_min_x: usize, render_max_x: usize, render_min_y: usize, render_max_y: usize) {
    let time = benchmark_start();
    println!("render_box: ({},{}) -> ({},{}) {{{} pixels}}",render_min_x,render_min_y,render_max_x,render_max_y ,(render_max_x-render_min_x)*(render_max_y-render_min_y));
    let chunk_size = buffer.len()/height;
    let chunks: Vec<Vec<u32>> = buffer.chunks(chunk_size).map(|c| c.to_owned()).collect();
    let chunks_len = chunks.len();
    println!("chunks.len(): {}", chunks.len());
    let mut handles = Vec::new();
    let amount_of_threads = num_cpus::get(); //Amount of CPU threads to use
    let global_mutex = Arc::new(Mutex::new(0));

    for thread_id in 0..amount_of_threads {
        let plane = (*c).clone();
        let buf = chunks.clone();
        let thread_mutex = Arc::clone(&global_mutex);

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
                    let point = index_to_point(i + chunk_start, width, height);
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
            let mut index = i*width;
            for pixel in chunk {
                buffer[index] = pixel;
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

fn translate_and_render_complex_plane_buffer(buffer: &mut Vec<u32>, c: &ComplexPlane, width: usize, height: usize, rows: i128, columns: i128, orbit_radius: f64, max_iterations: u16) {
    println!("rows: {}, columns: {}",rows, columns);
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

fn change_hue_of_buffer(buffer: &mut Vec<u32>, hue_offset: f64) {
    let time = benchmark_start();
    for pixel in buffer {
        let r = (*pixel >> 16) & 0xFF;
        let g = (*pixel >> 8) & 0xFF;
        let b = *pixel & 0xFF;
        let rgb = Rgb::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
        let hsv: Hsv<f64, _> = Hsv::from_color(&rgb);
        let hue: Deg<f64> = hsv.hue();
        let hue = hue.0 + hue_offset;
        let hsv = Hsv::new(Deg(hue.abs() % 359.0), hsv.saturation(), hsv.value());
        let rgb = Rgb::from_color(&hsv);
        *pixel = from_u8_rgb((rgb.red() * 255.0) as u8, (rgb.green() * 255.0) as u8, (rgb.blue() * 255.0) as u8);
    }
    benchmark("change_hue_of_buffer()",time);
}

fn benchmark_start() -> Instant {
    Instant::now()
}

fn benchmark(function: &str,time: Instant) {
    println!("[Benchmark] {}: {:.2?}",function, time.elapsed());
}