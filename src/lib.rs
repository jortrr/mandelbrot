#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::perf,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity
)]
#![allow(
    clippy::must_use_candidate,
    clippy::multiple_crate_versions,
    clippy::uninlined_format_args,
    clippy::let_and_return,
    clippy::missing_const_for_fn,
    clippy::use_self,
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
    clippy::needless_return,
    clippy::return_self_not_must_use,
    clippy::unreadable_literal,
    clippy::single_match_else,
    clippy::suboptimal_flops,
    clippy::many_single_char_names,
    clippy::cast_sign_loss
)]
#![forbid(unsafe_code)]

pub mod controller;
pub mod model;
pub mod view;

use std::error::Error;

//Crate includes
use controller::config::Config;
use controller::minifb_controller;
use model::coloring::ColorChannelMapping;
use model::coloring::TrueColor;
use model::complex_plane::View;
use model::mandelbrot_model::ColoringFunction;
use model::mandelbrot_model::MandelbrotModel;
use view::minifb_mandelbrot_view::MandelbrotView;
use view::terminal::print_banner;
use view::terminal::print_command_info;

//Coloring function
static COLORING_FUNCTION: ColoringFunction = TrueColor::new_from_bernstein_polynomials;

//Color channel mapping
static COLOR_CHANNEL_MAPPING: ColorChannelMapping = ColorChannelMapping::RGB;

//Window title
static WINDOW_TITLE: &str = "Mandelbrot by Jort";

//Banner values
static VERSION: &str = "1.5";

//Views
static VIEW_1: View = View::new(-0.6604166666666667, 0.4437500000000001, 0.1);
static VIEW_2: View = View::new(-1.0591666666666668, 0.2629166666666668, 0.01);
static VIEW_3: View = View::new(-0.4624999999999999, 0.55, 0.1);
static VIEW_4: View = View::new(-0.46395833333333325, 0.5531250000000001, 0.03);
static VIEW_5: View = View::new(-0.4375218333333333, 0.5632133750000003, 0.00002000000000000002);
static VIEW_6: View = View::new(-0.7498100000000001, -0.020300000000000054, 0.00006400000000000002);
static VIEW_7: View = View::new(-1.7862712000000047, 0.000052399999999991516, 0.00001677721600000001);
static VIEW_8: View = View::new(-1.7862581627050718, 0.00005198056959995248, 0.000006039797760000003);
static VIEW_9: View = View::new(-0.4687339999999999, 0.5425518958333333, 0.000010000000000000003);
static VIEW_0: View = View::new(-0.437520465811966, 0.5632133750000006, 0.000004000000000000004);

///Holds all the logic currently in the main function that isn't involved with setting up configuration or handling errors, to make `main` concise and
///easy to verify by inspection
/// # Panics
/// Will panic if minifb cannot open a Window
/// # Errors
/// Currently does not return any Errors
pub fn run() -> Result<(), Box<dyn Error>> {
    let mandelbrot_model = MandelbrotModel::get_instance();

    //Print the banner
    print_banner(VERSION);
    //Print command info
    print_command_info();
    //Create the view
    let mut mandelbrot_view = MandelbrotView::new(&mandelbrot_model);
    //Run the controller
    drop(mandelbrot_model); //TODO: Get rid of MandelbrotModel mutex, as this can cause deadlocks
    minifb_controller::run(&mut mandelbrot_view)?;
    Ok(())
}
