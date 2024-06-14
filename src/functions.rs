mod gtfs;
use crate::functions::gtfs::GtfsObject;

fn load_gtfs_file(gtfs_path: std::path::PathBuf) -> gtfs::GtfsFile {
    let gtfs_file = match gtfs::GtfsFile::new(&gtfs_path) {
        Ok(gtfs_file) =>  gtfs_file,
        Err(error) => panic!("{}", error)
    };
    gtfs_file
}    

pub fn file_list(gtfs_path: std::path::PathBuf) {
    let mut gtfs_file = load_gtfs_file(gtfs_path);

    let file_list: Vec<String> = gtfs_file.list_files();
        
    println!("GTFS archive contains: {:?}", file_list);
}

pub fn calendar_dates_out(gtfs_path: std::path::PathBuf) {
    let mut gtfs_file = load_gtfs_file(gtfs_path);
               
    // let agencies: Vec<Agency> = Agency::from_gtfs_file(&mut gtfs_file);
    
    
    let stop: Vec<gtfs::CalendarDates> = gtfs::CalendarDates::from_gtfs_file(&mut gtfs_file);
    println!("{:?}",stop)
}