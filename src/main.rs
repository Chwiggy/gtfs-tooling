mod gtfs;
use clap::{Parser, Subcommand, Args};
use crate::gtfs::{GtfsObject, CalendarDates};

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
    CalendarDates
}

fn main() {
    // TODO add user input instead of hard coding.
    let args = Cli::parse();

    match args.command {
        Commands::Echo(args) => {
            let gtfs_path = args.input;
            match args.file {
                StandardFiles::Files => {
                    file_list(gtfs_path)
                },
                StandardFiles::CalendarDates => {
                    calendar_dates_out(gtfs_path)
                },

            }
            
        }
    }
    
}

fn load_gtfs_file(gtfs_path: std::path::PathBuf) -> gtfs::GtfsFile {
    let gtfs_file = match gtfs::GtfsFile::new(&gtfs_path) {
        Ok(gtfs_file) =>  gtfs_file,
        Err(error) => panic!("{}", error)
    };
    gtfs_file
}    

fn file_list(gtfs_path: std::path::PathBuf) {
    let mut gtfs_file = load_gtfs_file(gtfs_path);

    let file_list: Vec<String> = gtfs_file.list_files();
        
    println!("GTFS archive contains: {:?}", file_list);
}

fn calendar_dates_out(gtfs_path: std::path::PathBuf) {
    let mut gtfs_file = load_gtfs_file(gtfs_path);
               
    // let agencies: Vec<Agency> = Agency::from_gtfs_file(&mut gtfs_file);
    
    
    let stop: Vec<CalendarDates> = CalendarDates::from_gtfs_file(&mut gtfs_file);
    println!("{:?}",stop)
}







// TODO Reading in GTFS File
// TODO option: clipping by bounds
// TODO option: filtering by various categories
// TODO option: clipping by time

// TODO think about visualisations