use minifb::{Key, Window, WindowOptions};

/// Creates a 32-bit color. The encoding for each pixel is `0RGB`:
/// The upper 8-bits are ignored, the next 8-bits are for the red channel, the next 8-bits
/// afterwards for the green channel, and the lower 8-bits for the blue channel.
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    // Window dimensions
    let width = 800;
    let height = 600;

    // Create a new window
    let mut window = Window::new(
        "Pixel Drawing Example",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Create a buffer to store pixel data
    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut r: u8= 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer to black
        buffer.iter_mut().for_each(|pixel| *pixel = from_u8_rgb(r, g, b));

        // Draw a red pixel at (100, 100)
        //let pixel_index = 100 + 100 * width;
        //buffer[pixel_index] = 0xFF0000;

        // Update the window with the new buffer
        window.update_with_buffer(&buffer, width, height).unwrap();

        // Handle any window events
        for key in window.get_keys_pressed(minifb::KeyRepeat::Yes) {
            println!("Key pressed: {:?}",key);
            match key {
                Key::Q => r = u8::wrapping_add(r, 1),
                Key::A => r = u8::wrapping_sub(r, 1),
                Key::W => g = u8::wrapping_add(g, 1),
                Key::S => g = u8::wrapping_sub(g, 1),
                Key::E => b = u8::wrapping_add(b, 1),
                Key::D => b = u8::wrapping_sub(b, 1),
                _ => (),
            }
            if vec![Key::Q,Key::A,Key::W,Key::S,Key::E,Key::D].contains(&key) {
            println!("(r: {r:0>3}, g: {g:0>3}, b: {b:0>3})");
            }
        }
    }
}
