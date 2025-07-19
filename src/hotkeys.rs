use device_query::Keycode::{self, F1, F2, F3, Key1, Key2, Key3};

pub fn map_hotkey_to_keycode(hotkey: &str) -> Option<Keycode> {
    match hotkey {
        "Key1" => Some(Key1),
        "Key2" => Some(Key2),
        "Key3" => Some(Key3),
        "F1" => Some(F1),
        "F2" => Some(F2),
        "F3" => Some(F3),
        _ => None, // Unsupported hotkey
    }
}
