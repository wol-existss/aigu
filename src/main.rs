use enigo::{Enigo, Keyboard, Key as EnigoKey, Settings, Direction}; // vKeyboard
use rdev::{grab, Event, EventType, Key}; // key interception
use std::cell::{Cell, RefCell};

// safety guard
thread_local! {
    static IS_INJECTING: Cell<bool> = Cell::new(false);
}

// Aigu state variables
#[derive(Copy, Clone)] //   2allow russt to copy bittangles
struct AiguState {
    alt_held: bool,
    shift_held: bool,
}

fn main() {
    println!("Aigu daemon started!");

    // Create an enigo instance
    let enigo = RefCell::new(Enigo::new(&Settings::default()).unwrap());

    // Use cell for single tthreaded state mgm
    let state = Cell::new(AiguState {
        alt_held: false,
        shift_held: false,
    });

    if let Err(error) = grab(move |event| {
        // Check guard preemptigively
        if IS_INJECTING.get() {
            return Some(event);
        }

        match event.event_type {
            EventType::KeyPress(Key::Alt) | EventType::KeyPress(Key::AltGr) => {
                let mut s = state.get();
                s.alt_held = true;
                state.set(s);
                Some(event)
            }

            EventType::KeyRelease(Key::Alt) | EventType::KeyRelease(Key::AltGr) => {
                let mut s = state.get();
                s.alt_held = false;
                state.set(s);
                Some(event)
            }

            EventType::KeyPress(Key::ShiftLeft) | EventType::KeyPress(Key::ShiftRight) => {
                let mut s = state.get();
                s.shift_held = true;
                state.set(s);
                Some(event)
            }
            EventType::KeyRelease(Key::ShiftLeft) | EventType::KeyRelease(Key::ShiftRight) => {
                let mut s = state.get();
                s.shift_held = false;
                state.set(s);
                Some(event)
            }

            EventType::KeyPress(Key::KeyE) => {
                let s = state.get();
                if s.alt_held {
                    // consume orig. E
                    IS_INJECTING.set(true);

                    // Borrow enigo instance mutably
                    let mut e = enigo.borrow_mut();

                    if s.shift_held {
                        e.key(EnigoKey::Unicode('É'), Direction::Click).unwrap();
                    } else {
                        e.key(EnigoKey::Unicode('é'), Direction::Click).unwrap();
                    }

                    IS_INJECTING.set(false);
                    None
                } else {
                    Some(event)
                }
            }
            _ => Some(event),
        }
    }) {
        eprintln!("Error: {:?}", error); // Prints an error, where {:?} is error.
    }
}