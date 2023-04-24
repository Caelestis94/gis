use anyhow::Result;
use clap::Parser;
use gis::{opts::Opts, config::{Config}};


fn main() -> Result<()> {
    let args :  Config= Opts::parse().try_into()?;
    let mut gis = gis::gis::Gis::from_config(args.config, args.pwd);

    // If we have identity and a workspace, we need to find out if we need to swap
    if gis.has_identity_and_workspace() {
        gis.workspace_identity_swap();
    }

    
    match args.command {
        gis::config::Command::Identity(op) => {
            match op {
                gis::config::Operation::Add(identity) => {
                    gis.add_identity(&identity);
                },
                gis::config::Operation::Remove(idx) => {
                    gis.remove_identity(idx);
                },
                gis::config::Operation::List => {
                    gis.list_identities();
                },
            }
        },
        gis::config::Command::Workspace(op) => {
            match op {
                gis::config::Operation::Add(name) => {
                    gis.add_workspace(&name);
                },
                gis::config::Operation::Remove(idx) => {
                    gis.remove_workspace(idx);
                },
                gis::config::Operation::List => {
                    gis.list_workspaces();
                },
            }

        },
        gis::config::Command::Swap(idx) => {
            // gis.list_identities();
            gis.swap_identity(idx);
        },
        gis::config::Command::Current => gis.current_identity(),
        gis::config::Command::Check => {},
    }

    return Ok(());
}
