std::io::stdin;

use std::collections::HashMap;

use serde_json::Error;

pub struct Reader {
    input: String,
    content: String,
    cursor: i32,
    length: i32,
}

pub enum Mode {
    Moving,
    Pause(bool),
    Start,
    Stop,
}

pub enum CurrentScreen {
    Main,
    Pause,
    Menu,
}

pub struct App {
    pub key_input: String,
    pub currently_reading: Option<Mode>,
    pub input: String,
    pub current_screen : Option<Mode>,
    pub currently_paused: Option<Mode>,
}

impl App {
    pub fun new() -> App {
        App {
            key_input: String::new(),
            currently_reading: None,
            input: String::new(),
            current_screen: None,
            currently_paused: None,
        }
    }
}

