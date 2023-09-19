#![feature(test)]

extern crate test;

use mandelbrot::{
    model::complex::Complex, model::complex_plane::ComplexPlane, model::mandelbrot_function::MandelbrotFunction,
    model::pixel_buffer::PixelBuffer, model::pixel_plane::PixelPlane, model::rendering, view::coloring::TrueColor,
};
use test::Bencher;

//Mandelbrot set parameters
static HIGH_MAX_ITERATIONS: u32 = 10000;
static DEFAULT_MAX_ITERATIONS: u32 = 1000;
static ORBIT_RADIUS: f64 = 2.0;
static POINT_INSIDE_MANDELBROT_SET: Complex = Complex::new(-0.3, 0.0);

//Screen parameters
static WIDTH: usize = 1280;
static HEIGHT: usize = 720;

#[bench]
///Run MandelbrotFunction::iterate on a point inside the Mandelbrot set, with typical Mandelbrot set parameters, 10k max_iterations, orbit_radius of 2.0
fn bench_mandelbrot_set_iterate(b: &mut Bencher) {
    //Setup
    let m: MandelbrotFunction = MandelbrotFunction::new(HIGH_MAX_ITERATIONS, ORBIT_RADIUS);
    //Benchmark
    b.iter(|| {
        let _ = m.iterate(&POINT_INSIDE_MANDELBROT_SET);
    })
}

#[bench]
///Renders a 1280x720 1x SSAA image of the Mandelbrot set default view using Bernstein polynomal coloring, 1k max_iterations
fn bench_render_mandelbrot_set_default_view_720p_1x_ssaa(b: &mut Bencher) {
    //Setup
    let mut p: PixelBuffer = PixelBuffer::new(PixelPlane::new(WIDTH, HEIGHT));
    let c: ComplexPlane = ComplexPlane::new(WIDTH, HEIGHT);
    let m: MandelbrotFunction = MandelbrotFunction::new(DEFAULT_MAX_ITERATIONS, ORBIT_RADIUS);
    let supersampling_amount = 1;
    let coloring_function = TrueColor::new_from_bernstein_polynomials;
    //Benchmark
    b.iter(|| {
        rendering::render_complex_plane_into_buffer(&mut p, &c, &m, supersampling_amount, coloring_function);
    })
}
