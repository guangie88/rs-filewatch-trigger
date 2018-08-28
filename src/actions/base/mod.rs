use failure::Error;
use std::{
    fmt::{self, Display},
    path::Path,
    str::FromStr,
};

bitflags! {
    pub struct EventType: u8 {
        const NONE = 0b00000000;
        const CREATED = 0b00000001;
        const DELETED = 0b00000010;
        const MODIFIED = 0b00000100;
        const MOVED = 0b00001000;
        const ALL = Self::CREATED.bits | Self::DELETED.bits | Self::MODIFIED.bits | Self::MOVED.bits;
    }
}

impl FromStr for EventType {
    type Err = Error;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let val = s.parse::<u8>()?;
        EventType::from_bits(val)
            .ok_or_else(|| format_err!("Cannot parse {} as EventType", val))
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub trait Action {
    fn invoke(&self, path: &Path, event: EventType) -> Result<()>;
}
