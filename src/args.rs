use std::path::PathBuf;

bitflags! {
    struct EventType: u8 {
        const NONE = 0b00000000;
        const CREATED = 0b00000001;
        const DELETED = 0b00000010;
        const MODIFIED = 0b00000100;
        const MOVED = 0b00001000;
        const ALL = Self::CREATED.bits | Self::DELETED.bits | Self::MODIFIED.bits | Self::MOVED.bits;
    }
}

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
    #[structopt(short = "f", long = "filter", default_value = "*.*")]
    pub filter: String,

    /// Event type to trigger on (0=NONE, 1=CREATED, 2=DELETED, 4=MODIFIED, 8=MOVED)
    #[structopt(short = "e", long = "event", default_value = "1")]
    pub event: u8,

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
        #[structopt(short = "c", long = "cmd")]
        cmd: String,
    },
}
