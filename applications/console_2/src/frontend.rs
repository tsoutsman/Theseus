use crate::input::Input;
use alloc::sync::Arc;
use text_terminal::TtyBackend;
use core::fmt::Write;
use irq_safety::MutexIrqSafe;
use serial_port::SerialPort;

pub(crate) struct Frontend {
    output: Arc<MutexIrqSafe<SerialPort>>,
    cursor: Cursor,
}

impl Frontend {
    pub(crate) fn new(output: Arc<MutexIrqSafe<SerialPort>>) -> Self {
        Self { output, cursor: Cursor }
    }
}

impl shell::Frontend for Frontend {
    type Cursor = Cursor;

    type Input = Input;

    fn push(&mut self, c: char) {
        self.output.lock().write_char(c).unwrap();
    }

    fn pop(&mut self, in_front: bool) {
        todo!()
    }

    fn push_str(&mut self, string: &str) {
        self.output.lock().write_str(string).unwrap();
    }

    fn clear(&mut self) {
        todo!()
    }

    fn cursor(&self) -> &Self::Cursor {
        todo!()
    }

    fn cursor_mut(&mut self) -> &mut Self::Cursor {
        todo!()
    }

    fn resize(&mut self, size: shell::Rectangle) {
        todo!()
    }

    fn to_begin(&mut self) {
        todo!()
    }

    fn to_end(&mut self) {
        todo!()
    }

    fn line_up(&mut self) {
        todo!()
    }

    fn line_down(&mut self) {
        todo!()
    }

    fn page_up(&mut self) {
        todo!()
    }

    fn page_down(&mut self) {
        todo!()
    }

    fn refresh(&mut self) {}
}

pub(crate) struct Cursor;

impl shell::Cursor for Cursor {
    fn enable(&mut self) {
        todo!();
    }

    fn disable(&mut self) {
        todo!();
    }

    fn offset(&self) -> usize {
        todo!();
    }

    fn left(&mut self) {
        todo!();
    }

    fn right(&mut self) {
        todo!();
    }

    fn leftmost(&mut self) {
        todo!();
    }

    fn rightmost(&mut self) {
        todo!();
    }
}
