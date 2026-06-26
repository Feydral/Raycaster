#![allow(dead_code)]

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::collections::HashSet;
use std::time::Duration;

pub struct Input {
    held_keys: HashSet<KeyCode>,
    down_keys: HashSet<KeyCode>,
    up_keys: HashSet<KeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            held_keys: HashSet::new(),
            down_keys: HashSet::new(),
            up_keys: HashSet::new(),
        }
    }

    pub fn update(&mut self) -> std::io::Result<()> {
        self.down_keys.clear();
        self.up_keys.clear();

        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                match kind {
                    crossterm::event::KeyEventKind::Press => {
                        if !self.held_keys.contains(&code) {
                            self.down_keys.insert(code);
                            self.held_keys.insert(code);
                        }
                    }
                    crossterm::event::KeyEventKind::Release => {
                        if self.held_keys.contains(&code) {
                            self.up_keys.insert(code);
                            self.held_keys.remove(&code);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.held_keys.contains(&key)
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.down_keys.contains(&key)
    }

    pub fn is_key_up(&self, key: KeyCode) -> bool {
        self.up_keys.contains(&key)
    }
}
