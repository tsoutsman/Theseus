use alloc::vec::Vec;
use async_channel::Receiver;
use serial_port::DataChunk;
use shell::{Event, KeyboardEvent};

pub(crate) struct SerialInput {
    pub(crate) receiver: Receiver<DataChunk>,
    pub(crate) parser: vte::Parser,
    pub(crate) events: Vec<Event>,
}

impl shell::Input for SerialInput {
    fn event(&mut self) -> Option<shell::Event> {
        if let Some(event) = self.events.pop() {
            return Some(event);
        }

        let DataChunk { len, data } = self.receiver.receive().ok()?;
        let mut performer = EventsPerformer(&mut self.events);

        for byte in &data[..len as usize] {
            self.parser.advance(&mut performer, *byte);
        }

        self.events.pop()
    }
}

struct EventsPerformer<'a>(&'a mut Vec<Event>);

impl vte::Perform for EventsPerformer<'_> {
    fn print(&mut self, c: char) {
        self.0.push(Event::Keyboard(match c {
            '\u{7f}' => KeyboardEvent::Backspace,
            _ => KeyboardEvent::Other(c),
        }));
    }

    fn execute(&mut self, byte: u8) {
        self.0.push(Event::Keyboard(match byte {
            0x3 => KeyboardEvent::CtrlC,
            0x4 => KeyboardEvent::CtrlD,
            0x9 => KeyboardEvent::Tab,
            0xd => KeyboardEvent::Enter,
            0x1a => KeyboardEvent::CtrlZ,
            _ => return,
        }));
    }

    fn csi_dispatch(&mut self, params: &vte::Params, _: &[u8], _: bool, action: char) {
        let first_param = params.into_iter().flatten().copied().next().unwrap_or_default();

        self.0.push(Event::Keyboard(match (first_param, action) {
            (0x0, 'A') => KeyboardEvent::Up,
            (0x0, 'B') => KeyboardEvent::Down,
            (0x0, 'C') => KeyboardEvent::Right,
            (0x0, 'D') => KeyboardEvent::Left,
            (0x1, 'H') => KeyboardEvent::Begin,
            (0x1, 'F') => KeyboardEvent::End,
            (0x1, '~') => KeyboardEvent::Leftmost,
            (0x3, '~') => KeyboardEvent::Delete,
            (0x4, '~') => KeyboardEvent::Rightmost,
            (0x5, '~') => KeyboardEvent::PageUp,
            (0x6, '~') => KeyboardEvent::PageDown,
            _ => return,
        }));
    }
}
