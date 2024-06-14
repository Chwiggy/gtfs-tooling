mod functions;

use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Echo(EchoArgs),
}

#[derive(Args)]
struct EchoArgs {
    input: std::path::PathBuf,
    
    #[command(subcommand)]
    file: StandardFiles,
}

#[derive(Subcommand)]
enum StandardFiles {
    Files,
    CalendarDates,
    Stops,
}

fn main() {
    // TODO add user input instead of hard coding.
    let args = Cli::parse();

    match args.command {
        Commands::Echo(args) => {
            let gtfs_path = args.input;
            match args.file {
                StandardFiles::Files => {
                    functions::file_list(gtfs_path)
                },
                StandardFiles::CalendarDates => {
                    functions::calendar_dates_out(gtfs_path)
                },
                StandardFiles::Stops => {
                    panic!()
                },
            } 
        },
    }
    
}


// TODO Reading in GTFS File
// TODO option: clipping by bounds
// TODO option: filtering by various categories
// TODO option: clipping by time

// TODO think about visualisations