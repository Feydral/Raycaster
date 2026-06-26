#![allow(dead_code)]
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyboardEnhancementFlags,
    PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, supports_keyboard_enhancement};
use std::collections::HashSet;
use std::time::Duration;

pub struct Input {
    held_keys: HashSet<KeyCode>,
    down_keys: HashSet<KeyCode>,
    up_keys: HashSet<KeyCode>,
    kitty_enabled: bool,
}

impl Input {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();

        #[cfg(target_os = "windows")]
        let kitty_enabled = false;

        #[cfg(not(target_os = "windows"))]
        let kitty_enabled = {
            assert!(
                matches!(supports_keyboard_enhancement(), Ok(true)),
                "Terminal does not support kitty keyboard protocol"
            );
            execute!(
                std::io::stdout(),
                PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
            )
            .unwrap();
            true
        };

        Self {
            held_keys: HashSet::new(),
            down_keys: HashSet::new(),
            up_keys: HashSet::new(),
            kitty_enabled,
        }
    }

    pub fn update(&mut self) -> std::io::Result<()> {
        self.down_keys.clear();
        self.up_keys.clear();
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                match kind {
                    KeyEventKind::Press => {
                        if self.held_keys.insert(code) {
                            self.down_keys.insert(code);
                        }
                    }
                    KeyEventKind::Release => {
                        if self.held_keys.remove(&code) {
                            self.up_keys.insert(code);
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

impl Drop for Input {
    fn drop(&mut self) {
        if self.kitty_enabled {
            let _ = execute!(std::io::stdout(), PopKeyboardEnhancementFlags);
        }
        disable_raw_mode().unwrap();
    }
}
