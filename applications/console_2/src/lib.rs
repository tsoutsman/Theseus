#![no_std]

extern crate alloc;

mod input;

use alloc::{borrow::ToOwned, format, vec::Vec};
use async_channel::Receiver;
use serial_port::{get_serial_port, DataChunk, SerialPortAddress};
use shell::Input;
use task::JoinableTaskRef;

pub fn init() -> Result<JoinableTaskRef, &'static str> {
    let (sender, receiver) = async_channel::new_channel(4);
    serial_port::set_connection_listener(sender);
    spawn::new_task_builder(connection_detector, receiver)
        .name("serial_port_connection_detector".to_owned())
        .spawn()
}

fn connection_detector(listener: Receiver<SerialPortAddress>) {
    loop {
        let address = listener.receive().unwrap();

        // TODO: Check if ignored serial port
        let port = match get_serial_port(address) {
            Some(sp) => sp.clone(),
            _ => continue,
        };

        let (sender, receiver) = async_channel::new_channel(16);
        if port.lock().set_data_sender(sender).is_err() {
            continue;
        }

        let _ = spawn::new_task_builder(connection_handler, receiver)
            .name(format!("console_loop_{:?}", address))
            .spawn();
    }
}

fn connection_handler(receiver: Receiver<DataChunk>) {
    let mut input = input::SerialInput { receiver, parser: vte::Parser::new(), events: Vec::new() };
    loop {
        if let Some(event) = input.event() {
            log::debug!("{event:#?}");
        }
    }
}
