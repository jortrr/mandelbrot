#![feature(test)]

extern crate test;

use mandelbrot::{mandelbrot_set::MandelbrotSet, complex::Complex};
use test::Bencher;

//Mandelbrot set parameters
static MAX_ITERATIONS: u32 = 10000;
static ORBIT_RADIUS: f64 = 2.0;
static POINT_INSIDE_MANDELBROT_SET: Complex = Complex::new(-0.3, 0.0);


#[bench]
///Run MandelbrotSet::iterate on a point inside the Mandelbrot set, with typical Mandelbrot set parameters
fn bench_mandelbrot_set_iterate(b: &mut Bencher) {
    let m: MandelbrotSet = MandelbrotSet::new(MAX_ITERATIONS, ORBIT_RADIUS);
    b.iter(|| {
        let _ = m.iterate(&POINT_INSIDE_MANDELBROT_SET);
    })
}