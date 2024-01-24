mod cli;
use clap::Parser;
use cli::{Cli, RunArgs, BuildArgs};
use envconfig::Envconfig;

use vaca_repl::{Repl, Settings};
use vaca_vm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match std::env::var("VACA_HOME") {
        Ok(_) => {},
        Err(_) => {
            let default = if cfg!(windows) {
                std::env::var("TMP").unwrap()
            } else {
                String::from("/tmp")
            };
            
            let vaca_home = format!("{}/.vaca", homedir::get_my_home()
                .unwrap_or(Some((&default).into()))
                .unwrap_or(default.into())
                .as_path()
                .to_string_lossy());
            std::env::set_var("VACA_HOME", &vaca_home);
        },
    };

    let res = match cli.command {
        Some(cmd) => match cmd {
            cli::Commands::Repl => repl(clap::crate_version!()),
            cli::Commands::Run(RunArgs { file: filename }) => vaca_vm::run(filename),
            cli::Commands::Build(BuildArgs { input, output}) => vaca_build::build(input, output)    
        },
        None => repl(clap::crate_version!())
    };

    match res {
        Err(e) => {eprintln!("{}", e); Err(e)},
        ok => ok
    }
}

fn repl(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = Settings::init_from_env()?;
    settings.version = version.to_string();

    let repl = Repl::new(settings).load_stl();

    repl.repl()
}

#[cfg(test)]
mod tests {
    // TODO fix SYmbolTable failure
}