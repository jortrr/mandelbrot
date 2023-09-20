use std::sync::atomic::{AtomicBool, Ordering};

use minifb::{MouseButton, Window};

fn was_clicked(current: bool, previous: bool) -> bool {
    current && !previous
}

/////Mouse click recorder with interior mutability to toggle mouse clicks;
/// without such a (static function) variable, clicking the screen once would result in multiple actions
pub struct MouseClickRecorder {
    mouse_button: MouseButton,
    previous: AtomicBool,
}

impl MouseClickRecorder {
    pub const fn new(mouse_button: MouseButton) -> MouseClickRecorder {
        MouseClickRecorder {
            mouse_button,
            previous: AtomicBool::new(false),
        }
    }

    ///Returns whether the `mouse_button` was clicked once
    pub fn was_clicked(&self, window: &Window) -> bool {
        let current = window.get_mouse_down(self.mouse_button);
        let previous = self.previous.load(Ordering::Relaxed);
        let result = was_clicked(current, previous);
        if current != previous {
            self.previous.store(current, Ordering::Relaxed)
        }
        result
    }
}
