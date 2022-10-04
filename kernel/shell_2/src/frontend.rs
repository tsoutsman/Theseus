pub trait Frontend {
    type Cursor: Cursor;
    type Input: Input;

    /// Insert a character at the cursor.
    fn push(&mut self, c: char);

    /// Remove a character at the cursor.
    ///
    /// If the delete key was pressed, `in_front` should be `true`. If the
    /// backspace key was pressed, it should be `false`.
    fn pop(&mut self, in_front: bool);

    /// Prints a string at the cursor.
    fn push_str(&mut self, string: &str);

    /// Clears the screen.
    fn clear(&mut self);

    fn cursor(&self) -> &Self::Cursor;
    fn cursor_mut(&mut self) -> &mut Self::Cursor;

    fn resize(&mut self, size: crate::Rectangle);

    fn to_begin(&mut self);
    fn to_end(&mut self);

    fn line_up(&mut self);
    fn line_down(&mut self);

    fn page_up(&mut self);
    fn page_down(&mut self);

    fn refresh(&mut self);
}

pub trait Input {
    fn event(&mut self) -> Option<crate::Event>;
}

pub trait Cursor {
    fn enable(&mut self);
    fn disable(&mut self);

    fn offset(&self) -> usize;

    fn left(&mut self);
    fn right(&mut self);

    fn leftmost(&mut self);
    fn rightmost(&mut self);
}

pub struct Rectangle;
