pub struct Cursor {
    enabled: bool,
    freq: u128,
    time: (),
    show: bool,
    color: (),
    offset: usize,
    underlying_char: char,
}

// impl Cursor {
//     /// Returns whether the cursor should currently be displayed.
//     ///
//     /// This function is only accurate if invoked in a loop.
//     pub fn blink(&mut self) -> bool {
//         if self.enabled {
//             let time = todo!();
//             if let Some(duration) = time.sub(&(self.time)) {
//                 if let Some(ns) = duration.to_ns() {
//                     if ns >= self.freq {
//                         self.time = time;
//                         self.show = !self.show;
//                         return self.show;
//                     }
//                 }
//             }
//         }
//         false
//     }

//     /// Returns whether the cursor should be displayed.
//     pub fn show(&self) -> bool {
//         self.enabled && self.show
//     }
// }

// impl shell::Cursor for Cursor {
//     fn enable(&mut self) {
//         self.enabled = true;
//         self.reset();
//     }

//     fn disable(&mut self) {
//         self.enabled = false;
//     }

//     fn offset(&self) -> usize {
//         self.offset
//     }

//     fn update(&mut self, offset: usize) {
//         self.offset = offset;
//     }
// }
