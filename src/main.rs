use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode};
use inputbot::MouseButton::LeftButton;
use inputbot::{KeybdKey, MouseCursor};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

static RUN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false); // Global RUN control

fn auto_attack(pos: Arc<Mutex<(i32, i32)>>) {
    let dir_vec = vec![
        (0, 3),
        (3, 3),
        (3, 0),
        (3, -3),
        (0, -3),
        (-3, -3),
        (-3, 0),
        (-3, 3),
    ];

    while RUN.load(std::sync::atomic::Ordering::SeqCst) {
        let current_pos = *pos.lock().unwrap(); // Lock to get position
        MouseCursor::move_abs(current_pos.0, current_pos.1); // Move mouse
        LeftButton.press(); // Simulate left click
        LeftButton.release(); // Simulate left click
        KeybdKey::AKey.press(); // Press 'A' key
        thread::sleep(Duration::from_secs(1));

        for move_dir in dir_vec.iter() {
            MouseCursor::move_rel(move_dir.0, move_dir.1); // Move mouse relative
            LeftButton.press(); // Simulate left click
            LeftButton.release(); // Simulate left click
            KeybdKey::AKey.press(); // Press 'A' key
            thread::sleep(Duration::from_secs(1));
        }
    }
}


fn main() {
    let device_state = DeviceState::new();

    let pos = Arc::new(Mutex::new((0, 0))); // Shared position wrapped in Mutex

    let pos_for_f11 = Arc::clone(&pos); // Clone Arc to share across F11 logic

    let _guard = device_state.on_key_down(move |key| {
        if *key == Keycode::F12 {
            RUN.store(!RUN.load(std::sync::atomic::Ordering::SeqCst), std::sync::atomic::Ordering::SeqCst); // Toggle RUN
            if RUN.load(std::sync::atomic::Ordering::SeqCst) {
                let pos_clone = Arc::clone(&pos);
                thread::spawn(move || {
                    auto_attack(pos_clone);
                });
            }
        }
        if *key == Keycode::F11 {
            // Update pos when F11 is pressed
            let device_state = DeviceState::new();
            let mut pos_lock = pos_for_f11.lock().unwrap();
            *pos_lock = device_state.get_mouse().coords;
        }
    });

    loop {} // Main loop does nothing; keyboard events are handled by the guard
}
