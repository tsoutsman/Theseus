#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use log::error;
use shapes::Coord;
use text_display::TextDisplay;
use text_terminal::{Column, Row, ScreenSize, TerminalBackend, TextTerminal};
use window::Window;

pub fn main(_: Vec<String>) -> isize {
    match _main() {
        Ok(()) => 0,
        Err(e) => {
            error!("terminal_emulator: {}", e);
            1
        }
    }
}

pub fn _main() -> Result<(), &'static str> {
    let backend = EmulatorBackend::new()?;
    let (width, height) = backend.text_display.get_dimensions();
    let terminal = TextTerminal::new(width as u16, height as u16, backend);

    Ok(())
}

struct Emulator {
    terminal: TextTerminal<EmulatorBackend>,
}

impl Emulator {
    fn new() -> Result<Self, &'static str> {
        let backend = EmulatorBackend::new()?;
        let (width, height) = backend.text_display.get_dimensions();
        let terminal = TextTerminal::new(width as u16, height as u16, backend);

        Ok(Self { terminal })
    }
}

struct EmulatorBackend {
    window: Window,
    text_display: TextDisplay,
}

impl EmulatorBackend {
    fn new() -> Result<Self, &'static str> {
        const FONT_FOREGROUND_COLOR: color::Color = color::LIGHT_GREEN;
        const FONT_BACKGROUND_COLOR: color::Color = color::BLACK;

        let wm_ref = window_manager::WINDOW_MANAGER
            .get()
            .ok_or("The window manager is not initialized")?;
        let (window_width, window_height) = {
            let wm = wm_ref.lock();
            wm.get_screen_size()
        };
        let window = window::Window::new(
            Coord::new(0, 0),
            window_width,
            window_height,
            FONT_BACKGROUND_COLOR,
        )?;

        let area = window.area();
        let text_display = TextDisplay::new(
            area.width(),
            area.height(),
            FONT_FOREGROUND_COLOR,
            FONT_BACKGROUND_COLOR,
        )?;

        Ok(Self {
            window,
            text_display,
        })
    }
}

impl TerminalBackend for EmulatorBackend {
    type DisplayError = ();

    fn screen_size(&self) -> ScreenSize {
        let (width, height) = self.text_display.get_dimensions();
        ScreenSize {
            num_columns: Column(width as u16),
            num_rows: Row(height as u16),
        }
    }

    fn update_screen_size(&mut self, new_size: ScreenSize) {
        todo!()
    }

    fn display(
        &mut self,
        display_action: text_terminal::DisplayAction,
        scrollback_buffer: &text_terminal::ScrollbackBuffer,
        previous_style: Option<text_terminal::Style>,
    ) -> Result<text_terminal::ScreenPoint, Self::DisplayError> {
        todo!()
    }

    fn move_cursor_to(
        &mut self,
        new_position: text_terminal::ScreenPoint,
    ) -> text_terminal::ScreenPoint {
        todo!()
    }

    fn move_cursor_by(&mut self, num_columns: i32, num_rows: i32) -> text_terminal::ScreenPoint {
        todo!()
    }

    fn set_insert_mode(&mut self, mode: text_terminal::InsertMode) {
        todo!()
    }

    fn reset_screen(&mut self) {
        todo!()
    }

    fn clear_screen(&mut self) {
        todo!()
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        panic!("write_bytes shouldn't be called for the emulator");
    }
}
