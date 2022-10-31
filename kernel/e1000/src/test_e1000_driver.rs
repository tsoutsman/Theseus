use super::{NetworkInterfaceCard, TransmitBuffer, E1000_NIC};

pub fn test_e1000_nic_driver(_: Option<u64>) {
    match dhcp_request_packet() {
        Ok(_) => debug!("test_e1000_nic_driver(): sent DHCP request packet successfully!"),
        Err(e) => {
            error!("test_e1000_nic_driver(): failed to send DHCP request packet: error {:?}", e)
        }
    };
}

#[repr(C, packed)]
pub struct arp_packet {
    pub dest1: u16, //set to broadcast ff:ff:...
    pub dest2: u16,
    pub dest3: u16,
    pub source1: u16,
    pub source2: u16,
    pub source3: u16,
    pub packet_type: u16,
    pub h_type: u16,
    pub p_type: u16,
    pub hlen: u8,
    pub plen: u8,
    pub oper: u16,
    pub sha1: u16, //sender hw address, first 2 bytes
    pub sha2: u16, // ", next 2 bytes
    pub sha3: u16, // ", last 2 bytes
    pub spa1: u16, // sender protocol address, first 2 B
    pub spa2: u16, // ", last 2 B
    pub tha1: u16, //target ", first
    pub tha2: u16, // ", next
    pub tha3: u16, // ", last
    pub tpa1: u16, // ", first
    pub tpa2: u16, // ", last
}

//should test packet transmission and reception as QEMU DHCP server will respond

//will only b able to see this Tx message in netdump.pcap if user is not mentioned in QEMU flags of Makefile
//QEMU_FLAGS += -net nic,vlan=0,model=e1000,macaddr=00:0b:82:01:fc:42 -net dump,file=netdump.pcap

//will only receive a response if user is mentioned in qemu flags
//QEMU_FLAGS += -net nic,vlan=1,model=e1000,macaddr=00:0b:82:01:fc:42 -net user,vlan=1 -net dump,file=netdump.pcap

//or else use a tap interface (default)
//QEMU_FLAGS += -device e1000,netdev=network0,mac=52:55:00:d1:55:01 -netdev tap,id=network0,ifname=tap0,script=no,downscript=no
//will receive a DHCP messgae from 00:1f:c6:9c:89:4c

pub fn dhcp_request_packet() -> Result<(), &'static str> {
    let packet: [u8; 314] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x1f, 0xc6, 0x9c, 0x89, 0x4c, 0x08, 0x00, 0x45,
        0x00, 0x01, 0x2c, 0xa8, 0x36, 0x00, 0x00, 0xfa, 0x11, 0x17, 0x8b, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0xff, 0xff, 0x00, 0x44, 0x00, 0x43, 0x01, 0x18, 0x59, 0x1f, 0x01, 0x01, 0x06,
        0x00, 0x00, 0x00, 0x3d, 0x1d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0xc6, 0x9c, 0x89,
        0x4c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x82, 0x53, 0x63, 0x35, 0x01, 0x01,
        0x3d, 0x07, 0x01, 0x00, 0x1f, 0xc6, 0x9c, 0x89, 0x4c, 0x32, 0x04, 0x00, 0x00, 0x00, 0x00,
        0x37, 0x04, 0x01, 0x03, 0x06, 0x2a, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let mut transmit_buffer = TransmitBuffer::new(packet.len() as u16)?;
    {
        let buffer: &mut [u8] = transmit_buffer.as_slice_mut(0, 314)?;
        buffer.copy_from_slice(&packet);
    }
    let mut e1000_nic = E1000_NIC.get().ok_or("e1000 NIC hasn't been initialized yet")?.lock();
    e1000_nic.send_packet(transmit_buffer)
}
