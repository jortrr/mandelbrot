use std::error::Error;

use minifb::{Key, MouseButton, MouseMode, Window};

use crate::{
    controller::{minifb_mouse_click_recorder::MouseClickRecorder, user_input::pick_option},
    model::{
        coloring::TrueColor,
        complex_plane::{ComplexPlane, View},
        pixel_buffer::PixelBuffer,
        pixel_plane::PixelPlane,
        rendering::{self, render, set_view},
    },
    view::minifb_mandelbrot_view::MandelbrotView,
    MandelbrotModel, VIEW_0, VIEW_1, VIEW_2, VIEW_3, VIEW_4, VIEW_5, VIEW_6, VIEW_7, VIEW_8, VIEW_9,
};

use super::{minifb_key_bindings::KeyBindings, user_input::ask};

// Handle any key events
pub fn handle_key_events(window: &Window, k: &KeyBindings) {
    if let Some(key) = window.get_keys_pressed(minifb::KeyRepeat::No).first() {
        print!("\nKey pressed: ");
        k.print_key(key);
        k.run(key);
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        match key {
            Key::K => k.print(),
            Key::A => {
                mandelbrot_model.coloring_function = pick_option(&[
                    ("HSV", TrueColor::new_from_hsv_colors),
                    ("Bernstein polynomials", TrueColor::new_from_bernstein_polynomials),
                ]);
                render(&mut mandelbrot_model);
            }
            _ => (),
        }
    }
}

pub fn handle_left_mouse_clicked(mandelbrot_model: &MandelbrotModel, x: f32, y: f32) {
    println!("\nMouseButton::Left -> Info at ({x}, {y})");
    //let iterations = MandelbrotModel::get_instance().p.iterations_at_point(x as usize, y as usize, MandelbrotModel::get_instance().m.max_iterations); //TODO: fix this
    let complex = mandelbrot_model.c.complex_from_pixel_plane(x.into(), y.into());
    println!("Complex: {:?}", complex);
    //println!("iterations: {}", iterations);
    println!();
}

pub fn handle_right_mouse_clicked(mandelbrot_model: &mut MandelbrotModel, x: f32, y: f32) {
    println!("\nMouseButton::Right -> Move to ({x}, {y})");
    let new_center = mandelbrot_model.c.complex_from_pixel_plane(x.into(), y.into());
    println!("mandelbrot_model.c.center: {:?}", mandelbrot_model.c.center());
    println!("new_center: {:?}", new_center);

    rendering::translate_to_center_and_render_efficiently(mandelbrot_model, &new_center);
    mandelbrot_model.c.print();
    println!();
}

pub fn handle_mouse_events(window: &Window) {
    let mut mandelbrot_model = MandelbrotModel::get_instance();
    static LEFT_MOUSE_RECORDER: MouseClickRecorder = MouseClickRecorder::new(MouseButton::Left); //Static variable with interior mutability to toggle mouse clicks; without such a variable, clicking the screen once would result in multiple actions
    static RIGHT_MOUSE_RECORDER: MouseClickRecorder = MouseClickRecorder::new(MouseButton::Right);

    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
        //Left mouse actions
        if LEFT_MOUSE_RECORDER.was_clicked(window) {
            handle_left_mouse_clicked(&mandelbrot_model, x, y);
        }

        //Right mouse actions
        if RIGHT_MOUSE_RECORDER.was_clicked(window) {
            handle_right_mouse_clicked(&mut mandelbrot_model, x, y);
        }
    }
}

pub fn initialize_keybindings(key_bindings: &mut KeyBindings) {
    let empty_closure = || ();
    key_bindings.add(Key::Up, "Move up translation_amount pixels", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        let rows = mandelbrot_model.vars.translation_amount;
        rendering::translate_and_render_efficiently(&mut mandelbrot_model, rows.into(), 0);
        mandelbrot_model.c.print();
    });
    key_bindings.add(Key::Down, "Move down translation_amount pixels", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        let rows = -i16::from(mandelbrot_model.vars.translation_amount);
        rendering::translate_and_render_efficiently(&mut mandelbrot_model, rows, 0);
        mandelbrot_model.c.print();
    });
    key_bindings.add(Key::Left, "Move left translation_amount pixels", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        let columns = -i16::from(mandelbrot_model.vars.translation_amount);
        rendering::translate_and_render_efficiently(&mut mandelbrot_model, 0, columns);
        mandelbrot_model.c.print();
    });
    key_bindings.add(Key::Right, "Move right translation_amount pixels", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        let columns = mandelbrot_model.vars.translation_amount.into();
        rendering::translate_and_render_efficiently(&mut mandelbrot_model, 0, columns);
        mandelbrot_model.c.print();
    });
    key_bindings.add(Key::R, "Reset the Mandelbrot set view to the starting view", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        mandelbrot_model.c.reset();
        render(&mut mandelbrot_model);
    });
    key_bindings.add(Key::NumPadPlus, "Increment translation_amount", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        mandelbrot_model.vars.increment_translation_amount();
        println!("translation_amount: {}", mandelbrot_model.vars.translation_amount);
    });

    key_bindings.add(Key::NumPadMinus, "Decrement translation amount", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        mandelbrot_model.vars.decrement_translation_amount();
        println!("translation_amount: {}", mandelbrot_model.vars.translation_amount);
    });
    key_bindings.add(Key::NumPadAsterisk, "Increment scale_numerator", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        mandelbrot_model.vars.increment_scale_numerator();
        println!(
            "scale factor: {}/{}",
            mandelbrot_model.vars.scale_numerator, mandelbrot_model.vars.scale_denominator
        );
    });
    key_bindings.add(Key::NumPadSlash, "Decrement scale_numerator", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        mandelbrot_model.vars.decrement_scale_numerator();
        println!(
            "scale factor: {}/{}",
            mandelbrot_model.vars.scale_numerator, mandelbrot_model.vars.scale_denominator
        );
    });
    key_bindings.add(Key::LeftBracket, "Scale the view by scaling_factor, effectively zooming in", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        let scaling_factor = mandelbrot_model.vars.scaling_factor();
        mandelbrot_model.c.scale(scaling_factor);
        render(&mut mandelbrot_model);
    });
    key_bindings.add(
        Key::RightBracket,
        "Scale the view by inverse_scaling_factor, effectively zooming out",
        || {
            let mut mandelbrot_model = MandelbrotModel::get_instance();
            let inverse_scaling_factor = mandelbrot_model.vars.inverse_scaling_factor();
            mandelbrot_model.c.scale(inverse_scaling_factor);
            render(&mut mandelbrot_model);
        },
    );
    key_bindings.add(Key::V, "Prints the current Mandelbrot set view; the center and scale", || {
        let mandelbrot_model = MandelbrotModel::get_instance();
        println!(
            "Center: {:?}, scale: {:?}",
            mandelbrot_model.c.center(),
            mandelbrot_model.c.get_scale()
        );
    });
    key_bindings.add(Key::Key1, "Renders VIEW_1", || {
        set_view(&mut MandelbrotModel::get_instance(), &VIEW_1)
    });
    key_bindings.add(Key::Key2, "Renders VIEW_2", || {
        set_view(&mut MandelbrotModel::get_instance(), &VIEW_2)
    });
    key_bindings.add(Key::Key3, "Renders VIEW_3", || {
        set_view(&mut MandelbrotModel::get_instance(), &VIEW_3)
    });
    key_bindings.add(Key::Key4, "Renders VIEW_4", || {
        set_view(&mut MandelbrotModel::get_instance(), &VIEW_4)
    });
    key_bindings.add(Key::Key5, "Renders VIEW_5", || {
        set_view(&mut MandelbrotModel::get_instance(), &VIEW_5)
    });
    key_bindings.add(Key::Key6, "Renders VIEW_6", || {
        set_view(&mut MandelbrotModel::get_instance(), &VIEW_6)
    });
    key_bindings.add(Key::Key7, "Renders VIEW_7", || {
        set_view(&mut MandelbrotModel::get_instance(), &VIEW_7)
    });
    key_bindings.add(Key::Key8, "Renders VIEW_8", || {
        set_view(&mut MandelbrotModel::get_instance(), &VIEW_8)
    });
    key_bindings.add(Key::Key9, "Renders VIEW_9", || {
        set_view(&mut MandelbrotModel::get_instance(), &VIEW_9)
    });
    key_bindings.add(Key::Key0, "Renders VIEW_0", || {
        set_view(&mut MandelbrotModel::get_instance(), &VIEW_0)
    });
    key_bindings.add(Key::K, "Prints the keybindings", empty_closure);
    key_bindings.add(
        Key::S,
        "Saves the current Mandelbrot set view as an image in the saved folder",
        || {
            let mut mandelbrot_model = MandelbrotModel::get_instance();
            let time_stamp = chrono::Utc::now().to_string();
            if mandelbrot_model.config.window_scale == 1.0 {
                mandelbrot_model.p.save_as_png(
                    &time_stamp,
                    &mandelbrot_model.c.get_view(),
                    &mandelbrot_model.m,
                    mandelbrot_model.config.supersampling_amount,
                );
            } else {
                let mut image_p: PixelBuffer = PixelBuffer::new(PixelPlane::new(
                    mandelbrot_model.config.image_width,
                    mandelbrot_model.config.image_height,
                ));
                let mut image_c: ComplexPlane =
                    ComplexPlane::new(mandelbrot_model.config.image_width, mandelbrot_model.config.image_height);
                image_p.color_channel_mapping = mandelbrot_model.p.color_channel_mapping;
                image_c.set_view(&mandelbrot_model.c.get_view());
                rendering::render_complex_plane_into_buffer(&mut mandelbrot_model);
                image_p.save_as_png(
                    &time_stamp,
                    &mandelbrot_model.c.get_view(),
                    &mandelbrot_model.m,
                    mandelbrot_model.config.supersampling_amount,
                );
            }
        },
    );
    key_bindings.add(Key::I, "Manually input a Mandelbrot set view", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        mandelbrot_model.c.set_view(&View::new(ask("x"), ask("y"), ask("scale")));
        render(&mut mandelbrot_model);
    });
    key_bindings.add(Key::A, "Pick an algorithm to color the Mandelbrot set view", empty_closure);
    key_bindings.add(Key::M, "Change the Mandelbrot set view max_iterations", || {
        let mut mandelbrot_model = MandelbrotModel::get_instance();
        mandelbrot_model.m.max_iterations = ask("max_iterations");
        render(&mut mandelbrot_model);
    });
    key_bindings.add(
        Key::O,
        "Change the Mandelbrot set view color channel mapping, xyz -> RGB, where x,y,z ∈ {{'R','G','B'}} (case-insensitive)",
        || {
            let mut mandelbrot_model = MandelbrotModel::get_instance();
            mandelbrot_model.p.color_channel_mapping = ask("color_channel_mapping");
            render(&mut mandelbrot_model);
        },
    );
    key_bindings.add(
        Key::Q,
        "Change the window and image quality of the Mandelbrot set rendering by setting the SSAA multiplier, clamped from 1x to 64x",
        || {
            let mut mandelbrot_model = MandelbrotModel::get_instance();
            mandelbrot_model.config.supersampling_amount = ask::<u8>("supersampling_amount").clamp(1, 64);
            render(&mut mandelbrot_model);
        },
    );
    key_bindings.add(
        Key::X,
        "Change the image quality of the Mandelbrot set rendering by setting the SSAA multiplier, clamped from 1x to 64x",
        empty_closure,
    );
    key_bindings.add(Key::C, "Prints the configuration variables", || {
        println!("{:?}", MandelbrotModel::get_instance().config);
    });
}

pub fn run(mandelbrot_view: &mut MandelbrotView) -> Result<(), Box<dyn Error>> {
    let mut mandelbrot_model = MandelbrotModel::get_instance();
    //Initialize keybindings
    let mut key_bindings: KeyBindings = KeyBindings::new(Vec::new()); //TODO: Make KeyBindings a singleton
    initialize_keybindings(&mut key_bindings);
    key_bindings.print();

    println!("\nRendering Mandelbrot set starting view");
    render(&mut mandelbrot_model);
    drop(mandelbrot_model);

    // Main loop
    while mandelbrot_view.window.is_open() && !mandelbrot_view.window.is_key_down(Key::Escape) {
        // Handle any window events
        handle_key_events(&mandelbrot_view.window, &key_bindings);

        //Handle any mouse events
        handle_mouse_events(&mandelbrot_view.window);

        // Update the window with the new buffer
        let mandelbrot_model = MandelbrotModel::get_instance();
        mandelbrot_view.update(&mandelbrot_model);
    }

    mandelbrot_view.exit();
    Ok(())
}
