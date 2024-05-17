mod gtfs;

fn main() {
    // TODO add user input instead of hard coding.
    let gtfs_path: String = String::from("test_data/20240513_fahrplaene_gesamtdeutschland_gtfs.zip");

    let gtfs_file: gtfs::GtfsFile = gtfs::GtfsFile::new(&gtfs_path);
    let file_list: Vec<String> = gtfs_file.list_files();

    println!("GTFS archive contains: {:?}", file_list);


}

// TODO Reading in GTFS File
// TODO option: clipping by bounds
// TODO option: filtering by various categories
// TODO option: clipping by time

// TODO think about visualisations