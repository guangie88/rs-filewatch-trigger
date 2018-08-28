use std::{
    process::{Command, Output},
    str,
};
use strfmt::strfmt;

use actions::{Action, Result};
use types::PathEvent;

pub struct CmdAction {
    pub cmd: String,
    pub print_stdout: bool,
    pub print_stderr: bool,
}

fn run_raw_command(
    cmd: &str,
    print_stdout: bool,
    print_stderr: bool,
) -> Result<Output> {
    let output = if cfg!(target_os = "windows") {
        Ok(Command::new("cmd").args(&["/C", cmd]).output()?)
    } else {
        Ok(Command::new("sh").args(&["-c", cmd]).output()?)
    };

    if let Ok(ref output) = output {
        if print_stdout {
            print!("{}", str::from_utf8(&output.stdout)?);
        }

        if print_stderr {
            eprint!("{}", str::from_utf8(&output.stderr)?);
        }
    }

    output
}

impl CmdAction {
    pub fn new<S: AsRef<str>>(
        cmd: S,
        print_stdout: bool,
        print_stderr: bool,
    ) -> CmdAction {
        CmdAction {
            cmd: cmd.as_ref().to_owned(),
            print_stdout,
            print_stderr,
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
        run_raw_command(&interpolated_cmd, self.print_stdout, self.print_stderr)
            .map(|_| ())
    }
}
