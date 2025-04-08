use clap::{Args, Parser, Subcommand};

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
   Install {},
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

fn main() {
   let args = Cli::parse();

   match args.command {
      Commands::Links(links_args) => match links_args.command {
         LinksCommands::Install {} => {
            unimplemented!("Install symlinks");
         }
         LinksCommands::Remove {} => {
            unimplemented!("Remove symlinks");
         }
         LinksCommands::List {} => {
            unimplemented!("List symlinks");
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
}
