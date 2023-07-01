use std::fmt;

use minifb::Key;

//https://stackoverflow.com/questions/68066875/how-to-store-a-closure-inside-rust-struct
//https://stackoverflow.com/questions/65756096/how-can-i-store-a-closure-object-in-a-struct
pub struct KeyAction {
    pub key: Key,
    pub description: &'static str,
    action: Box<dyn Fn()>,
}

impl KeyAction {
    pub fn new(key: Key, description: &'static str, action: Box<dyn Fn()>) -> KeyAction {
        KeyAction {
            key,
            description,
            action,
        }
    }

    ///Run self.action
    pub fn action(&self) {
        (self.action)();
    }
}

impl fmt::Debug for KeyAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f,"{:?} -> {}", &self.key, &self.description)
    }
}

pub struct KeyBindings {
    key_actions: Vec<KeyAction>,
}

impl KeyBindings {
    pub fn new(key_actions: Vec<KeyAction>) -> KeyBindings {
        KeyBindings { key_actions }
    }

    ///Adds a KeyAction to these KeyBindings, will remove any existing KeyAction x where x.key == key_action.key
    pub fn add_key(&mut self, key_action: KeyAction) {
        //Remove any KeyAction x where x.key == key_action.key
        self.key_actions.retain(|x| x.key != key_action.key);

        self.key_actions.push(key_action);
    }

    ///Adds a KeyAction to these KeyBindings, will remove any existing KeyAction x where x.key == key
    pub fn add<F: Fn() + 'static>(&mut self, key: Key, description: &'static str, action: F) {
        self.add_key(KeyAction::new(key, description, Box::new(action)));
    }

    pub fn key_actions(&self) -> &Vec<KeyAction> {
        &self.key_actions
    }

    /// Prints all KeyActions in these KeyBindings to stdout
    pub fn print(&self) {
        println!("{:?}",self);
    }

    pub fn print_key(&self, key: &Key) {
        for key_action in &self.key_actions {
            if key_action.key == *key {
                println!("{:?}", key_action);
                return;
            }
        }
        //KeyBindings does not contain a KeyAction x with x.key == key, unbound
        println!("{:?}", key);
    }
}

impl fmt::Debug for KeyBindings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "KeyBindings {{")?;
        for key_action in &self.key_actions {
            writeln!(f, "    {:?},", key_action)?;
        }
        write!(f, "}}")
    }
}

impl Default for KeyBindings {
    /// Define all your keybindings here
    fn default() -> KeyBindings {
        let mut key_bindings = KeyBindings::new(Vec::new());
        key_bindings.add(Key::A, "This is the A key", ||println!("Action A"));
        key_bindings.add(Key::B, "This is the B key", ||println!("Action B"));
        key_bindings
    }
}
