use nic_buffers::{ReceivedFrame, TransmitBuffer};
use smoltcp::phy;

pub use phy::DeviceCapabilities;

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

/// Wrapper around a network device.
///
/// We use this because we can't directly implement [`phy::Device`] for `T` due
/// to the following error:
/// ```
/// error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
/// ```
#[doc(hidden)]
pub struct DeviceWrapper<T>(pub(crate) &'static mut T)
where
    T: 'static + Device + ?Sized;

impl<'a, T> phy::Device<'a> for DeviceWrapper<T>
where
    T: 'static + Device + ?Sized,
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
        if self.0 .0.len() > 1 {
            log::error!("RxToken::consume(): trying to consume frame with multiple buffers");
        }
        f(self.0 .0[0].as_slice_mut())
    }
}

#[doc(hidden)]
pub struct TxToken<'a, T>(&'a mut T)
where
    T: 'a + Device + ?Sized;

impl<'a, T> phy::TxToken for TxToken<'a, T>
where
    T: 'a + Device + ?Sized,
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
