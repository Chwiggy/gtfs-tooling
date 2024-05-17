use polars::io::csv::CsvReader;
use polars::prelude::SerReader;

mod gtfs;


fn main() {
    // TODO add user input instead of hard coding.
    let gtfs_path: String = String::from("test_data/sample-feed-1.zip");

    let mut gtfs_file = match gtfs::GtfsFile::new(&gtfs_path) {
        Ok(gtfs_file) =>  gtfs_file,
        Err(error) => panic!("{}", error)
    };

    
    let file_list: Vec<String> = gtfs_file.list_files();
    println!("GTFS archive contains: {:?}", &file_list);

    let path: String = gtfs_path.clone();
    let df = CsvReader::from_path(path).unwrap()
        .has_header(true)
        .finish();

    let agencies = match df {
        Ok(data) =>  data,
        Err(error) => panic!("Error: {}", error)
    };

    println!("{}", agencies);


    

    
    

}


// TODO Reading in GTFS File
// TODO option: clipping by bounds
// TODO option: filtering by various categories
// TODO option: clipping by time

// TODO think about visualisations