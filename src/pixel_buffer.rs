use std::{fs::File, io::BufWriter, path::Path};

use crate::{
    coloring::{ColorChannelMapping, TrueColor},
    complex_plane::View,
    mandelbrot_set::MandelbrotSet,
    pixel_plane::PixelPlane,
};

#[derive(Clone)]
pub struct PixelBuffer {
    pub pixel_plane: PixelPlane,
    pub colors: Vec<TrueColor>,
    pub pixels: Vec<u32>,
    pub color_channel_mapping: ColorChannelMapping,
}

impl PixelBuffer {
    pub fn new(pixel_plane: PixelPlane) -> PixelBuffer {
        // Create a buffer to store pixel data in the form of TrueColor's
        let black = TrueColor::new(0, 0, 0);
        let colors: Vec<TrueColor> = vec![black; pixel_plane.width * pixel_plane.height];
        let color_channel_mapping = ColorChannelMapping::RGB;
        let pixels: Vec<u32> = PixelBuffer::colors_to_pixels(&colors, &color_channel_mapping);
        PixelBuffer {
            pixel_plane,
            colors,
            pixels,
            color_channel_mapping,
        }
    }

    /// Converts a buffer index to a screen coordinate
    pub fn index_to_point(&self, index: usize) -> (usize, usize) {
        (index % self.pixel_plane.width, index / self.pixel_plane.width)
    }

    /// Converts a screen coordinate to a buffer index
    pub fn point_to_index(&self, x: usize, y: usize) -> usize {
        y * self.pixel_plane.width + x
    }

    ///Converts a `TrueColor` vector to minifb compatible u32 pixel values
    pub fn colors_to_pixels(colors: &[TrueColor], color_channel_mapping: &ColorChannelMapping) -> Vec<u32> {
        colors.iter().map(|x| x.to_32_bit(color_channel_mapping)).collect()
    }

    ///Updates pixels from colors
    pub fn update_pixels(&mut self) {
        self.pixels = PixelBuffer::colors_to_pixels(&self.colors, &self.color_channel_mapping);
    }

    /// Returns the amount of Mandelbrot iterations at a given point inside the pixel plane //TODO: move this somewhere else
    /*pub fn iterations_at_point(&self, x: usize, y: usize, max_iterations: u32) -> u32 {
        let index = self.point_to_index(x, y);
        let pixel = self.buffer[index];
        let iterations = crate::iterations_from_hsv_pixel(pixel, max_iterations);
        iterations
    }*/

    /// Translate the complex plane in the `buffer` `rows` up and `columns` to the right.
    /// This operation is significantly less expensive than the `render_box_render_complex_plane_into_buffer` function, as it does not rerender anything in the complex plane, it simply
    /// get rids of `rows.abs()` rows and `columns.abs()` columns, and moves the image rows to the right and columns up.
    /// Note: The removed rows and columns should be rerendered by the `render_box_render_complex_plane_into_buffer` function.
    pub fn translate_buffer(&mut self, rows_up: i128, columns_right: i128) {
        //Iterate over the correct y's in the correct order
        let y_range: Vec<usize> = if rows_up > 0 {
            ((rows_up as usize)..self.pixel_plane.height).rev().collect()
        } else {
            (0..((self.pixel_plane.height as i128 + rows_up) as usize)).collect()
        };
        //Iterate over the correct x's in the correct order
        let x_range: Vec<usize> = if columns_right > 0 {
            ((columns_right as usize)..self.pixel_plane.width).rev().collect()
        } else {
            (0..((self.pixel_plane.width as i128 + columns_right) as usize)).collect()
        };

        for y in y_range {
            let other_y = (y as i128 - rows_up) as usize;
            //println!("y: {y} and other_y: {other_y}");
            for x in &x_range {
                let other_x = (*x as i128 - columns_right) as usize;
                //println!("x: {} and other_x: {other_x}",*x);
                let index = self.point_to_index(*x, y);
                let other_index = self.point_to_index(other_x, other_y);
                self.colors[index] = self.colors[other_index];
                self.pixels[index] = self.pixels[other_index];
            }
        }
    }

    ///Saves the `PixelBuffer` as an RGB png image to `saved/{file_name_without_extension}.png` </br>
    ///Stores the current `ComplexPlane` View in the png's metadata under the view keyword </br>
    ///Stores the `supersampling_amount` in the metadata </br>
    ///Also stores author and application metadata
    /// # Panics
    /// If the file `saved/{file_name_without_extension}.png` cannot be created
    pub fn save_as_png(&self, file_name_without_extension: &str, view: &View, m: &MandelbrotSet, supersampling_amount: u8) {
        let file_name_without_extension = file_name_without_extension.replace(':', "-").replace(' ', "_"); //Replace ':' with '-' for Windows file system. Replace ' ' with '_' because spaces are annoying in filenames.
        let file_name = format!("saved{}{}.png", std::path::MAIN_SEPARATOR_STR, file_name_without_extension);
        match std::fs::create_dir_all("saved") {
            //Create the saved folder if it does not exist
            Ok(()) => (), //Currently not doing anything with the Result of trying to create the saved folder
            Err(err) => eprintln!("{}", err),
        }
        let path = Path::new(&file_name);
        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.pixel_plane.width as u32, self.pixel_plane.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let view_text = format!("{:?}", view);
        encoder.add_text_chunk(String::from("view"), view_text).unwrap();
        let mandelbrot_set_text = format!("{:?}", m);
        encoder.add_text_chunk(String::from("mandelbrot_set"), mandelbrot_set_text).unwrap();
        let supersampling_amount_text = format!("{}x", supersampling_amount);
        encoder
            .add_text_chunk(String::from("supersampling_amount"), supersampling_amount_text)
            .unwrap();
        encoder
            .add_text_chunk(
                String::from("application"),
                String::from("Mandelbrot by Jort (https://github.com/jortrr/mandelbrot)"),
            )
            .unwrap();
        encoder
            .add_text_chunk(String::from("author"), String::from("jortrr (https://github.com/jortrr/)"))
            .unwrap();
        let color_channel_mapping_text = format!("{:?}", self.color_channel_mapping);
        encoder
            .add_text_chunk(String::from("color_channel_mapping"), color_channel_mapping_text)
            .unwrap();
        let mut data: Vec<u8> = Vec::new();
        let (r_map, g_map, b_map) = self.color_channel_mapping.get_r_g_b_mapping();
        for color in &self.colors {
            data.push(color.get_color(r_map));
            data.push(color.get_color(g_map));
            data.push(color.get_color(b_map));
        }
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&data).unwrap();
    }
}
