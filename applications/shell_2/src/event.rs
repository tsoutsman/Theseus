use alloc::string::String;

pub enum Event {
    CtrlC {
        clear: bool,
    },
    Complete(String),
    Backspace,
    Delete,
    Newline,
    Print(String),
    ScreenBegin,
    ScreenEnd,
    PageUp,
    PageDown,
    LineUp,
    LineDown,
    CursorLeftmost,
    CursorRightmost,
    CursorLeft,
    CursorRight,
}
