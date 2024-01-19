use clap::{Args, Subcommand};
use envconfig::Envconfig;

/// Vaca Lang repl, interpreter and compiler, all-in-one
#[derive(Debug, clap::Parser)]
#[command(name = "Vaca")]
#[command(author = "OJarrisonn <j.h.m.t.v.10@gmail.com>")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// The command to be run
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Starts a Vaca repl
    Repl,
    /// Runs a .vaca or .leite file
    Run(RunArgs),
    /// Builds a .vaca file an generates a .leite binary file
    Build(BuildArgs)
}

#[derive(Args, Debug)]
pub struct RunArgs {
    pub file: String
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// The input .vaca file to be built
    pub input: String,
    /// Optional .leite filename to store the build result
    pub output: Option<String>
}

#[derive(Envconfig, Debug)]
pub struct Settings {
    #[envconfig(from = "VACA_HOME")]
    pub vaca_home: String,
    #[envconfig(from = "VACA_REPL_HIST_LEN", default = "100")]
    pub repl_history_len: usize,
}

// TODO Repl Helper