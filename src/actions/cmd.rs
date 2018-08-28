use std::process::{Command, Output};
use strfmt::strfmt;

use actions::{Action, Result};
use types::PathEvent;

pub struct CmdAction {
    pub cmd: String,
}

fn run_raw_command(cmd: &str) -> Result<Output> {
    if cfg!(target_os = "windows") {
        Ok(Command::new("cmd").args(&["/C", cmd]).output()?)
    } else {
        Ok(Command::new("sh").args(&["-c", cmd]).output()?)
    }
}

impl CmdAction {
    pub fn new<S: AsRef<str>>(cmd: S) -> CmdAction {
        CmdAction {
            cmd: cmd.as_ref().to_owned(),
        }
    }
}

impl Action for CmdAction {
    fn invoke(&self, path_event: &PathEvent) -> Result<()> {
        let path = &path_event.path;
        let event = &path_event.event;

        let mapping = hashmap! {
            "path".to_owned() => path.to_str()
                .map(|s| s.to_owned())
                .ok_or_else(
                    || format_err!("Unable to format {:?} as str", path))?,
            "event".to_owned() => format!("{}", event),
        };

        let interpolated_cmd = strfmt(&self.cmd, &mapping)?;
        run_raw_command(&interpolated_cmd).map(|_| ())
    }
}
