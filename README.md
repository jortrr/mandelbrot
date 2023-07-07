# Mandeljort by Oscar. Improved fork fo Mandelbrot by Jort
---
- [Running](#running)
- [Usage](#usage)
- [Controls](#controls)
- [Screenshots](#screenshots)
- [Wallpapers](#wallpapers)
- [Benchmarks](#benchmarks)
---

## Running
## 1. [Install Rust](https://www.rust-lang.org/tools/install)
On Linux:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### 2. Run Mandeljort by Jort using `cargo run` in the root of the repository
```
cargo run --release
```

## Usage
<pre>

Run Mandeljort using:
	cargo run --release -- &lt;width&gt; &lt;height&gt; &lt;max_iterations&gt; &lt;supersampling_amount&gt; &lt;window_scale&gt;
where &lt;arg&gt; means substitute with the value of arg
use '-' to use the default value of arg

KeyBindings {
    Up -> Move up translation_amount pixels,
    Down -> Move down translation_amount pixels,
    Left -> Move left translation_amount pixels,
    Right -> Move right translation_amount pixels,
    R -> Reset the Mandeljort set view to the starting view,
    NumPadPlus -> Increment translation_amount,
    NumPadMinus -> Decrement translation amount,
    NumPadAsterisk -> Increment scale_numerator,
    NumPadSlash -> Decrement scale_numerator,
    LeftBracket -> Scale the view by scaling_factor, effectively zooming in,
    RightBracket -> Scale the view by inverse_scaling_factor, effectively zooming out,
    V -> Prints the current Mandeljort set view; the center and scale,
    Key1 -> Renders VIEW_1,
    Key2 -> Renders VIEW_2,
    Key3 -> Renders VIEW_3,
    Key4 -> Renders VIEW_4,
    Key5 -> Renders VIEW_5,
    Key6 -> Renders VIEW_6,
    Key7 -> Renders VIEW_7,
    Key8 -> Renders VIEW_8,
    Key9 -> Renders VIEW_9,
    Key0 -> Renders VIEW_0,
    K -> Prints the keybindings,
    S -> Saves the current Mandeljort set view as an image in the saved folder,
    I -> Manually input a Mandeljort set view,
    A -> Pick an algorithm to color the Mandeljort set view,
    M -> Change the Mandeljort set view max_iterations,
    O -> Change the Mandeljort set view color channel mapping, xyz -> RGB, where x,y,z ∈ {{'R','G','B'}} (case-insensitive),
    Q -> Change the window and image quality of the Mandeljort set rendering by setting the SSAA multiplier, clamped from 1x to 64x,
    X -> Change the image quality of the Mandeljort set rendering by setting the SSAA multiplier, clamped from 1x to 64x,
    C -> Prints the configuration variables,
}

</pre>
## Controls
Keys | Action
:---:|:------
<kbd>↑</kbd>, <kbd>↓</kbd>, <kbd>←</kbd>, <kbd>→</kbd> | Move up, down, left, or right
<kbd>R</kbd> | Reset the Mandeljort set view to the starting view
<kbd>[</kbd> | Zoom in
<kbd>]</kbd> | Zoom out
<kbd>V</kbd> | Prints the current Mandeljort set view; the center and scale
<kbd>0</kbd>, ...,  <kbd>9</kbd> | Render a preconfigured view
<kbd>K</kbd> | Print the keybindings 
<kbd>S</kbd> | Saves the current Mandeljort set view as an image
<kbd>I</kbd> | Manually input a Mandeljort set view
<kbd>A</kbd> | Pick an algorithm to color the Mandeljort set view
<kbd>M</kbd> | Change the Mandeljort set view max_iterations
<kbd>O</kbd> | Change the Mandeljort set view color channel mapping
<kbd>Q</kbd> | Change the window and image quality of the Mandeljort set rendering by setting the SSAA multiplier
<kbd>X</kbd> | Change the image quality of the Mandeljort set rendering by setting the SSAA multiplier
<kbd>ESC</kbd>, <kbd>CTRL</kbd>+<kbd>C</kbd> | Exit
