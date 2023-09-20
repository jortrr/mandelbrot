use std::fmt;

use minifb::Key;

static KEY_BINDING_DESCRIPTION_OFFSET: usize = 20;
static KEY_BINDING_SEPARATOR: &str = " -> ";

//https://stackoverflow.com/questions/68066875/how-to-store-a-closure-inside-rust-struct
//https://stackoverflow.com/questions/65756096/how-can-i-store-a-closure-object-in-a-struct
pub struct KeyBinding {
    pub key: Key,
    pub description: &'static str,
    action: Box<dyn Fn()>,
}

impl KeyBinding {
    pub fn new(key: Key, description: &'static str, action: Box<dyn Fn()>) -> KeyBinding {
        KeyBinding { key, description, action }
    }

    ///Run self.action
    pub fn run(&self) {
        (self.action)();
    }

    pub fn to_formatted_str(&self) -> String {
        let mut key_binding_formatted = format!("{:?}", self);
        while key_binding_formatted.contains("  ") {
            key_binding_formatted = key_binding_formatted.replace("  ", " ");
        }
        key_binding_formatted
    }
}

impl fmt::Debug for KeyBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key = format!("{:?}", &self.key);
        let offset = KEY_BINDING_DESCRIPTION_OFFSET - key.len() - KEY_BINDING_SEPARATOR.len();
        write!(f, "{}{}{}{}", key, KEY_BINDING_SEPARATOR, " ".repeat(offset), &self.description)
    }
}

pub struct KeyBindings {
    key_bindings: Vec<KeyBinding>,
}

impl KeyBindings {
    pub fn new(key_bindings: Vec<KeyBinding>) -> KeyBindings {
        KeyBindings { key_bindings }
    }

    ///Adds a `KeyBinding` to these `KeyBindings`, will remove any existing `KeyBinding` `x` where `x.key` == `key_action.key`
    pub fn add_key(&mut self, key_action: KeyBinding) {
        //Remove any KeyBinding x where x.key == key_action.key
        self.key_bindings.retain(|x| x.key != key_action.key);

        self.key_bindings.push(key_action);
    }

    ///Adds a `KeyBinding` to these `KeyBindings`, will remove any existing `KeyBinding` `x` where `x.key` == `key`
    pub fn add<F: Fn() + 'static>(&mut self, key: Key, description: &'static str, action: F) {
        self.add_key(KeyBinding::new(key, description, Box::new(action)));
    }

    pub fn key_actions(&self) -> &Vec<KeyBinding> {
        &self.key_bindings
    }

    /// Prints all `KeyBinding`s in these `KeyBindings` to stdout
    pub fn print(&self) {
        println!("{:?}", self);
    }

    pub fn print_key(&self, key: &Key) {
        for key_action in &self.key_bindings {
            if key_action.key == *key {
                print!("{}", key_action.to_formatted_str());
                return;
            }
        }
        //KeyBindings does not contain a KeyBinding x with x.key == key, unbound
        print!("{:?}", key);
    }

    pub fn run(&self, key: &Key) {
        for key_binding in &self.key_bindings {
            if key_binding.key == *key {
                key_binding.run();
                break;
            }
        }
    }
}

impl fmt::Debug for KeyBindings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "KeyBindings {{")?;
        for key_binding in &self.key_bindings {
            writeln!(f, "    {:?},", key_binding)?;
        }
        write!(f, "}}")
    }
}

impl Default for KeyBindings {
    /// Define all your keybindings here
    fn default() -> KeyBindings {
        let mut key_bindings = KeyBindings::new(Vec::new());
        key_bindings.add(Key::A, "This is the A key", || println!("Action A"));
        key_bindings.add(Key::B, "This is the B key", || println!("Action B"));
        key_bindings
    }
}
