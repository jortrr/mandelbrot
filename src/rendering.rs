//Temporary file to group together all rendering functionality
use std::{time::Instant, thread, sync::{Arc, Mutex, atomic::{AtomicU8, Ordering}}, io::{self, Write}};

use angular_units::Deg;
use prisma::{Hsv, Rgb, FromColor};

use crate::{pixel_buffer::PixelBuffer, complex_plane::ComplexPlane, mandelbrot_set::MandelbrotSet, complex::Complex};


/// Render the Complex plane c into the 32-bit pixel buffer by applying the Mandelbrot formula iteratively to every Complex point mapped to a pixel in the buffer. 
/// The buffer should have a size of width*height.
/// orbit_radius determines when Zn is considered to have gone to infinity.
/// max_iterations concerns the maximum amount of times the Mandelbrot formula will be applied to each Complex number.
/// Note: This function is computationally intensive, and should not be used for translations
pub fn render_complex_plane_into_buffer(p: &mut PixelBuffer, c: &ComplexPlane, m: &MandelbrotSet) {
    render_box_render_complex_plane_into_buffer(p, c, m, 0, p.pixel_plane.width, 0, p.pixel_plane.height);
}

/// Render the Complex plane c into the 32-bit pixel buffer by applying the Mandelbrot formula iteratively to every Complex point mapped to a pixel in the buffer. 
/// The buffer should have a size of width*height.
/// Only renders Pixels inside the render box denoted by render_min_x, render_max_x, render_min_y, render_max_y
/// orbit_radius determines when Zn is considered to have gone to infinity.
/// max_iterations concerns the maximum amount of times the Mandelbrot formula will be applied to each Complex number.
/// Note: This function is computationally intensive, and should not be used for translations
/// Note: This function is multithreaded
pub fn render_box_render_complex_plane_into_buffer(p: &mut PixelBuffer, c: &ComplexPlane, m: &MandelbrotSet, render_min_x: usize, render_max_x: usize, render_min_y: usize, render_max_y: usize) {
    let time = benchmark_start();
    println!("render_box: ({},{}) -> ({},{}) {{{} pixels}}",render_min_x,render_min_y,render_max_x,render_max_y ,(render_max_x-render_min_x)*(render_max_y-render_min_y));
    let chunk_size = p.buffer.len()/p.pixel_plane.height;
    let chunks: Vec<Vec<u32>> = p.buffer.chunks(chunk_size).map(|c| c.to_owned()).collect();
    let chunks_len = chunks.len();
    println!("chunks.len(): {}", chunks.len());
    let mut handles = Vec::new();
    let amount_of_threads = num_cpus::get(); //Amount of CPU threads to use
    let global_mutex = Arc::new(Mutex::new(0));
    let max_progress: u8 = 30;
    let chunks_len_over_max_progress = chunks_len / max_progress as usize;
    let current_progress_atomic: Arc<Mutex<AtomicU8>>= Arc::new(Mutex::new(AtomicU8::new(0)));

    for thread_id in 0..amount_of_threads {
        let plane = (*c).clone();
        let buf = chunks.clone();
        let thread_mutex = Arc::clone(&global_mutex);
        let pixel_buffer = (*p).clone();
        let ms = (*m).clone();
        let atm = Arc::clone(&current_progress_atomic);

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
                //println!("Thread[{}] takes chunk[{}]", thread_id, current_chunk);
                if current_chunk % chunks_len_over_max_progress == 0 {
                    let mutex = atm.lock().unwrap();
                    let current_progress =   (*mutex).load(Ordering::Relaxed);
                    print_progress_bar(current_progress, max_progress);
                    if current_progress < u8::MAX
                    {
                        (*mutex).store(current_progress+1, Ordering::Relaxed);
                    }
                }

            
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
                    let iterations = ms.iterate(complex);
                    //println!("iterations: {}", iterations);
                    //println!();
                    //let hue: f64 = 359.0 * (iterations as f64 / ms.max_iterations as f64);
                    let hue = 0.3 *iterations as f64;
                    let value: f64 = if iterations < ms.max_iterations {1.0} else {0.0};
                    let hsv = Hsv::new(Deg(hue % 359.0),0.8,value);
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
    println!();
    benchmark("render_box_render_complex_plane_into_buffer()", time);
}

pub fn translate_and_render_complex_plane_buffer(p: &mut PixelBuffer, c: &ComplexPlane, m: &MandelbrotSet, rows: i128, columns: i128) {
    println!("rows: {}, columns: {}",rows, columns);
    let max_x: usize = if columns > 0 {columns as usize} else {p.pixel_plane.width-1};
    let max_y: usize = if rows > 0 {rows as usize} else {p.pixel_plane.height-1};
    p.translate_buffer(rows, columns);
    if rows == 0 {
        render_box_render_complex_plane_into_buffer(p, c, m, (max_x as i128-columns.abs()) as usize, max_x, 0, p.pixel_plane.height);
    }
    else if columns == 0 {
        render_box_render_complex_plane_into_buffer(p, c, m, 0, p.pixel_plane.width, (max_y as i128 -rows.abs()) as usize, max_y);
    } else {
        println!("ERROR: translate_and_render_complex_plane_buffer() requires that rows == 0 || columns == 0");
    }
}

pub fn translate_and_render_efficiently(c: &mut ComplexPlane, p: &mut PixelBuffer, m: &MandelbrotSet, rows_up: i16, columns_right: i16) {
    if rows_up != 0 && columns_right != 0 {
        panic!("translate_and_render_efficiently: rows_up should be 0 or columns_right should be 0!")
    }
    let row_sign: f64 = if rows_up > 0 {-1.0} else {1.0};
    let column_sign: f64 = if columns_right > 0 {1.0} else {-1.0};
    c.translate(column_sign*c.pixels_to_real(columns_right.abs() as u8), row_sign*c.pixels_to_imaginary(rows_up.abs() as u8)); 
    translate_and_render_complex_plane_buffer(p, c, m, rows_up.into(), (-columns_right).into());
}

pub fn translate_to_center_and_render_efficiently(c: &mut ComplexPlane, p: &mut PixelBuffer, m: &MandelbrotSet, new_center: &Complex) {
    let mut translation: Complex = new_center.subtract(&c.center());
    //Mirror the y translation because the screen y is mirrored compared to the complex plane y axis
    translation.y = -translation.y;

    //Translate x, to the right
    c.translate(translation.x, 0.0);
    let columns_right = -c.real_to_pixels(translation.x);
    dbg!(columns_right);
    translate_and_render_complex_plane_buffer(p, c, m, 0, columns_right.into());

    //Translate y, up
    c.translate(0.0, translation.y);
    let rows_up = -c.imaginary_to_pixels(translation.y);
    dbg!(rows_up);
    translate_and_render_complex_plane_buffer(p, c, m, rows_up.into(), 0);
}

/// Creates a 32-bit color. The encoding for each pixel is `0RGB`:
/// The upper 8-bits are ignored, the next 8-bits are for the red channel, the next 8-bits
/// afterwards for the green channel, and the lower 8-bits for the blue channel.
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn benchmark_start() -> Instant {
    Instant::now()
}

fn benchmark(function: &str,time: Instant) {
    println!("[Benchmark] {}: {:.2?}",function, time.elapsed());
}

///Prints a progress bar on the current line, will print over the contents of the cursor's current line, so make sure the function is given a newline to print over
fn print_progress_bar(current_progress: u8, max_progress: u8) {
    print!("\rProgress: ["); //Print a \r carriage return to return the cursor to the beginning of the line: https://stackoverflow.com/questions/59890270/how-do-i-overwrite-console-output
    for i in 0..max_progress {
        let symbol = if i <= current_progress {'+'} else {'.'};
        print!("{}", symbol);
    }
    print!("]");
    io::stdout().flush().unwrap();
}