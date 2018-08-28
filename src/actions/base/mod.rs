use failure::Error;

use types::PathEvent;

pub type Result<T> = ::std::result::Result<T, Error>;

pub trait Action {
    fn invoke(&self, path_event: &PathEvent) -> Result<()>;
}
