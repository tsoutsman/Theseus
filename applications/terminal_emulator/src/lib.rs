#![no_std]

extern crate alloc;

use alloc::{format, string::String, sync::Arc, vec::Vec};
use core2::io;
use keycodes_ascii::KeyAction;
use log::{debug, error, warn};
use shapes::Coord;
use text_display::TextDisplay;
use tty::{Master, Tty};
use window::{Event, KeyboardInputEvent, Window};

const FONT_FOREGROUND_COLOR: color::Color = color::LIGHT_GREEN;
const FONT_BACKGROUND_COLOR: color::Color = color::BLACK;

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

    let tty = Tty::new();

    let mut emulator = Emulator {
        master: tty.master(),
        text_display,
        window,
        cursor: Cursor::default(),
        saved_cursor: None,
    };

    let task = spawn::new_task_builder(shell::main, Vec::new())
        // TODO: Use unique name
        .name(format!("shell"))
        .block()
        .spawn()
        .unwrap();

    let id = task.id;
    let stream = Arc::new(tty.slave());
    app_io::insert_child_streams(
        id,
        app_io::IoStreams {
            discipline: Some(stream.discipline()),
            stdin: stream.clone(),
            stdout: stream.clone(),
            stderr: stream,
        },
    );

    task.unblock();

    let mut parser = vte::Parser::new();

    loop {
        while let Some(event) = emulator.event()? {
            match event {
                Event::KeyboardEvent(KeyboardInputEvent { key_event }) => {
                    if key_event.action == KeyAction::Pressed {
                        if let Some(ascii) = key_event.keycode.to_ascii(key_event.modifiers) {
                            emulator
                                .write_byte(ascii)
                                .map_err(|_| "failed to write to master")?;
                        } else {
                            warn!("terminal_emulator: couldn't convert keycode to ascii {key_event:?}");
                        }
                    }
                }
                Event::OutputEvent(_) => todo!(),
                Event::WindowResizeEvent(_) => todo!(),
                Event::ExitEvent => todo!(),
                Event::MousePositionEvent(_) => {}
                Event::MouseMovementEvent(_) => {}
            }
        }

        let mut buf = [0; 256];
        let len = match emulator.try_read(&mut buf) {
            Ok(len) => len,
            Err(e) => {
                if e.kind() == core2::io::ErrorKind::WouldBlock {
                    continue;
                } else {
                    return Err("failed to read from master");
                }
            }
        };

        for byte in &buf[..len] {
            parser.advance(&mut emulator, *byte);
        }

        // TODO: Refresh window
    }
}

struct Emulator {
    master: Master,
    text_display: TextDisplay,
    window: Window,
    cursor: Cursor,
    saved_cursor: Option<Cursor>,
}

impl Emulator {
    fn event(&mut self) -> Result<Option<Event>, &'static str> {
        self.window.handle_event()
    }

    fn write_byte(&mut self, byte: u8) -> io::Result<()> {
        self.master.write_byte(byte)
    }

    fn try_read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.master.try_read(buf)
    }
}

// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

impl vte::Perform for Emulator {
    fn print(&mut self, c: char) {
        debug!("[print]: {c:?}");
    }

    fn execute(&mut self, byte: u8) {
        debug!("[execute]: {byte:?}");
        match byte {
            0x7 => todo!("execute bell"),
            0x8 => todo!("execute backspace"),
            b'\t' => todo!("execute tab"),
            b'\n' => todo!("execute line feed"),
            b'\r' => self.cursor.column = 0,
            0x7f => todo!("execute delete"),
            _ => warn!("[execute]: unknown byte {byte:?}"),
        }
    }

    fn hook(&mut self, params: &vte::Params, intermediates: &[u8], ignore: bool, action: char) {
        debug!("[hook]: {params:?}, {intermediates:?}, {ignore}, {action:?}");
    }

    fn put(&mut self, byte: u8) {
        debug!("[put]: {byte:?}")
    }

    fn unhook(&mut self) {
        debug!("[unhook]");
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        debug!("[osc_dispatch]: {params:?}, {bell_terminated}");
    }

    // CSI = ESC[
    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        intermediates: &[u8],
        ignore: bool,
        action: char,
    ) {
        debug!("[csi_dispatch]: {params:?}, {intermediates:?}, {ignore}, {action:?}");

        if let Ok((param_1, param_2)) = parse_params(params) {
            match (param_1, param_2, action) {
                // Cursor controls
                (None, None, 'H') => todo!("move cursor to home"),
                (Some(row), Some(column), 'H' | 'f') => {
                    let (width, height) = self.text_display.get_dimensions();
                    let row = core::cmp::min((width - 1) as u16, row);
                    let column = core::cmp::min((height - 1) as u16, column);

                    self.cursor = Cursor { row, column };
                }
                (Some(num), None, 'A') => todo!("move cursor up {num} lines"),
                (Some(num), None, 'B') => todo!("move cursor down {num} lines"),
                (Some(num), None, 'C') => todo!("move cursor right {num} columns"),
                (Some(num), None, 'D') => todo!("move cursor left {num} columns"),
                (Some(num), None, 'E') => {
                    todo!("move cursor to beggining of line, {num} lines down")
                }
                (Some(num), None, 'F') => todo!("move cursor to beggining of line, {num} lines up"),
                (Some(num), None, 'G') => todo!("move cursor to column {num}"),
                (Some(6), None, 'n') => {
                    // TODO: Don't cast
                    self.master
                        .write(&[
                            0x1b,
                            b'[',
                            self.cursor.row as u8,
                            b';',
                            self.cursor.column as u8,
                            b'R',
                        ])
                        .unwrap();
                }
                (None, None, 's') => self.saved_cursor = Some(self.cursor.clone()),
                (None, None, 'u') => self.cursor = self.saved_cursor.unwrap_or(Cursor::default()),

                // Erase functions
                (None | Some(0), None, 'J') => todo!("erase from cursor until end of screen"),
                (Some(1), None, 'J') => todo!("erase from cursor to beggining of screen"),
                (Some(2), None, 'J') => todo!("erase entire screen"),
                (Some(3), None, 'J') => todo!("erase saved lines"),
                (None | Some(0), None, 'K') => todo!("erase from cursor to end of line"),
                (Some(1), None, 'K') => todo!("erase start of line to the cursor"),
                (Some(2), None, 'K') => todo!("erase the entire line"),

                _ => todo!(),
            }
        } else {
            warn!("[csi_dispatch]: failed to parse params {params:?}");
        }
    }

    // ESC (no [)
    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        debug!("[esc_dispatch]: {intermediates:?}, {ignore}, {byte:?}");

        match byte {
            b'M' => todo!("move cursor one line up, scrolling if needed"),
            b'7' => self.saved_cursor = Some(self.cursor.clone()),
            b'8' => self.cursor = self.saved_cursor.unwrap_or(Cursor::default()),
            _ => todo!(),
        }
    }
}

fn parse_params(params: &vte::Params) -> Result<(Option<u16>, Option<u16>), ()> {
    let mut iter = params.into_iter();

    let (mut param_1_iter, mut param_2_iter) = (
        iter.next().map(|array| array.iter()),
        iter.next().map(|array| array.iter()),
    );
    let (param_1, param_2) = (
        param_1_iter
            .as_mut()
            .map(|iter| iter.next())
            .flatten()
            .copied(),
        param_2_iter
            .as_mut()
            .map(|iter| iter.next())
            .flatten()
            .copied(),
    );

    if iter.next().is_some()
        || param_1_iter
            .map(|mut iter| iter.next().is_some())
            .unwrap_or(false)
        || param_2_iter
            .map(|mut iter| iter.next().is_some())
            .unwrap_or(false)
    {
        Err(())
    } else {
        Ok((param_1, param_2))
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Cursor {
    column: u16,
    row: u16,
}
