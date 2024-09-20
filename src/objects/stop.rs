use crate::functions::gtfs::{self, Stop, StopTime};
use std::collections::BTreeMap;


pub struct FullStop {
    stop_description: Stop,
    stop_times: Option<Vec<StopTime>>
}

impl FullStop {
    fn from_stop(stop: &Stop) -> Self {
        FullStop{
            stop_description: stop.clone(),
            stop_times: None,
        }
    }
}

pub fn print_stop_details(stop_id: String, gtfs_file: &mut gtfs::GtfsFile) {
    let stop = extract_stop_info(gtfs_file, &stop_id);
    match stop {
        None => println!("Stop id {} not found", stop_id),
        Some(stop) => println!("{:?}", stop.stop_description)
    }   
}

fn extract_stop_info(gtfs_file: &mut gtfs::GtfsFile, id: &String) -> Option<FullStop> {
    let stops: Vec<gtfs::Stop> = gtfs_file.read_vec();

    let stops_map = stops_hash(stops);

    let stop = stops_map.get(id)?;

    let full_stop = FullStop::from_stop(stop);
    Some(full_stop)
}

fn stops_hash(stops: Vec<Stop>) -> BTreeMap<String, Stop> {
    let mut stops_map = BTreeMap::new();
    for stop in stops {
        stops_map.insert(stop.stop_id.to_owned(), stop);
    }
    stops_map
}
