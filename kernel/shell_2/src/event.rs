#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    Keyboard(KeyboardEvent),
    Exit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyboardEvent {
    /// Ctrl + C
    CtrlC,
    /// Ctrl + Z
    CtrlZ,
    /// Ctrl + D
    CtrlD,
    /// Ctrl + Home
    Begin,
    /// Ctrl + End
    End,
    /// Shift + PageUp
    PageUp,
    /// Shift + PageDown
    PageDown,
    /// Ctrl + Shift + Up
    LineUp,
    /// Ctrl + Shift + Down
    LineDown,
    /// Tab
    Tab,
    /// Backspace
    Backspace,
    /// Delete
    Delete,
    /// Enter
    Enter,
    /// Home
    Leftmost,
    /// End
    Rightmost,
    /// Up
    Up,
    /// Down
    Down,
    /// Left
    Left,
    /// Right
    Right,
    /// Other
    Other(char),
}