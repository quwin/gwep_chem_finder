use clap::Parser;
use cli::cli::start_cli;
use data::chemicals::*;
use data::fetch::update;
use data::initialize::initialize_compound_tree;
use data::local::data_exists;
extern crate pest;
extern crate pest_derive;

/// Gwep Chem Finder
#[derive(clap::Parser)]
#[command()]
struct Args {
    ///Forces the program to update
    #[arg(short, long)]
    update: bool,
    ///Runs the program in CLI mode
    #[arg(short, long)]
    cli: bool,
}

fn main() {
    println!("Welcome to gwep chem finder!");
    println!("Available Bases: {:?}", BASES);

    let args = Args::parse();

    let update_result = update();

    let updated;
    let paths = match update_result {
        (s, b) => {
            updated = b;
            Some(s)
        }
    };
    let data_string = "data/data.json".to_string();
   
    if updated || !data_exists(&data_string) || args.update {
        initialize_compound_tree(paths);
    } else {
        initialize_compound_tree(None);
    }

    // Command Line Interface for looking up Compounds
    if args.cli {
        start_cli();
    }
}
