#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate maplit;
extern crate notify;
extern crate strfmt;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate vlog;

use failure::Error;
use notify::{watcher, DebouncedEvent::*, PollWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use structopt::StructOpt;

mod actions;
mod args;
mod watchers;

use args::{ActionConf, ArgConf};
use watchers::WeakWatcher;

fn main() -> Result<(), Error> {
    let config = ArgConf::from_args();
    vlog::set_verbosity_level(usize::from(config.verbose));

    let (tx, rx) = channel();
    let delay = Duration::from_secs(1);

    let mut watcher: Box<WeakWatcher> = if !config.force_poll {
        Box::new(watcher(tx, delay)?)
    } else {
        Box::new(PollWatcher::new(tx, delay)?)
    };

    watcher.watch(&config.path, RecursiveMode::Recursive)?;
    v0!("Filewatch Trigger has started, CTRL-C to terminate...");

    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    Create(path) => unimplemented!(),
                    Remove(path) => unimplemented!(),
                    Write(path) => unimplemented!(),
                    Rename(old_path, new_path) => unimplemented!(),
                    _ => unimplemented!(),
                }

                // match &config.action {
                //     ActionConf::Cmd { cmd, .. } => {
                //         // let interpolated_cmd = strfmt(&cmd, )
                //         v1!("Running shell template command: ")
                //     }
                // }

                v0!("{:?}", event);
            }
            Err(e) => ve0!("Watch error: {:?}", e),
        }
    }
}
