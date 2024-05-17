mod gtfs;

fn main() {
    // TODO add user input instead of hard coding.
    let gtfs_path: String = String::from("test_data/sample-feed-1.zip");

    let mut gtfs_file = match gtfs::GtfsFile::new(&gtfs_path) {
        Ok(gtfs_file) =>  gtfs_file,
        Err(error) => panic!("{}", error)
    };

    
    let file_list: Vec<String> = gtfs_file.list_files();

    println!("GTFS archive contains: {:?}", file_list);
}

// TODO Reading in GTFS File
// TODO option: clipping by bounds
// TODO option: filtering by various categories
// TODO option: clipping by time

// TODO think about visualisations