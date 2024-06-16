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
    GeoJson(GeoJsonArgs)
}

#[derive(Args)]
struct EchoArgs {
    input: std::path::PathBuf,
    
    #[command(subcommand)]
    file: StandardFiles,
}

#[derive(Args)]
struct GeoJsonArgs {
    input: std::path::PathBuf,
    
    #[command(subcommand)]
    file: StandardFiles,

    output: std::path::PathBuf
}


#[derive(Subcommand)]
enum StandardFiles {
    Files,
    Agencies,
    Stops,
    Routes,
    Trips,
    StopTimes,
    Calendar,
    CalendarDates,
    FareAttributes
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
                StandardFiles::Agencies => {
                    functions::agency_out(gtfs_path)
                },
                StandardFiles::Stops => {
                    functions::stops_out(gtfs_path)
                },
                StandardFiles::Routes => {
                    functions::routes_out(gtfs_path)
                },
                StandardFiles::Trips => {
                    functions::trips_out(gtfs_path)
                },
                StandardFiles::StopTimes => {
                    functions::stop_times_out(gtfs_path)
                },
                StandardFiles::Calendar => {
                    functions::calendar_out(gtfs_path)
                },
                StandardFiles::CalendarDates => {
                    functions::calendar_dates_out(gtfs_path)
                },
                StandardFiles::FareAttributes => {
                    functions::fare_attributes_out(gtfs_path)
                },
                
            } 
        },
        Commands::GeoJson(args) => {
            let gtfs_path = args.input;
            match args.file {
                StandardFiles::Stops => {
                    let json = functions::simple_stops_json(gtfs_path);
                    std::fs::write(args.output, json).expect("Unable to write file");
                },
                _ => {
                    println!("Not implemented yet")
                }
            }
        }
    }
    
}


// TODO Reading in GTFS File
// TODO option: clipping by bounds
// TODO option: filtering by various categories
// TODO option: clipping by time

// TODO think about visualisations