#[macro_use]
extern crate bitflags;
extern crate failure;
extern crate notify;
extern crate strfmt;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate vlog;

mod args;

use args::{ActionConf, ArgConf};
use failure::Error;
use notify::{watcher, PollWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;
use strfmt::strfmt;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let config = ArgConf::from_args();
    vlog::set_verbosity_level(usize::from(config.verbose));

    let (tx, rx) = channel();
    let delay = Duration::from_secs(1);

    let mut watcher: Box<Watcher> = if !config.force_poll {
        Box::new(watcher(tx, delay)?)
    } else {
        Box::new(PollWatcher::new(tx, delay)?)
    };

    v0!("Filewatch Trigger has started, CTRL-C to terminate...");

    match config.action {
        ActionConf::Cmd { cmd, .. } => {
            // let interpolated_cmd = strfmt(&cmd, )
            v1!("Running shell template command: ")
        }
    }

    Ok(())
}
