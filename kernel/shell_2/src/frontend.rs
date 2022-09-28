pub trait Frontend {
    type Cursor: Cursor;
    type Input: Input;

    fn insert_char(&mut self, c: char, offset_from_end: usize);
    fn remove_char(&mut self, offset_from_end: usize);

    fn print(&mut self, string: &str);
    
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

    fn set_offset(&mut self, offset: usize);
}

pub struct Rectangle;
