# FileWatch Trigger

FileWatch Trigger application in Rust. The binary is named `fwt`.

The application watches for any type of file changes in the recursively watched
directory, and triggers the given action.

Currently able to perform the following action(s):

- Run shell template command with path and event

## How to install

You may also choose to visit
[releases](https://github.com/guangie88/rs-filewatch-trigger/releases)
and download the latest version of statically built binary in the zip asset.

Alternatively, if you have the `cargo` command, simply run

```bash
cargo install rs-filewatch-trigger
```

## Example Command

For help:

```bash
fwt -h
```

The following contains the help message:

```bash
USAGE:
    fwt [FLAGS] [OPTIONS] <path> <SUBCOMMAND>

FLAGS:
        --force-poll    Force using polling implementation, works for any platform
    -h, --help          Prints help information
        --relative      Use relative path instead of absolute path for path matches
    -V, --version       Prints version information
    -v, --verbose       Verbose mode (-v, -vv, -vvv)

OPTIONS:
    -d, --delay <delay_ms>     Delay interval in milliseconds between each file watch detection [default: 1000]
    -e, --event <event>        Event type to trigger on (0=NONE, 1=CREATED, 2=DELETED, 4=MODIFIED, 8=MOVED) [default: 1]
    -f, --filters <filters>    Glob pattern(s) for file matching (comma delimited) [default: *.*]

ARGS:
    <path>    Directory path to watch recursive

SUBCOMMANDS:
    cmd     FileWatch Trigger to run various actions
    help    Prints this message or the help of the given subcommand(s)
```

### Shell template command action

Assuming the current working directory is `/home/xxx`:

```bash
fwt -e 15 \
    -f "*.jpg,*.png" \
    -v \
    . \
    cmd --print-stdout "echo {path}: {event}"
```

Example output:

```bash
/home/xxx/a.jpg: CREATED
```

The following contains the help message for this action:

```bash
USAGE:
    fwt cmd [FLAGS] <cmd>

FLAGS:
    -h, --help            Prints help information
        --print-stderr    Prints shell stderr to main stderr
        --print-stdout    Prints shell stdout to main stdout
    -V, --version         Prints version information

ARGS:
    <cmd>    Shell template command to run with string interpolation ({path}: triggered file path) ({event}: event
             type number)
```
