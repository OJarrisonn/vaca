use clap::{Args, Subcommand};

/// Vaca Lang repl, interpreter and compiler, all-in-one
#[derive(Debug, clap::Parser)]
#[command(name = "Vaca")]
#[command(author, version, about, long_about = None)]
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
    /// Runs a .vaca or .casco file
    Run(RunArgs),
    /// Builds a .vaca file an generates a .casco binary file
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
    /// Optional .casco filename to store the build result
    pub output: Option<String>
}

// TODO Repl Helper