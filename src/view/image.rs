use crate::model::mandelbrot_model::MandelbrotModel;

pub fn save(mandelbrot_model: &MandelbrotModel) {
    let time_stamp = chrono::Utc::now().to_string();
    if mandelbrot_model.config.window_scale == 1.0 {
        mandelbrot_model.p.save_as_png(
            &time_stamp,
            &mandelbrot_model.c.get_view(),
            &mandelbrot_model.m,
            mandelbrot_model.config.supersampling_amount,
        );
    } else {
        println!(
            "[UNSUPPORTED]: There is currently no support for saving images in the window scale is not 1.0. The window scale is {}.",
            mandelbrot_model.config.window_scale
        )
    }

    /*else {
        let mut image_p: PixelBuffer = PixelBuffer::new(PixelPlane::new(
            mandelbrot_model.config.image_width,
            mandelbrot_model.config.image_height,
        ));
        let mut image_c: ComplexPlane = ComplexPlane::new(mandelbrot_model.config.image_width, mandelbrot_model.config.image_height);
        image_p.color_channel_mapping = mandelbrot_model.p.color_channel_mapping;
        image_c.set_view(&mandelbrot_model.c.get_view());
        rendering::render_complex_plane_into_buffer(mandelbrot_model);
        image_p.save_as_png(
            &time_stamp,
            &mandelbrot_model.c.get_view(),
            &mandelbrot_model.m,
            mandelbrot_model.config.supersampling_amount,
        );
    }*/
}
