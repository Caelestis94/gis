use std::path::PathBuf;

use anyhow::{anyhow, Context, Error, Ok, Result};

use crate::opts::Opts;

/// Config struct that holds the parsed arguments
#[derive(Debug)]
pub struct Config {
    pub command: Command,
    pub config: PathBuf,
    pub pwd: PathBuf,
}

/// Operation enum that holds the parsed subcommands
#[derive(Debug, PartialEq)]
pub enum Operation {
    List,
    Add(String),
    Remove(usize),
}
/// Command enum that holds the parsed commands
#[derive(Debug, PartialEq)]
pub enum Command {
    Identity(Operation),
    Workspace(Operation),
    Swap(usize),
    Current,
    Check,
}

impl TryFrom<Opts> for Config {
    type Error = Error;

    fn try_from(value: Opts) -> Result<Self> {
        let args = value.command;
        let config = get_config(value.config)?;
        let command = Command::try_from(args)?;
        let pwd = get_pwd(value.pwd)?;

        Ok(Config {
            pwd,
            command,
            config,
        })
    }
}

impl TryFrom<Vec<String>> for Command {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut args = value;
        if args.len() == 0 {
            return Err(anyhow!("No arguments provided, see --help"));
        }
        let cmd = args.get(0).expect("Impossible");
        match cmd.as_str() {
            "identity" => {
                args.remove(0);
                let op = Operation::try_from(args)?;
                Ok(Command::Identity(op))
            }
            "workspace" => {
                args.remove(0);
                let op = Operation::try_from(args)?;
                Ok(Command::Workspace(op))
            }
            "swap" =>{
                args.remove(0);
                if args.len() == 0 {
                    return Err(anyhow!("No index provided."));
                }
                let num = args.remove(0);
                let num = num.parse::<usize>().context("Invalid number")?;
                Ok(Command::Swap(num))
            },
            "current" => Ok(Command::Current),
            "check" => Ok(Command::Check),
            _ => Err(anyhow!("Unknown command {}", cmd)),

        }
    }
}

impl TryFrom<Vec<String>> for Operation {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut args = value;
        if args.len() == 0 {
            return Err(anyhow!("The following subcommands are available:\n\nlist : list all items\nadd : add an item\nremove : remove an item\n"));
        }
        let cmd = args.get(0).expect("Impossible");
        match cmd.as_str() {
            "list" => {
                args.remove(0);
                Ok(Operation::List)
            }
            "add" => {
                args.remove(0);
                if args.len() == 0 {
                    return Err(anyhow!("No value provided."));
                }
                let name = args.remove(0);
                Ok(Operation::Add(name))
            }
            "remove" => {
                args.remove(0);
                if args.len() == 0 {
                    return Err(anyhow!("No index provided."));
                }
                let num = args.remove(0);
                let num = num.parse::<usize>().context("Could not parse number")?;
                Ok(Operation::Remove(num))
            }
            _ => Err(anyhow!("Unknown command {}", cmd)),
        }
    }
}

/// Get the config file path from the command line arguments. Defaults to $HOME/.gisrc
fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }
    let loc = std::env::var("HOME").context("No $HOME variable found")?;
    let mut path = PathBuf::from(loc);
    path.push(".gisrc");
    Ok(path)
}

/// Get the current working directory from the command line arguments. Defaults to the current directory.
fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = pwd {
        return Ok(v);
    }
    let loc = std::env::current_dir().context("Could not get current directory")?;
    Ok(loc)
}
