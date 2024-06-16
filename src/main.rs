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
    FareAttributes,
    FareRules,
    Timeframes,
    FareMedia,
    FareProducts,
}

fn main() {
    // TODO add user input instead of hard coding.
    let args = Cli::parse();

    match args.command {
        Commands::Echo(args) => {
            let gtfs_path = args.input;
            let mut gtfs_file = functions::load_gtfs_file(gtfs_path);
            
            match args.file {
                StandardFiles::Files => {
                    functions::file_list(&mut gtfs_file)
                },
                StandardFiles::Agencies => {
                    functions::agency_out(&mut gtfs_file)
                },
                StandardFiles::Stops => {
                    functions::stops_out(&mut gtfs_file)
                },
                StandardFiles::Routes => {
                    functions::routes_out(&mut gtfs_file)
                },
                StandardFiles::Trips => {
                    functions::trips_out(&mut gtfs_file)
                },
                StandardFiles::StopTimes => {
                    functions::stop_times_out(&mut gtfs_file)
                },
                StandardFiles::Calendar => {
                    functions::calendar_out(&mut gtfs_file)
                },
                StandardFiles::CalendarDates => {
                    functions::calendar_dates_out(&mut gtfs_file)
                },
                StandardFiles::FareAttributes => {
                    functions::fare_attributes_out(&mut gtfs_file)
                },
                StandardFiles::FareRules => {
                    functions::fare_rules_out(&mut gtfs_file)
                },
                StandardFiles::Timeframes => {
                    functions::timeframes_out(&mut gtfs_file)
                },
                StandardFiles::FareMedia => {
                    functions::fare_media_out(&mut gtfs_file)
                }
                StandardFiles::FareProducts => {
                    functions::fare_products_out(&mut gtfs_file)
                }
            } 
        },
        Commands::GeoJson(args) => {
            let gtfs_path = args.input;
            let mut gtfs_file = functions::load_gtfs_file(gtfs_path);

            match args.file {
                StandardFiles::Stops => {
                    let json = functions::simple_stops_json(&mut gtfs_file);
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