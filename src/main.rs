use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;

use enigo::{Button, Coordinate, Direction::Click, Enigo, Key, Keyboard, Mouse, Settings};
use std::sync::{Arc, Mutex};
use std::thread;

static RUN: AtomicBool = AtomicBool::new(false); // Global RUN control

fn auto_attack(pos: Arc<Mutex<(i32, i32)>>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
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
    while RUN.load(Ordering::SeqCst) {
        let current_pos = *pos.lock().unwrap(); // Lock to get position
        enigo
            .move_mouse(current_pos.0, current_pos.1, Coordinate::Abs)
            .expect("failed to move mouse");
        enigo
            .button(Button::Left, Click)
            .expect("TODO: panic message");
        enigo
            .key(Key::Unicode('a'), Click)
            .expect("TODO: panic message");
        sleep(std::time::Duration::from_secs(1));
        for move_dir in dir_vec.iter() {
            enigo
                .move_mouse(move_dir.0, move_dir.1, Coordinate::Rel)
                .expect("TODO: panic message");
            enigo
                .button(Button::Left, Click)
                .expect("TODO: panic message");
            enigo
                .key(Key::Unicode('a'), Click)
                .expect("TODO: panic message");
            sleep(std::time::Duration::from_secs(1));
        }
    }
}

fn main() {
    let device_state = DeviceState::new();

    let pos = Arc::new(Mutex::new((0, 0))); // Shared position wrapped in Mutex

    let pos_for_f11 = Arc::clone(&pos); // Clone Arc to share across F11 logic

    let _guard = device_state.on_key_down(move |key| {
        if *key == Keycode::F12 {
            RUN.store(!RUN.load(Ordering::SeqCst), Ordering::SeqCst); // Toggle RUN
            if RUN.load(Ordering::SeqCst) {
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
