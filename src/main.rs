#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
extern crate globset;
#[macro_use]
extern crate maplit;
extern crate notify;
extern crate strfmt;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate vlog;

use actions::base::{Action, Result};
use notify::{watcher, DebouncedEvent::*, PollWatcher, RecursiveMode, Watcher};
use std::{path::Path, sync::mpsc::channel, time::Duration};
use structopt::StructOpt;

mod actions;
mod args;
mod types;
mod watchers;

use actions::CmdAction;
use args::{ActionConf, ArgConf};
use types::{EventType, GlobMatcher, PathEvent};
use watchers::WeakWatcher;

fn select_path_event<P: AsRef<Path>>(
    filters: &[GlobMatcher],
    path: P,
    conf_event: EventType,
    target_event: EventType,
) -> Option<PathEvent> {
    if conf_event & target_event != EventType::NONE
        && filters.iter().any(|f| f.is_match(path.as_ref()))
    {
        Some(PathEvent::new(path, target_event))
    } else {
        None
    }
}

fn main() -> Result<()> {
    let config = ArgConf::from_args();
    vlog::set_verbosity_level(usize::from(config.verbose));
    v3!("Config: {:#?}", config);

    let (tx, rx) = channel();
    let delay = Duration::from_millis(config.delay_ms);

    let mut watcher: Box<WeakWatcher> = if !config.force_poll {
        Box::new(watcher(tx, delay)?)
    } else {
        Box::new(PollWatcher::new(tx, delay)?)
    };

    watcher.watch(&config.path, RecursiveMode::Recursive)?;
    v1!("Filewatch Trigger has started, CTRL-C to terminate...");

    loop {
        match rx.recv() {
            Ok(event) => {
                let event_opt = match &event {
                    Create(path) => select_path_event(
                        &config.filter,
                        path,
                        config.event,
                        EventType::CREATED,
                    ),
                    Remove(path) => select_path_event(
                        &config.filter,
                        path,
                        config.event,
                        EventType::DELETED,
                    ),
                    Write(path) => select_path_event(
                        &config.filter,
                        path,
                        config.event,
                        EventType::MODIFIED,
                    ),
                    Rename(old_path, _) => select_path_event(
                        &config.filter,
                        old_path,
                        config.event,
                        EventType::MOVED,
                    ),
                    _ => None,
                };

                if let Some(path_event) = event_opt {
                    let action_res = match &config.action {
                        ActionConf::Cmd { cmd, .. } => {
                            let cmd_action = CmdAction::new(cmd);
                            cmd_action.invoke(&path_event)
                        }
                    };

                    match action_res {
                        Ok(()) => {
                            v2!("Invoked action on path: {:?}", path_event.path)
                        }
                        Err(e) => ve0!("Action error: {}", e),
                    }
                }
            }
            Err(e) => ve0!("Watch error: {:?}", e),
        }
    }
}
