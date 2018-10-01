#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
extern crate globset;
#[macro_use]
extern crate maplit;
extern crate notify;
extern crate path_absolutize;
extern crate signal_hook;
extern crate strfmt;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate vlog;

use notify::{
    watcher,
    DebouncedEvent::{Create, Error, Remove, Rename, Write},
    PollWatcher, RecursiveMode, Watcher,
};
use path_absolutize::Absolutize;
use std::{
    env,
    path::Path,
    sync::{mpsc::channel, Mutex},
    time::Duration,
};
use structopt::StructOpt;

mod actions;
mod args;
mod types;
mod watchers;

use actions::{
    base::{Action, Result},
    CmdAction,
};
use args::{ActionConf, ArgConf};
use types::{EventType, PathEvent};
use watchers::WeakWatcher;

fn main() -> Result<()> {
    // config set-up
    let config = ArgConf::from_args();

    vlog::set_verbosity_level(usize::from(config.verbose));
    v3!("Config: {:#?}", config);

    // watcher set-up
    let (tx, rx) = channel();
    let signal_tx = Mutex::from(tx.clone());

    // for handling of signals
    unsafe {
        signal_hook::register(signal_hook::SIGINT, move || {
            v1!("Detected SIGINT for termination...");
            
            match signal_tx.try_lock() {
                Ok(tx) => {
                    if let Err(e) = tx.send(Error(
                        notify::Error::Generic("SIGNALLED".to_owned()),
                        None,
                    )) {
                        ve1!("Tx error: {}", e);
                    }
                }
                Err(e) => ve1!("Lock error: {}", e),
            };
        })?;
    }

    let delay = Duration::from_millis(config.delay_ms);

    let mut watcher: Box<WeakWatcher> = if !config.force_poll {
        Box::new(watcher(tx, delay)?)
    } else {
        Box::new(PollWatcher::new(tx, delay)?)
    };

    watcher.watch(&config.path, RecursiveMode::Recursive)?;
    v1!("Filewatch Trigger has started, CTRL-C to terminate...");

    let select_path_event =
        |path: &Path, target_event| -> Result<Option<PathEvent>> {
            if config.event & target_event != EventType::NONE
                && config.filters.iter().any(|filter| filter.is_match(path))
            {
                let triggered_path = if config.relative {
                    path.strip_prefix(env::current_dir()?)?.to_owned()
                } else {
                    path.absolutize()?
                };

                Ok(Some(PathEvent::new(triggered_path, target_event)))
            } else {
                Ok(None)
            }
        };

    let loop_recv = || -> Result<Option<PathEvent>> {
        let event = rx.recv()?;

        let path_event = match &event {
            Create(path) => select_path_event(path, EventType::CREATED),
            Remove(path) => select_path_event(path, EventType::DELETED),
            Write(path) => select_path_event(path, EventType::MODIFIED),
            Rename(old_path, _) => {
                select_path_event(old_path, EventType::MOVED)
            }
            Error(_, None) => Ok(None),
            Error(ref e, _) => {
                ve1!("Path event error: {}", e);
                Ok(None)
            },
            _ => Ok(None),
        }?;

        if let Some(path_event) = path_event {
            match &config.action {
                ActionConf::Cmd {
                    cmd,
                    print_stdout,
                    print_stderr,
                    ..
                } => {
                    let cmd_action =
                        CmdAction::new(cmd, *print_stdout, *print_stderr);
                    cmd_action.invoke(&path_event)
                }
            }?;

            Ok(Some(path_event))
        } else {
            Ok(None)
        }
    };

    loop {
        match loop_recv() {
            Ok(Some(path_event)) => {
                v2!("Invoked action on path: {:?}", path_event.path)
            }
            Err(e) => ve0!("Error: {:?}", e),
            _ => (),
        }
    }
}
