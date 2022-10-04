// use vte::Perform;

// use crate::{ScreenCursor, TerminalActionHandler, TerminalBackend, TextTerminal};

// impl<T> shell::Frontend for TextTerminal<T>
// where
//     T: TerminalBackend,
// {
//     type Cursor = ScreenCursor;

//     fn insert_char(&mut self, c: char, offset_from_end: usize) {
//         todo!()
//     }

//     fn remove_char(&mut self, offset_from_end: usize) {
//         todo!()
//     }

//     fn print(&mut self, string: &str) {
//         let mut handler = TerminalActionHandler::from_terminal(self);
//         for c in string.chars() {
//             handler.print(c);
//         }
//     }

//     fn clear(&mut self) {
//         todo!()
//     }

//     fn cursor(&self) -> &Self::Cursor {
//         &self.screen_cursor
//     }

//     fn cursor_mut(&mut self) -> &mut Self::Cursor {
//         &mut self.screen_cursor
//     }

//     fn resize(&mut self, size: shell::Rectangle) {
//         todo!()
//     }

//     fn to_begin(&mut self) {
//         todo!()
//     }

//     fn to_end(&mut self) {
//         todo!()
//     }

//     fn line_up(&mut self) {
//         todo!()
//     }

//     fn line_down(&mut self) {
//         todo!()
//     }

//     fn page_up(&mut self) {
//         todo!()
//     }

//     fn page_down(&mut self) {
//         todo!()
//     }

//     fn event(&mut self) -> Option<shell::Event> {
//         todo!()
//     }

//     fn refresh(&mut self) {
//         todo!()
//     }
// }

// impl shell::Cursor for ScreenCursor {
//     fn enable(&mut self) {
//         todo!()
//     }

//     fn disable(&mut self) {
//         todo!()
//     }

//     fn offset(&self) -> usize {
//         todo!()
//     }

//     fn set_offset(&mut self, offset: usize) {
//         todo!()
//     }
// }
