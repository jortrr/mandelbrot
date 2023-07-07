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

## Screenshots
![1](https://github.com/jortrr/mandeljort/assets/38651009/8a09d7a2-69e0-47b4-8e60-06392321ed08)
![2](https://github.com/jortrr/mandeljort/assets/38651009/e2055e8b-6e12-4d73-8257-ca800c8a2159)
![3](https://github.com/jortrr/mandeljort/assets/38651009/8a5fb455-f200-482e-927b-c34adbed45ca)
![4](https://github.com/jortrr/mandeljort/assets/38651009/7d385212-0d91-4a5c-b6d1-4b75a2acaf02)
![5](https://github.com/jortrr/mandeljort/assets/38651009/267ad0b1-c774-4474-816c-3de8150667c5)
![6](https://github.com/jortrr/mandeljort/assets/38651009/c57b035d-b3b1-4b54-a92f-c6d6952f26fb)
![7](https://github.com/jortrr/mandeljort/assets/38651009/3ff0c244-6055-4228-8e8d-dbb852a1d26a)
![8](https://github.com/jortrr/mandeljort/assets/38651009/21775d83-a455-4a0c-9002-edada7c44a25)
![9](https://github.com/jortrr/mandeljort/assets/38651009/f21e12b8-365b-49f7-aef0-7022afe14369)
![10](https://github.com/jortrr/mandeljort/assets/38651009/cff43524-f882-4181-92c9-4660f09342bf)
![11](https://github.com/jortrr/mandeljort/assets/38651009/e267cc25-8af3-4421-9361-59c7724b2dcb)
![12](https://github.com/jortrr/mandeljort/assets/38651009/32412e40-49af-483a-84e8-90f22cc7b1d1)
![13](https://github.com/jortrr/mandeljort/assets/38651009/c158fc1d-d376-47f2-9be0-433637e0d7e2)
![14](https://github.com/jortrr/mandeljort/assets/38651009/cbad0e72-420a-459d-87ab-c551dd67efbe)
![15](https://github.com/jortrr/mandeljort/assets/38651009/4f784846-2f1c-4af4-b841-309e6e25578d)

## Wallpapers
### 3440x1440
![2023-06-26_12-48-59 189737_UTC](https://github.com/jortrr/mandeljort/assets/38651009/d487285e-d233-4374-bb5c-c46d84f0d83f)
![2023-06-26_12-35-03 632016400_UTC](https://github.com/jortrr/mandeljort/assets/38651009/ccad4f4a-1bc3-455b-98d9-53c5c0f85db3)
![2023-06-26_15-36-32 408135800_UTC](https://github.com/jortrr/mandeljort/assets/38651009/9baa2097-7cd7-4e53-b799-bfeab95c14dd)
![2023-06-26_15-25-34 763976800_UTC](https://github.com/jortrr/mandeljort/assets/38651009/49c02944-1d34-493c-a047-6478b4046052)
![2023-06-26_12-56-26 498671100_UTC](https://github.com/jortrr/mandeljort/assets/38651009/91813670-bcdd-480d-bb28-0fd68f8dad10)
![2023-06-26_08-54-45 849997300_UTC](https://github.com/jortrr/mandeljort/assets/38651009/0f28dfac-ed62-4096-8c9a-73370a2d69fb)

### 2560x1600
![2023-06-24 18:50:23 842186828 UTC](https://github.com/jortrr/mandeljort/assets/38651009/03ce23c3-770e-444a-b163-aa247cfeda7c)
![2023-06-29_11-45-50 778868321_UTC](https://github.com/jortrr/mandeljort/assets/38651009/845cf9cf-3ac5-4df7-8150-abdd227af18a)
![2023-06-29_11-46-06 821888383_UTC](https://github.com/jortrr/mandeljort/assets/38651009/4f369655-c36d-42e3-8864-0ae3cf998854)
![2023-06-29_11-48-21 628194507_UTC](https://github.com/jortrr/mandeljort/assets/38651009/57c09c54-5616-487d-a7e8-66e69d8009db)
![2023-07-01_15-51-57 214248342_UTC](https://github.com/jortrr/mandeljort/assets/38651009/5f66ad28-0bcd-4648-93c6-4f5915055538)

### 1170x2532 (Iphone 13)
![2023-06-24 19:29:53 436765840 UTC](https://github.com/jortrr/mandeljort/assets/38651009/9b67dcdb-9dc3-4646-bbb4-96d7dc3ddb8f)

## Benchmarks
- [Benchmark results on GitHub pages](https://jortrr.github.io/mandeljort/dev/bench/)
