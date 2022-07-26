use crate::{Device, DeviceWrapper, Error, Result};
use alloc::collections::BTreeMap;
use smoltcp::{iface, phy, wire};

pub use wire::{IpAddress, IpCidr};
pub use smoltcp::socket;

pub struct Interface<T>
where
    T: 'static + Device + ?Sized,
    DeviceWrapper<T>: for<'d> phy::Device<'d>,
{
    inner: iface::Interface<'static, DeviceWrapper<T>>,
}

impl<D> Interface<D>
where
    D: 'static + Device + ?Sized,
    // TODO: Remove this bound so that DeviceWrapper (and TxToken/RxToken) don't have to be public.
    DeviceWrapper<D>: for<'d> phy::Device<'d>,
{
    /// Creates a new interface using the specified `device`, `ip`, and `gateway`.
    ///
    /// Returns an error if the gateway is not an Ipv4 or Ipv6 address.
    pub fn new(device: &'static mut D, ip: IpCidr, gateway: IpAddress) -> Result<Self> {
        let hardware_addr = wire::EthernetAddress(device.mac_address()).into();
        let mut routes = iface::Routes::new(BTreeMap::new());

        match gateway {
            IpAddress::Ipv4(addr) => routes.add_default_ipv4_route(addr),
            IpAddress::Ipv6(addr) => routes.add_default_ipv6_route(addr),
            _ => return Err(Error::InvalidGateway),
        }?;

        Ok(Self {
            inner: iface::InterfaceBuilder::new(DeviceWrapper(device), [])
                // FIXME: Use an actual random seed
                .random_seed(0)
                .hardware_addr(hardware_addr)
                .ip_addrs([ip])
                .routes(routes)
                .neighbor_cache(iface::NeighborCache::new(BTreeMap::new()))
                .finalize(),
        })
    }

    pub fn add_socket<T>(&mut self, socket: T) -> iface::SocketHandle
    where
        T: socket::AnySocket<'static>,
    {
        self.inner.add_socket(socket)
    }

    pub fn get_socket<T>(&mut self, handler: iface::SocketHandle) -> &mut T
    where
        T: socket::AnySocket<'static>,
    {
        self.inner.get_socket(handler)
    }

    pub fn get_socket_and_context<T>(
        &mut self,
        handler: iface::SocketHandle,
    ) -> (&mut T, &mut iface::Context<'static>)
    where
        T: socket::AnySocket<'static>,
    {
        self.inner.get_socket_and_context(handler)
    }

    pub fn remove_socket(&mut self, handler: iface::SocketHandle) -> socket::Socket<'static> {
        self.inner.remove_socket(handler)
    }
}
