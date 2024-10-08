mod functions;
mod objects;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use functions::{gtfs, visualize};
use objects::{route, stop};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Echo(EchoArgs),
    GeoJson(GeoJsonArgs),
    Extract(ExtractArgs),
    Visualize(VisualizeArgs),
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

    output: std::path::PathBuf,
}

#[derive(Args)]
struct ExtractArgs {
    input: std::path::PathBuf,

    #[command(subcommand)]
    file: StandardFiles,

    id: String
}

#[derive(Args)]
struct VisualizeArgs {
    input: std::path::PathBuf,

    #[command(subcommand)]
    file: StandardFiles,

    id: String,
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
    FareLegRules,
    FareTransferRules,
    Areas,
    StopAreas,
    Networks,
    RouteNetworks,
    Shapes,
    Frequencies,
    Transfers,
    Pathways,
    Levels,
    LocationGroups,
    LocationGroupStops,
    BookingRules,
    Translations,
    FeedInfo,
    Attributions,
}

fn main() {
    // TODO add user input instead of hard coding.
    let args = Cli::parse();

    match args.command {
        Commands::Echo(args) => {
            let gtfs_path = args.input;
            let mut gtfs_file = functions::load_gtfs_file(gtfs_path);

            match args.file {
                StandardFiles::Files => functions::file_list(&mut gtfs_file),
                StandardFiles::Agencies => {
                    gtfs_file.to_stdout::<gtfs::Agency>();
                }
                StandardFiles::Stops => gtfs_file.to_stdout::<gtfs::Stop>(),
                StandardFiles::Routes => gtfs_file.to_stdout::<gtfs::Route>(),
                StandardFiles::Trips => gtfs_file.to_stdout::<gtfs::Trip>(),
                StandardFiles::StopTimes => {
                    // Stop Times are a special case due to the broken sample gtfs
                    functions::stop_times_out(&mut gtfs_file)
                }
                StandardFiles::Calendar => gtfs_file.to_stdout::<gtfs::Calendar>(),
                StandardFiles::CalendarDates => gtfs_file.to_stdout::<gtfs::CalendarDate>(),
                StandardFiles::FareAttributes => {
                    // Another special case
                    functions::fare_attributes_out(&mut gtfs_file)
                }
                StandardFiles::FareRules => gtfs_file.to_stdout::<gtfs::FareRule>(),
                StandardFiles::Timeframes => gtfs_file.to_stdout::<gtfs::Timeframe>(),
                StandardFiles::FareMedia => gtfs_file.to_stdout::<gtfs::FareMedium>(),
                StandardFiles::FareProducts => gtfs_file.to_stdout::<gtfs::FareProduct>(),
                StandardFiles::FareLegRules => gtfs_file.to_stdout::<gtfs::FareLegRule>(),
                StandardFiles::FareTransferRules => gtfs_file.to_stdout::<gtfs::FareTransferRule>(),
                StandardFiles::Areas => gtfs_file.to_stdout::<gtfs::Area>(),
                StandardFiles::StopAreas => gtfs_file.to_stdout::<gtfs::StopArea>(),
                StandardFiles::Networks => gtfs_file.to_stdout::<gtfs::Network>(),
                StandardFiles::RouteNetworks => gtfs_file.to_stdout::<gtfs::RouteNetwork>(),
                StandardFiles::Shapes => gtfs_file.to_stdout::<gtfs::Shape>(),
                StandardFiles::Frequencies => gtfs_file.to_stdout::<gtfs::Frequency>(),
                StandardFiles::Transfers => gtfs_file.to_stdout::<gtfs::Transfer>(),
                StandardFiles::Pathways => gtfs_file.to_stdout::<gtfs::Pathway>(),
                StandardFiles::Levels => gtfs_file.to_stdout::<gtfs::Level>(),
                StandardFiles::LocationGroups => gtfs_file.to_stdout::<gtfs::LocationGroup>(),
                StandardFiles::LocationGroupStops => {
                    gtfs_file.to_stdout::<gtfs::LocationGroupStop>()
                }
                StandardFiles::BookingRules => gtfs_file.to_stdout::<gtfs::BookingRule>(),
                StandardFiles::Translations => gtfs_file.to_stdout::<gtfs::Translation>(),
                StandardFiles::FeedInfo => gtfs_file.to_stdout::<gtfs::FeedInfo>(),
                StandardFiles::Attributions => gtfs_file.to_stdout::<gtfs::Attributions>(),
            }
        }
        Commands::GeoJson(args) => {
            let gtfs_path = args.input;
            let mut gtfs_file = functions::load_gtfs_file(gtfs_path);

            match args.file {
                StandardFiles::Stops => {
                    let json = functions::simple_stops_json(&mut gtfs_file);
                    std::fs::write(args.output, json).expect("Unable to write file");
                }
                StandardFiles::Shapes => {
                    let json: String = functions::simple_shapes_json(&mut gtfs_file);
                    std::fs::write(args.output, json).expect("Unable to write file");
                }
                _ => {
                    println!("Not implemented yet")
                }
            }
        }
        Commands::Extract(args) => {
            let gtfs_path = args.input;
            let mut gtfs_file = functions::load_gtfs_file(gtfs_path);

            match args.file {
                StandardFiles::Stops => {
                    stop::print_stop_details(args.id, &mut gtfs_file)
                }
                StandardFiles::Routes => {
                    route::print_route_details(&mut gtfs_file, args.id);
                }
                _ => {
                    println!("Not implemented yet")
                }
            }
        },
        Commands::Visualize(args) => {
            let gtfs_path: PathBuf = args.input;
            let mut gtfs_file = functions::load_gtfs_file(gtfs_path);

            match args.file {
                StandardFiles::Routes => {
                    let dot = visualize::route_to_dot(args.id, &mut gtfs_file);
                    std::fs::write(args.output, dot).expect("Unable to write file");
                },
                _ => {
                    println!("Not implemented yet")
                },
            }
        }
    }
}

// TODO Reading in GTFS File
// TODO option: clipping by bounds
// TODO option: filtering by various categories
// TODO option: clipping by time

// TODO think about visualisations
