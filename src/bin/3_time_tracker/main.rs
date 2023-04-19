mod entries;

use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// Add a new time entry
    Add {/* TODO */},

    /// Show all saved time entries
    Show {/* TODO */},
}

fn main() {
    let args = Args::parse();
    let entries = entries::load_entries().unwrap();

    match &args.command {
        Command::Add { /* TODO */ } => {
            unimplemented!()
        }
        Command::Show { /* TODO */ } => {
            unimplemented!()
        }
    }
}
