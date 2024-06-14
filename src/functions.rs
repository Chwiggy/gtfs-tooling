mod gtfs;
mod geodata;
use geojson::ser;
use crate::functions::geodata::from_stop;


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

pub fn stops_out(gtfs_path: std::path::PathBuf) {
    let mut gtfs_file = load_gtfs_file(gtfs_path);  

    let stops: gtfs::Iter<gtfs::Stops> = gtfs_file.into_iter();
    for stop in stops {
        println!("{:?}", stop.unwrap())
    }
}

pub fn calendar_dates_out(gtfs_path: std::path::PathBuf) {
    let mut gtfs_file = load_gtfs_file(gtfs_path);  
    
    let calendar_dates: gtfs::Iter<gtfs::CalendarDates> = gtfs_file.into_iter();
    for calendar_date in calendar_dates {
        println!("{:?}", calendar_date.unwrap())
    }
}

pub fn simple_stops_json(gtfs_path: std::path::PathBuf) -> String {
    let mut gtfs_file = load_gtfs_file(gtfs_path);

    let stops: Vec<gtfs::Stops> = gtfs_file.read_vec();

    let mut json_stops: Vec<geodata::StopsJson> = vec![];
    for stop in stops {
        match from_stop(stop) {
            Some(json_stop) => {
                json_stops.push(json_stop)
            },
            None => continue
        }
    }
    let output_geojson = ser::to_feature_collection_string(&json_stops).unwrap();
    output_geojson
}

