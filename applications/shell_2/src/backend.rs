pub trait Backend {
    fn insert_char(&mut self, c: char, offset_from_end: usize);
    fn remove_char(&mut self, offset_from_end: usize);

    fn print_to_terminal(&mut self, string: &str);

    fn cursor(&mut self) -> &mut Cursor;

    fn resize(&mut self, size: Rectangle);
    
    fn to_begin(&mut self);
    fn to_end(&mut self);

    fn line_up(&mut self);
    fn line_down(&mut self);

    fn page_up(&mut self);
    fn page_down(&mut self);
    
    fn refresh(&mut self);
}

pub struct Cursor;

pub struct Rectangle;
