use crate::functions::gtfs::{self, Stop, StopTime, Trip};
use std::collections::BTreeMap;


pub struct FullStop {
    stop_description: Stop,
    stop_times: Vec<StopTime>,
    associated_trips: Vec<Trip>
}



pub fn print_stop_details(stop_id: String, gtfs_file: &mut gtfs::GtfsFile) {
    let stop = extract_stop_info(gtfs_file, &stop_id);
    match stop {
        None => println!("Stop id {} not found", stop_id),
        Some(stop) => {
            println!("{:?}", stop.stop_description);
            println!("{:?}", stop.stop_times);
            println!("{:?}", stop.associated_trips);
        }
    }   
}

fn extract_stop_info(gtfs_file: &mut gtfs::GtfsFile, id: &String) -> Option<FullStop> {
    
    let stop = get_stop_description(gtfs_file, id)?;
    let stop_times = get_stop_times(gtfs_file, id);
    let associated_trips = get_matching_trips(gtfs_file, &stop_times);

    
    let stop_info = FullStop {
        stop_description: stop,
        stop_times: stop_times,
        associated_trips: associated_trips,
    };

    Some(stop_info)
}

fn get_stop_description(gtfs_file: &mut gtfs::GtfsFile, id: &String) -> Option<Stop> {
    let stops: Vec<gtfs::Stop> = gtfs_file.read_vec();
    let stops_map = stops_hash(stops);
    let stop = stops_map.get(id)?;
    Some(stop.clone())
}

fn get_stop_times(gtfs_file: &mut gtfs::GtfsFile, stop_id: &String) -> Vec<StopTime> {
    let stop_times: Vec<gtfs::StopTime> = gtfs_file.read_vec();
    let mut local_stop_vec: Vec<StopTime> = Vec::new();
    
    for stop_time in stop_times {
        if let Some(id) = stop_time.stop_id.clone() {
            if id == stop_id.to_owned() {
                local_stop_vec.push(stop_time)
            }
        }
        
    }

    local_stop_vec
}

fn get_matching_trips(gtfs_file: &mut gtfs::GtfsFile, stop_times: &Vec<StopTime>) -> Vec<Trip> {
    let trips: Vec<gtfs::Trip> = gtfs_file.read_vec();
    let mut trip_map: BTreeMap<String, Trip> = BTreeMap::new();
    for trip in trips {
        trip_map.insert(trip.trip_id.clone(), trip);
    }

    let mut matching_trips: Vec<Trip> = Vec::new();
    for stop_time in stop_times {
        let matching_trip = trip_map.remove(&stop_time.trip_id).unwrap();
        matching_trips.push(matching_trip);
    }

    matching_trips
}

fn stops_hash(stops: Vec<Stop>) -> BTreeMap<String, Stop> {
    let mut stops_map = BTreeMap::new();
    for stop in stops {
        stops_map.insert(stop.stop_id.to_owned(), stop);
    }
    stops_map
}
