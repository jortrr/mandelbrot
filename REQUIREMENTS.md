# Requirements (WIP)

- [x] Create command line arguments for the resolution and the amount of iterations
- [ ] Create a PixelBuffer struct. This struct should contain its associated PixelPlane struct, and should store the color of each pixel in 8 bits per color, so 8 bits for red, green and blue. This is also called true color (24-bit). The PixelBuffer should be the input for rendering functions.
- [ ] Create a TrueColor struct, containing a 24-bit true color, with 8-bits for the R, G and B channels. The struct should contain methods for conversion to HSV and from HSV. The TrueColor struct should be constructable from a u32 representing a true color.
- [ ] Create a PixelPlane struct. This struct should contain a width and height in pixels, and an aspect ratio.
- [x] Create a MandelbrotSet struct. This struct should contain max_iterations, orbit radius, and an iterate function.
- [ ] Create a logging function, with log levels DEBUG, INFO, ERROR, and a commandline argument specifying the level, default should be INFO
- [ ] Create a struct to combine KEY, description, action, to make it easy to display what keys are doing in the future, self-documenting
- [ ] Save the current Mandelbrot view with the 'S' key to a .png file, the name should be the view: x, y and scale
- [ ] Allow the user to swap R, G and B colors in an image
