mod gtfs;

use crate::gtfs::{GtfsObject, Trip};

fn main() {
    // TODO add user input instead of hard coding.
    let gtfs_path: String = String::from("test_data/sample-feed-1.zip");

    let mut gtfs_file = match gtfs::GtfsFile::new(&gtfs_path) {
        Ok(gtfs_file) =>  gtfs_file,
        Err(error) => panic!("{}", error)
    };

    
    let file_list: Vec<String> = gtfs_file.list_files();

    println!("GTFS archive contains: {:?}", file_list);

    // let agencies: Vec<Agency> = Agency::from_gtfs_file(&mut gtfs_file);


    let stop: Vec<Trip> = Trip::from_gtfs_file(&mut gtfs_file);
    println!("{:?}",stop)
    
}


// TODO Reading in GTFS File
// TODO option: clipping by bounds
// TODO option: filtering by various categories
// TODO option: clipping by time

// TODO think about visualisations