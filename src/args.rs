use std::path::PathBuf;

use types::{EventType, GlobMatchers};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "FileWatch Trigger Rust Application", about = "Configuration for FWT"
)]
pub struct ArgConf {
    /// Action to trigger
    #[structopt(subcommand)]
    pub action: ActionConf,

    /// Directory path to watch recursive
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    pub path: PathBuf,

    /// Glob pattern(s) for file matching (comma delimited)
    #[structopt(short = "f", long = "filters", default_value = "*.*")]
    pub filters: GlobMatchers,

    /// Event type to trigger on (0=NONE, 1=CREATED, 2=DELETED, 4=MODIFIED, 8=MOVED)
    #[structopt(short = "e", long = "event", default_value = "1")]
    pub event: EventType,

    /// Delay interval in milliseconds between each file watch detection
    #[structopt(short = "d", long = "delay", default_value = "1000")]
    pub delay_ms: u64,

    /// Force using polling implementation, works for any platform
    #[structopt(long = "force-poll")]
    pub force_poll: bool,

    /// Use relative path instead of absolute path for path matches
    #[structopt(long = "relative")]
    pub relative: bool,

    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "action")]
pub enum ActionConf {
    #[structopt(name = "cmd")]
    Cmd {
        /// Shell template command to run with string interpolation
        /// ({path}: triggered file path)
        /// ({event}: event type number)
        #[structopt()]
        cmd: String,

        /// Prints shell stdout to main stdout
        #[structopt(long = "print-stdout")]
        print_stdout: bool,

        /// Prints shell stderr to main stderr
        #[structopt(long = "print-stderr")]
        print_stderr: bool,
    },
}
