# Mandelbrot by Jort, Mandelbrot set viewer written in Rust
    __  __                 _      _ _               _   
    |  \/  |               | |    | | |             | |  
    | \  / | __ _ _ __   __| | ___| | |__  _ __ ___ | |_ 
    | |\/| |/ _` | '_ \ / _` |/ _ \ | '_ \| '__/ _ \| __|
    | |  | | (_| | | | | (_| |  __/ | |_) | | | (_) | |_ 
    |_|  |_|\__,_|_| |_|\__,_|\___|_|_.__/|_|  \___/ \__|
       __             __         __ 
      / /  __ __  __ / /__  ____/ /_
     / _ \/ // / / // / _ \/ __/ __/
    /_.__/\_, /  \___/\___/_/  \__/ 
         /___/                      v1.2
         
---
- [Running](#running)
- [Usage](#usage)
- [Controls](#controls)
- [Screenshots](#screenshots)
- [Wallpapers](#wallpapers)
---

## Running
## 1. [Install Rust](https://www.rust-lang.org/tools/install)
On Linux:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### 2. Run Mandelbrot by Jort using `cargo run` in the root of the repository
```
cargo run --release
```

## Usage
<pre>
    
__  __                 _      _ _               _   
|  \/  |               | |    | | |             | |  
| \  / | __ _ _ __   __| | ___| | |__  _ __ ___ | |_ 
| |\/| |/ _` | '_ \ / _` |/ _ \ | '_ \| '__/ _ \| __|
| |  | | (_| | | | | (_| |  __/ | |_) | | | (_) | |_ 
|_|  |_|\__,_|_| |_|\__,_|\___|_|_.__/|_|  \___/ \__|
   __             __         __ 
  / /  __ __  __ / /__  ____/ /_
 / _ \/ // / / // / _ \/ __/ __/
/_.__/\_, /  \___/\___/_/  \__/ 
     /___/                      v1.2


Run Mandelbrot using:
	cargo run --release -- &lt;width&gt; &lt;height&gt; &lt;max_iterations&gt; &lt;supersampling_amount&gt; &lt;window_scale&gt;
where &lt;arg&gt; means substitute with the value of arg
use '-' to use the default value of arg

KeyBindings {
    Up -> Move up translation_amount pixels,
    Down -> Move down translation_amount pixels,
    Left -> Move left translation_amount pixels,
    Right -> Move right translation_amount pixels,
    R -> Reset the Mandelbrot set view to the starting view,
    NumPadPlus -> Increment translation_amount,
    NumPadMinus -> Decrement translation amount,
    NumPadAsterisk -> Increment scale_numerator,
    NumPadSlash -> Decrement scale_numerator,
    LeftBracket -> Scale the view by scaling_factor, effectively zooming in,
    RightBracket -> Scale the view by inverse_scaling_factor, effectively zooming out,
    C -> Prints the current Mandelbrot set view; the center and scale,
    Key1 -> Renders VIEW_1,
    Key2 -> Renders VIEW_2,
    Key3 -> Renders VIEW_3,
    Key4 -> Renders VIEW_4,
    Key5 -> Renders VIEW_5,
    Key6 -> Renders VIEW_6,
    Key7 -> Renders VIEW_7,
    Key8 -> Renders VIEW_8,
    Key9 -> Renders VIEW_9,
    K -> Prints the keybindings,
    S -> Saves the current Mandelbrot set view as an image in the saved folder,
    I -> Manually input a Mandelbrot set view,
    A -> Pick an algorithm to color the Mandelbrot set view,
    M -> Change the Mandelbrot set view max_iterations,
}

</pre>
## Controls
Keys | Action
:---:|:------
<kbd>↑</kbd>, <kbd>↓</kbd>, <kbd>←</kbd>, <kbd>→</kbd> | Move up, down, left, or right
<kbd>R</kbd> | Reset the Mandelbrot set view to the starting view
<kbd>[</kbd> | Zoom in
<kbd>]</kbd> | Zoom out
<kbd>C</kbd> | Prints the current Mandelbrot set view; the center and scale
<kbd>0</kbd>, ...,  <kbd>9</kbd> | Render a preconfigured view
<kbd>K</kbd> | Print the keybindings 
<kbd>S</kbd> | Saves the current Mandelbrot set view as an image
<kbd>I</kbd> | Manually input a Mandelbrot set view
<kbd>A</kbd> | Pick an algorithm to color the Mandelbrot set view
<kbd>M</kbd> | Change the Mandelbrot set view max_iterations
<kbd>ESC</kbd>, <kbd>CTRL</kbd>+<kbd>C</kbd> | Exit

## Screenshots
![1.png](images/1.png)
![2.png](images/2.png)
![3.png](images/3.png)
![4.png](images/4.png)
![5.png](images/5.png)
![6.png](images/6.png)
![7.png](images/7.png)
![8.png](images/8.png)
![9.png](images/9.png)
![10.png](images/10.png)
![11.png](images/11.png)
![12.png](images/12.png)
![13.png](images/13.png)
![14.png](images/14.png)
![15.png](images/15.png)

## Wallpapers
### 3440x1440
![2023-06-26_12-48-59 189737_UTC](https://github.com/jortrr/mandelbrot/assets/38651009/d487285e-d233-4374-bb5c-c46d84f0d83f)
![2023-06-26_12-35-03 632016400_UTC](https://github.com/jortrr/mandelbrot/assets/38651009/ccad4f4a-1bc3-455b-98d9-53c5c0f85db3)
![2023-06-26_15-36-32 408135800_UTC](https://github.com/jortrr/mandelbrot/assets/38651009/9baa2097-7cd7-4e53-b799-bfeab95c14dd)
![2023-06-26_15-25-34 763976800_UTC](https://github.com/jortrr/mandelbrot/assets/38651009/49c02944-1d34-493c-a047-6478b4046052)
![2023-06-26_12-56-26 498671100_UTC](https://github.com/jortrr/mandelbrot/assets/38651009/91813670-bcdd-480d-bb28-0fd68f8dad10)
![2023-06-26_08-54-45 849997300_UTC](https://github.com/jortrr/mandelbrot/assets/38651009/0f28dfac-ed62-4096-8c9a-73370a2d69fb)

### 2560x1600
![2023-06-24 18:50:23 842186828 UTC](https://github.com/jortrr/mandelbrot/assets/38651009/03ce23c3-770e-444a-b163-aa247cfeda7c)
![2023-06-29_11-45-50 778868321_UTC](https://github.com/jortrr/mandelbrot/assets/38651009/845cf9cf-3ac5-4df7-8150-abdd227af18a)
![2023-06-29_11-46-06 821888383_UTC](https://github.com/jortrr/mandelbrot/assets/38651009/4f369655-c36d-42e3-8864-0ae3cf998854)
![2023-06-29_11-48-21 628194507_UTC](https://github.com/jortrr/mandelbrot/assets/38651009/57c09c54-5616-487d-a7e8-66e69d8009db)

### 1170x2532 (Iphone 13)
![2023-06-24 19:29:53 436765840 UTC](https://github.com/jortrr/mandelbrot/assets/38651009/9b67dcdb-9dc3-4646-bbb4-96d7dc3ddb8f)
