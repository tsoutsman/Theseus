//! This application pings a specific IPv4 address and gets ping statistics.
//! Important: QEMU does not support the ICMP protocol by default so it's
//! important to run this command: sudo sh -c "echo \"0 2147483647\" >
//! /proc/sys/net/ipv4/ping_group_range" in the environment prior to running
//! this application

#![no_std]

extern crate alloc;

use alloc::{string::String, vec, vec::Vec};
use net::{
    socket::{IcmpPacketMetadata, IcmpSocket, IcmpSocketBuffer},
    IpAddress,
};

pub fn main(args: Vec<String>) -> isize {
    let address: IpAddress = args[1].parse().unwrap();
    let iface = net::get_interface(None).unwrap();

    let icmp_rx_buffer = IcmpSocketBuffer::new(vec![IcmpPacketMetadata::EMPTY], vec![0; 256]);
    let icmp_tx_buffer = IcmpSocketBuffer::new(vec![IcmpPacketMetadata::EMPTY], vec![0; 256]);
    let icmp_socket = IcmpSocket::new(icmp_rx_buffer, icmp_tx_buffer);

    let socket_handle = iface.lock().add_socket(icmp_socket);

    loop {
        let socket = iface.lock().get_socket::<IcmpSocket>(socket_handle);
        socket.is_open();
    }

    0
}
