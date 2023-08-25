use std::{ffi::OsStr, path::PathBuf, str::FromStr};

use argp::{FromArgValue, FromArgs};

pub mod analysis;
pub mod argp_version;
pub mod cmd;
pub mod obj;
pub mod util;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl FromStr for LogLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "error" => Self::Error,
            "warn" => Self::Warn,
            "info" => Self::Info,
            "debug" => Self::Debug,
            "trace" => Self::Trace,
            _ => return Err(()),
        })
    }
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        }
        .to_string()
    }
}

impl FromArgValue for LogLevel {
    fn from_arg_value(value: &OsStr) -> Result<Self, String> {
        String::from_arg_value(value)
            .and_then(|s| Self::from_str(&s).map_err(|_| "Invalid log level".to_string()))
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Yet another GameCube/Wii decompilation toolkit.
struct TopLevel {
    #[argp(subcommand)]
    command: SubCommand,
    #[argp(option, short = 'C')]
    /// Change working directory.
    chdir: Option<PathBuf>,
    #[argp(option, short = 'L', default = "LogLevel::Info")]
    /// Minimum logging level. (Default: info)
    /// Possible values: error, warn, info, debug, trace
    log_level: LogLevel,
    /// Print version information and exit.
    #[argp(switch, short = 'V')]
    version: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argp(subcommand)]
enum SubCommand {
    Ar(cmd::ar::Args),
    Demangle(cmd::demangle::Args),
    Dol(cmd::dol::Args),
    Dwarf(cmd::dwarf::Args),
    Elf(cmd::elf::Args),
    Elf2Dol(cmd::elf2dol::Args),
    // Map(cmd::map::Args),
    MetroidBuildInfo(cmd::metroidbuildinfo::Args),
    Rel(cmd::rel::Args),
    Rso(cmd::rso::Args),
    Shasum(cmd::shasum::Args),
}

fn main() {
    let format = tracing_subscriber::fmt::format().with_target(false).without_time();
    tracing_subscriber::fmt().event_format(format).init();
    // TODO reimplement log level selection

    let args: TopLevel = argp_version::from_env();
    let mut result = Ok(());
    if let Some(dir) = &args.chdir {
        result = std::env::set_current_dir(dir).map_err(|e| {
            anyhow::Error::new(e)
                .context(format!("Failed to change working directory to '{}'", dir.display()))
        });
    }
    result = result.and_then(|_| match args.command {
        SubCommand::Ar(c_args) => cmd::ar::run(c_args),
        SubCommand::Demangle(c_args) => cmd::demangle::run(c_args),
        SubCommand::Dol(c_args) => cmd::dol::run(c_args),
        SubCommand::Dwarf(c_args) => cmd::dwarf::run(c_args),
        SubCommand::Elf(c_args) => cmd::elf::run(c_args),
        SubCommand::Elf2Dol(c_args) => cmd::elf2dol::run(c_args),
        // SubCommand::Map(c_args) => cmd::map::run(c_args),
        SubCommand::MetroidBuildInfo(c_args) => cmd::metroidbuildinfo::run(c_args),
        SubCommand::Rel(c_args) => cmd::rel::run(c_args),
        SubCommand::Rso(c_args) => cmd::rso::run(c_args),
        SubCommand::Shasum(c_args) => cmd::shasum::run(c_args),
    });
    if let Err(e) = result {
        eprintln!("Failed: {e:?}");
        std::process::exit(1);
    }
}
