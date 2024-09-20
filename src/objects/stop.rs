use crate::functions::gtfs::{self, Stop, StopTime};
use std::collections::BTreeMap;


pub struct FullStop {
    stop_description: Stop,
    stop_times: Option<Vec<StopTime>>,
    associated_trips: Option<Vec<String>>
}

impl FullStop {
    fn from_stop(stop: &Stop) -> Self {
        FullStop{
            stop_description: stop.clone(),
            stop_times: None,
            associated_trips: None,
        }
    }

    fn add_stop_times(mut self, gtfs_file: &mut gtfs::GtfsFile) -> Self {
        let stop_times: Vec<gtfs::StopTime> = gtfs_file.read_vec();
        let mut local_stop_vec: Vec<StopTime> = Vec::new();
        
        for stop_time in stop_times {
            if let Some(id) = stop_time.stop_id.clone() {
                if id == self.stop_description.stop_id{
                    local_stop_vec.push(stop_time)
                }
            }
            
        }

        self.stop_times = Some(local_stop_vec);
        self
    }

    fn add_associated_trips(mut self) -> Self {
        if let Some(stop_times) = &self.stop_times {
            let mut trips_vec: Vec<String> = Vec::new();
            for time in stop_times {
                trips_vec.push(time.trip_id.to_string());
            }
            self.associated_trips = Some(trips_vec);
        } 
        self
    }
}


pub fn print_stop_details(stop_id: String, gtfs_file: &mut gtfs::GtfsFile) {
    let stop = extract_stop_info(gtfs_file, &stop_id);
    match stop {
        None => println!("Stop id {} not found", stop_id),
        Some(stop) => {
            println!("{:?}", stop.stop_description);
            if let Some(trips) = stop.associated_trips {
                for id in trips {
                    println!("{}", id);
                }
            }
        }
    }   
}

fn extract_stop_info(gtfs_file: &mut gtfs::GtfsFile, id: &String) -> Option<FullStop> {
    let stops: Vec<gtfs::Stop> = gtfs_file.read_vec();

    let stops_map = stops_hash(stops);

    let stop = stops_map.get(id)?;

    let mut full_stop = FullStop::from_stop(stop);
    full_stop = full_stop.add_stop_times(gtfs_file);
    full_stop = full_stop.add_associated_trips();
    Some(full_stop)
}

fn stops_hash(stops: Vec<Stop>) -> BTreeMap<String, Stop> {
    let mut stops_map = BTreeMap::new();
    for stop in stops {
        stops_map.insert(stop.stop_id.to_owned(), stop);
    }
    stops_map
}
