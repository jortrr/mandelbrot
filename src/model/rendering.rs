//Temporary file to group together all rendering functionality
use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Instant,
};

use rand::Rng;

use crate::model::coloring::TrueColor;
use crate::{model::complex::Complex, MandelbrotModel};

use super::complex_plane::View;

///A box representing the area to render by rendering functions
#[derive(Clone, Copy)]
pub struct RenderBox {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl RenderBox {
    pub fn new(min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> RenderBox {
        RenderBox {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn print(&self) {
        println!(
            "RenderBox: ({},{}) -> ({},{}) {{{} pixels}}",
            self.min_x,
            self.min_y,
            self.max_x,
            self.max_y,
            self.compute_pixel_count()
        );
    }

    pub fn compute_pixel_count(&self) -> usize {
        (self.max_x - self.min_x) * (self.max_y - self.min_y)
    }

    ///Returns whether the point (x,y) is inside the `RenderBox`
    pub fn contains(&self, point: (usize, usize)) -> bool {
        !(point.0 < self.min_x || point.0 > self.max_x || point.1 < self.min_y || point.1 > self.max_y)
    }
}

/// Render the Complex plane c into the 32-bit pixel buffer by applying the Mandelbrot formula iteratively to every Complex point mapped to a pixel in the buffer.
/// The buffer should have a size of width*height.
/// `orbit_radius` determines when Zn is considered to have gone to infinity.
/// `max_iterations` concerns the maximum amount of times the Mandelbrot formula will be applied to each Complex number.
/// Note: This function is computationally intensive, and should not be used for translations
pub fn render_complex_plane_into_buffer(mandelbrot_model: &mut MandelbrotModel) {
    let render_box = RenderBox::new(0, mandelbrot_model.p.pixel_plane.width, 0, mandelbrot_model.p.pixel_plane.height);
    render_box_render_complex_plane_into_buffer(mandelbrot_model, render_box);
}

/// Render the Complex plane c into the 32-bit pixel buffer by applying the Mandelbrot formula iteratively to every Complex point mapped to a pixel in the buffer.
/// The buffer should have a size of width*height.
/// Only renders Pixels inside the render box denoted by `render_min_x`, `render_max_x`, `render_min_y`, `render_max_y`
/// `orbit_radius` determines when Zn is considered to have gone to infinity.
/// `max_iterations` concerns the maximum amount of times the Mandelbrot formula will be applied to each Complex number.
/// Note: This function is computationally intensive, and should not be used for translations
/// Note: This function is multithreaded
/// * `coloring_function` - e.g. `TrueColor::new_from_hsv`
/// # Panics
/// If `lock().unwrap()` panics
pub fn render_box_render_complex_plane_into_buffer(mandelbrot_model: &mut MandelbrotModel, render_box: RenderBox) {
    let time = benchmark_start();
    let supersampling_amount = mandelbrot_model.config.supersampling_amount.clamp(1, 64); //Supersampling_amount should be at least 1 and atmost 64
    render_box.print();
    println!("View: {:?}", mandelbrot_model.c.get_view());
    println!("SSAA: {}x", supersampling_amount);
    let chunk_size = mandelbrot_model.p.pixel_plane.width;
    let chunks: Vec<Vec<TrueColor>> = mandelbrot_model.p.colors.chunks(chunk_size).map(ToOwned::to_owned).collect();
    let chunks_len = chunks.len();
    //println!("chunks.len(): {}", chunks.len());
    let mut handles = Vec::new();
    let amount_of_threads = num_cpus::get(); //Amount of CPU threads to use
    let global_mutex = Arc::new(Mutex::new(0));
    let max_progress: u8 = 30;
    let chunks_len_over_max_progress = chunks_len / max_progress as usize;
    let current_progress_atomic: Arc<Mutex<AtomicU8>> = Arc::new(Mutex::new(AtomicU8::new(0)));

    for _thread_id in 0..amount_of_threads {
        let plane = (mandelbrot_model.c).clone();
        let buf = chunks.clone();
        let thread_mutex = Arc::clone(&global_mutex);
        let pixel_buffer = (mandelbrot_model.p).clone();
        let ms = (mandelbrot_model.m).clone();
        let atm = Arc::clone(&current_progress_atomic);
        let coloring_function = (mandelbrot_model.coloring_function).clone();

        let handle = thread::spawn(move || {
            let mut thread_chunks = Vec::new();

            loop {
                let mut data = thread_mutex.lock().unwrap();
                let current_chunk = *data;
                *data += 1;
                drop(data);
                if current_chunk >= chunks_len {
                    return thread_chunks;
                }
                //println!("Thread[{}] takes chunk[{}]", thread_id, current_chunk);
                if current_chunk % chunks_len_over_max_progress == 0 {
                    let current_progress = atm.lock().unwrap().load(Ordering::Relaxed);
                    print_progress_bar(current_progress, max_progress);
                    if current_progress < u8::MAX {
                        atm.lock().unwrap().store(current_progress + 1, Ordering::Relaxed);
                    }
                }

                let chunk_start = chunk_size * current_chunk;
                let mut chunk = buf[current_chunk].clone();

                for (i, pixel) in chunk.iter_mut().enumerate() {
                    let point = pixel_buffer.index_to_point(i + chunk_start);
                    if !render_box.contains(point) {
                        continue; //Do not render Pixel points outside of the render box
                    }
                    let original_x: f64 = f64::from(point.0 as u32);
                    let original_y: f64 = f64::from(point.1 as u32);
                    //Supersampling, see: https://darkeclipz.github.io/fractals/paper/Fractals%20&%20Rendering%20Techniques.html
                    let mut colors: Vec<TrueColor> = Vec::new();
                    for _ in 0..supersampling_amount {
                        let (random_x, random_y): (f64, f64) = rand::thread_rng().gen::<(f64, f64)>();
                        let (x, y): (f64, f64) = (original_x + random_x, original_y + random_y);
                        let complex = plane.complex_from_pixel_plane(x, y);
                        let iterations = ms.iterate(&complex);
                        let color = coloring_function(iterations, ms.max_iterations);
                        colors.push(color);
                    }
                    let supersampled_color = TrueColor::average(&colors);
                    *pixel = supersampled_color;
                }
                thread_chunks.push((current_chunk, chunk.clone()));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let thread_chunks = handle.join().unwrap();
        for (i, chunk) in thread_chunks {
            let mut index = i * mandelbrot_model.p.pixel_plane.width;
            for color in chunk {
                mandelbrot_model.p.colors[index] = color;
                index += 1;
            }
        }
    }
    mandelbrot_model.p.update_pixels();
    println!();
    benchmark("render_box_render_complex_plane_into_buffer()", time);
}

pub fn translate_and_render_complex_plane_buffer(mandelbrot_model: &mut MandelbrotModel, rows: i128, columns: i128) {
    println!("rows: {}, columns: {}", rows, columns);
    let max_x: usize = if columns > 0 {
        columns as usize
    } else {
        mandelbrot_model.p.pixel_plane.width - 1
    };
    let max_y: usize = if rows > 0 {
        rows as usize
    } else {
        mandelbrot_model.p.pixel_plane.height - 1
    };
    mandelbrot_model.p.translate_buffer(rows, columns);
    if rows == 0 {
        let render_box = RenderBox::new(
            (max_x as i128 - columns.abs()) as usize,
            max_x,
            0,
            mandelbrot_model.p.pixel_plane.height,
        );
        render_box_render_complex_plane_into_buffer(mandelbrot_model, render_box);
    } else if columns == 0 {
        let render_box = RenderBox::new(
            0,
            mandelbrot_model.p.pixel_plane.width,
            (max_y as i128 - rows.abs()) as usize,
            max_y,
        );
        render_box_render_complex_plane_into_buffer(mandelbrot_model, render_box);
    } else {
        println!("ERROR: translate_and_render_complex_plane_buffer() requires that rows == 0 || columns == 0");
    }
}

///# Panics
/// If `rows_up` != 0 && `columns_right` != 0
pub fn translate_and_render_efficiently(mandelbrot_model: &mut MandelbrotModel, rows_up: i16, columns_right: i16) {
    assert!(
        rows_up == 0 || columns_right == 0,
        "translate_and_render_efficiently: rows_up should be 0 or columns_right should be 0!"
    );

    let row_sign: f64 = if rows_up > 0 { -1.0 } else { 1.0 };
    let column_sign: f64 = if columns_right > 0 { 1.0 } else { -1.0 };
    mandelbrot_model.c.translate(
        column_sign * mandelbrot_model.c.pixels_to_real(columns_right.unsigned_abs() as u8),
        row_sign * mandelbrot_model.c.pixels_to_imaginary(rows_up.unsigned_abs() as u8),
    );
    translate_and_render_complex_plane_buffer(mandelbrot_model, rows_up.into(), (-columns_right).into());
}

pub fn translate_to_center_and_render_efficiently(mandelbrot_model: &mut MandelbrotModel, new_center: &Complex) {
    let mut translation: Complex = new_center.subtract(&mandelbrot_model.c.center());
    //Mirror the y translation because the screen y is mirrored compared to the complex plane y axis
    translation.y = -translation.y;

    //Translate x, to the right
    mandelbrot_model.c.translate(translation.x, 0.0);
    let columns_right = -mandelbrot_model.c.real_to_pixels(translation.x);
    dbg!(columns_right);
    translate_and_render_complex_plane_buffer(mandelbrot_model, 0, columns_right.into());

    //Translate y, up
    mandelbrot_model.c.translate(0.0, translation.y);
    let rows_up = -mandelbrot_model.c.imaginary_to_pixels(translation.y);
    dbg!(rows_up);
    translate_and_render_complex_plane_buffer(mandelbrot_model, rows_up.into(), 0);
}

fn benchmark_start() -> Instant {
    Instant::now()
}

fn benchmark(function: &str, time: Instant) {
    println!("[Benchmark] {}: {:.2?}", function, time.elapsed());
}

///Prints a progress bar on the current line, will print over the contents of the cursor's current line, so make sure the function is given a newline to print over
fn print_progress_bar(current_progress: u8, max_progress: u8) {
    print!("\rProgress: ["); //Print a \r carriage return to return the cursor to the beginning of the line: https://stackoverflow.com/questions/59890270/how-do-i-overwrite-console-output
    for i in 0..max_progress {
        let symbol = if i <= current_progress { '+' } else { '.' };
        print!("{}", symbol);
    }
    print!("]");
    io::stdout().flush().unwrap();
}

pub fn render(mandelbrot_model: &mut MandelbrotModel) {
    //TODO: Observer pattern view -> model to update the view, instead of rendering manually
    render_complex_plane_into_buffer(mandelbrot_model);
    mandelbrot_model.c.print();
}

pub fn set_view(mandelbrot_model: &mut MandelbrotModel, view: &View) {
    mandelbrot_model.c.set_view(view);
    render(mandelbrot_model);
}
