use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use simple_expand_tilde::expand_tilde;
use std::path::PathBuf;

mod commands;
mod config;
mod error;

use crate::config::load_config_from_yaml;
use crate::error::Error;

const CMD_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Parser)]
#[command(
   name = CMD_NAME,
   version = env!("CARGO_PKG_VERSION"),
   about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Cli {
   #[command(subcommand)]
   command: Commands,
   #[arg(
      long,
      short,
      global = true,
      help = "Path to the dotfiles directory",
      env = "DOTFILES_DIR",
      default_value = "~/dotfiles"
   )]
   dotfiles_dir: PathBuf,
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
}

#[derive(Debug, Args)]
struct LinksArgs {
   #[command(subcommand)]
   command: LinksCommands,
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

fn get_config_path(dotfiles_dir: &PathBuf) -> Result<PathBuf, Error> {
   let config_path = dotfiles_dir.join(".dotman.yaml");
   expand_tilde(&config_path).ok_or(Error::FailedToExpandTilde(config_path.clone()))
}

fn inner_main() -> Result<(), Error> {
   let args = Cli::parse();

   let dotfiles_dir = args.dotfiles_dir;
   let config_file = std::fs::File::open(get_config_path(&dotfiles_dir)?)
      .map_err(|e| Error::FailedToLoadConfig(e))?;
   let config = load_config_from_yaml(config_file).map_err(|e| Error::FailedToParseConfig(e))?;

   match args.command {
      Commands::Links(links_args) => match links_args.command {
         LinksCommands::Install { force, dry_run } => {
            commands::links::install(&config.mappings, &dotfiles_dir, force, dry_run)?;
         }
         LinksCommands::Remove {} => {
            commands::links::remove(&config.mappings, &dotfiles_dir)?;
         }
         LinksCommands::List {} => {
            commands::links::list(&config.mappings, &dotfiles_dir)?;
         }
      },
      Commands::Update {} => {
         unimplemented!("Update dotfiles");
      }
      Commands::SelfKw(self_args) => match self_args.command {
         SelfCommands::Update {} => {
            unimplemented!("Self update");
         }
      },
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
