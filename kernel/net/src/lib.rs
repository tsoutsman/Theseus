#![no_std]

extern crate alloc;

mod error;

use alloc::collections::BTreeMap;
use nic_buffers::{ReceivedFrame, TransmitBuffer};
use smoltcp::{iface, phy, wire};

pub use error::{Error, Result};
pub use phy::DeviceCapabilities;
pub use wire::{EthernetAddress, IpAddress, IpCidr};

pub struct Interface<'a, T>
where
    T: Device,
    DeviceWrapper<'a, T>: for<'d> phy::Device<'d>,
{
    inner: iface::Interface<'static, DeviceWrapper<'a, T>>,
}

impl<'a, T> Interface<'a, T>
where
    T: Device,
    // TODO: Remove this bound so that DeviceWrapper (and TxToken/RxToken) don't have to be public.
    DeviceWrapper<'a, T>: for<'d> phy::Device<'d>,
{
    pub fn new(card: &'a mut T, ip: wire::IpCidr, gateway: wire::IpCidr) -> Result<Self> {
        // FIXME: Check if unicast?
        let hardware_addr = EthernetAddress(card.mac_address()).into();
        let mut routes = iface::Routes::new(BTreeMap::new());

        match gateway.address() {
            IpAddress::Ipv4(addr) => routes.add_default_ipv4_route(addr),
            IpAddress::Ipv6(addr) => routes.add_default_ipv6_route(addr),
            _ => return Err(Error::InvalidGateway),
        }?;

        Ok(Self {
            inner: iface::InterfaceBuilder::new(DeviceWrapper(card), [])
                // FIXME: Use an actual random seed
                .random_seed(0)
                .hardware_addr(hardware_addr)
                .ip_addrs([ip])
                .routes(routes)
                .neighbor_cache(iface::NeighborCache::new(BTreeMap::new()))
                .finalize(),
        })
    }
}

pub trait Device {
    fn send(
        &mut self,
        transmit_buffer: nic_buffers::TransmitBuffer,
    ) -> core::result::Result<(), &'static str>;

    fn receive(&mut self) -> Option<ReceivedFrame>;

    fn poll_receive(&mut self) -> core::result::Result<(), &'static str>;

    fn mac_address(&self) -> [u8; 6];

    fn capabilities(&self) -> DeviceCapabilities {
        DeviceCapabilities::default()
    }
}

/// Wrapper around a NIC.
///
/// We use this because we can't directly implement [`phy::Device`] for `T` due
/// to the following error:
/// ```
/// error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
/// ```
#[doc(hidden)]
pub struct DeviceWrapper<'a, T>(&'a mut T)
where
    T: 'a + Device;

impl<'a, T> phy::Device<'a> for DeviceWrapper<'a, T>
where
    T: 'a + Device,
{
    type RxToken = RxToken;

    type TxToken = TxToken<'a, T>;

    fn receive(&'a mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        let received_frame = self.0.receive()?;
        Some((RxToken(received_frame), TxToken(self.0)))
    }

    fn transmit(&'a mut self) -> Option<Self::TxToken> {
        Some(TxToken(self.0))
    }

    fn capabilities(&self) -> DeviceCapabilities {
        self.0.capabilities()
    }
}

#[doc(hidden)]
pub struct RxToken(ReceivedFrame);

impl phy::RxToken for RxToken {
    fn consume<R, F>(mut self, _timestamp: smoltcp::time::Instant, f: F) -> smoltcp::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>,
    {
        // FIXME: Support handling frame of multiple buffers
        f(self.0 .0[0].as_slice_mut())
    }
}

#[doc(hidden)]
pub struct TxToken<'a, T>(&'a mut T)
where
    T: 'a + Device;

impl<'a, T> phy::TxToken for TxToken<'a, T>
where
    T: 'a + Device,
{
    fn consume<R, F>(
        self,
        _timestamp: smoltcp::time::Instant,
        len: usize,
        f: F,
    ) -> smoltcp::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>,
    {
        // FIXME: Unwraps
        let len = u16::try_from(len).unwrap();
        let mut buf = TransmitBuffer::new(len).unwrap();
        let r = f(buf.as_slice_mut())?;
        self.0.send(buf).unwrap();
        Ok(r)
    }
}
