use failure::Error;
use std::{
    fmt::{self, Display},
    str::FromStr,
};

bitflags! {
    pub struct EventType: u8 {
        const NONE = 0b0000_0000;
        const CREATED = 0b0000_0001;
        const DELETED = 0b0000_0010;
        const MODIFIED = 0b0000_0100;
        const MOVED = 0b0000_1000;
        const ALL =
            Self::CREATED.bits |
            Self::DELETED.bits |
            Self::MODIFIED.bits |
            Self::MOVED.bits;
    }
}

impl FromStr for EventType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
