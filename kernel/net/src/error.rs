pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    Exhausted,
    Illegal,
    Unaddressable,
    Finished,
    Truncated,
    Checksum,
    Unrecognized,
    Fragmented,
    Malformed,
    Dropped,
    NotSupported,
    InvalidGateway,
    Unknown,
}

impl From<smoltcp::Error> for Error {
    fn from(e: smoltcp::Error) -> Self {
        match e {
            smoltcp::Error::Exhausted => Self::Exhausted,
            smoltcp::Error::Illegal => Self::Illegal,
            smoltcp::Error::Unaddressable => Self::Unaddressable,
            smoltcp::Error::Finished => Self::Finished,
            smoltcp::Error::Truncated => Self::Truncated,
            smoltcp::Error::Checksum => Self::Checksum,
            smoltcp::Error::Unrecognized => Self::Unrecognized,
            smoltcp::Error::Fragmented => Self::Fragmented,
            smoltcp::Error::Malformed => Self::Malformed,
            smoltcp::Error::Dropped => Self::Dropped,
            smoltcp::Error::NotSupported => Self::NotSupported,
            _ => Self::Unknown,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(
            match self {
                Error::Exhausted => todo!(),
                Error::Illegal => todo!(),
                Error::Unaddressable => todo!(),
                Error::Finished => todo!(),
                Error::Truncated => todo!(),
                Error::Checksum => todo!(),
                Error::Unrecognized => todo!(),
                Error::Fragmented => todo!(),
                Error::Malformed => todo!(),
                Error::Dropped => todo!(),
                Error::NotSupported => todo!(),
                Error::InvalidGateway => todo!(),
                Error::Unknown => todo!(),
            }
        )
    }
}
