use clap::{Args, CommandFactory as _, Parser, Subcommand, ValueHint};
use clap_complete::{Shell, generate};
use colored::Colorize;
use shadow_rs::{formatcp, shadow};
use simple_expand_tilde::expand_tilde;
use std::path::PathBuf;

mod commands;
mod config;
mod error;

use crate::config::load_config_from_yaml;
use crate::error::Error;

shadow!(build);

const CMD_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = formatcp!(
   r#"{}
Commit: {} (branch: {})
Build: {}, {}"#,
   env!("CARGO_PKG_VERSION"),
   build::COMMIT_HASH,
   build::BRANCH,
   build::RUST_VERSION,
   build::RUST_CHANNEL,
);

#[derive(Debug, Parser)]
#[command(
   name = CMD_NAME,
   version = VERSION,
   about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Cli {
   #[command(subcommand)]
   command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
   #[command(about = "Manage symlinks for dotfiles")]
   Links(LinksArgs),
   #[command(
      about = "Update dotfiles repository (alias for `git -C <dotfiles directory> pull origin master`)"
   )]
   Update {},
   #[command(name = "self", about = format!("Modify the {CMD_NAME} installation"))]
   SelfKw(SelfArgs), // Avoids using `Self` as a name, it's a reserved keyword.
   #[command(about = "Generate shell completion scripts")]
   Completion {
      #[clap(long, short)]
      shell: Shell,
   },
}

#[derive(Debug, Args)]
struct LinksArgs {
   #[command(subcommand)]
   command: LinksCommands,
   #[command(flatten)]
   dotfiles: Dotfiles,
}

#[derive(Debug, Subcommand)]
enum LinksCommands {
   #[command(about = "Install symlinks")]
   Install {
      #[arg(
         long,
         short,
         help = "Force install symlinks, overwriting existing files without confirmation"
      )]
      force: bool,
      #[arg(
         long,
         help = "Dry run, show symlinks that would be installed without actually installing them"
      )]
      dry_run: bool,
   },
   #[command(about = "Remove symlinks")]
   Remove {},
   #[command(about = "List installed symlinks")]
   List {},
}

#[derive(Debug, Args)]
struct SelfArgs {
   #[command(subcommand)]
   command: SelfCommands,
}

#[derive(Debug, Subcommand)]
enum SelfCommands {
   #[command(about = "Download and install the latest version")]
   Update {},
}

#[derive(Debug, Args)]
struct Dotfiles {
   #[arg(
      long = "dotfiles-dir",
      short,
      global = true,
      name = "DOTFILES_DIR",
      help = "Path to the dotfiles directory",
      env = "DOTFILES_DIR",
      default_value = "~/dotfiles",
      value_hint = ValueHint::DirPath,
   )]
   dir: PathBuf,
}

fn get_config_path(dotfiles_dir: &PathBuf) -> Result<PathBuf, Error> {
   let config_path = dotfiles_dir.join(".dotman.yaml");
   expand_tilde(&config_path).ok_or(Error::FailedToExpandTilde(config_path.clone()))
}

fn inner_main() -> Result<(), Error> {
   let args = Cli::parse();

   match args.command {
      Commands::Links(links_args) => {
         let dotfiles_dir = links_args.dotfiles.dir;
         let config_file = std::fs::File::open(get_config_path(&dotfiles_dir)?)
            .map_err(|e| Error::FailedToLoadConfig(e))?;
         let config =
            load_config_from_yaml(config_file).map_err(|e| Error::FailedToParseConfig(e))?;

         match links_args.command {
            LinksCommands::Install { force, dry_run } => {
               commands::links::install(&config.mappings, &dotfiles_dir, force, dry_run)?;
            }
            LinksCommands::Remove {} => {
               commands::links::remove(&config.mappings, &dotfiles_dir)?;
            }
            LinksCommands::List {} => {
               commands::links::list(&config.mappings, &dotfiles_dir)?;
            }
         }
      }
      Commands::Update {} => {
         unimplemented!("Update dotfiles");
      }
      Commands::SelfKw(self_args) => match self_args.command {
         SelfCommands::Update {} => {
            unimplemented!("Self update");
         }
      },
      Commands::Completion { shell } => {
         generate(shell, &mut Cli::command(), CMD_NAME, &mut std::io::stdout());
      }
   }
   Ok(())
}

fn main() {
   match inner_main() {
      Ok(_) => {}
      Err(e) => {
         eprintln!("{} {}", "error:".bright_red(), e);
         std::process::exit(1);
      }
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn verify_cli() {
      Cli::command().debug_assert()
   }
}
