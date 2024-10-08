use std::collections::BTreeMap;

use crate::functions::gtfs::{self, Route, Stop, StopTime, Trip};

#[derive(Clone)]
pub struct FullRoute {
    pub route_description: Route,
    pub trips: Vec<Trip>,
    pub associated_stop_times: Vec<StopTime>,
    pub associated_stops: Vec<Stop>,
}

// impl FullRoute {
//     pub fn add_parent_stops(self, gtfs_file: &mut gtfs::GtfsFile) -> Self {
//         let stops: Vec<Stop> = gtfs_file.read_vec();
//         let stops_map = stopid_stops_hash(&stops);
//         let mut stops = self.associated_stops.clone();
//         for stop in self.associated_stops {
//             if let Some(parent_id) = stop.parent_station {
//                 let parent_stop = stops_map.get(&parent_id).unwrap();
//                 stops.push(parent_stop.clone());
//             }
//         }
        

//         FullRoute {
//             route_description: self.route_description,
//             trips: self.trips,
//             associated_stop_times: self.associated_stop_times,
//             associated_stops: stops
//         }
//     }

//     pub fn parent_stops(&self, gtfs_file: &mut gtfs::GtfsFile) -> Vec<Stop> {
//         let mut stops: Vec<Stop> = gtfs_file.read_vec();
//         let mut stops_map = stopid_stops_hash(&stops);
//         let mut parents: Vec<Stop> = Vec::new();
//         for stop in &self.associated_stops {
//             if let Some(parent_station) = &stop.parent_station {
//                 parents.push(stops_map.remove(parent_station).unwrap())
//             }
//         }       

//         parents
//     }
// }

pub fn print_route_details(gtfs_file: &mut gtfs::GtfsFile, route_id:String) {
    let route = extract_route_info(gtfs_file, &route_id);
    match route {
        None => println!("Stop id {} not found", route_id),
        Some(route) => {
            println!("{:?}", route.route_description);
            println!("{:?}", route.trips);
            println!("{:?}", route.associated_stop_times);
            println!("{:?}", route.associated_stops)
        }
    }
}

pub fn extract_route_info(gtfs_file: &mut gtfs::GtfsFile, id: &String) -> Option<FullRoute> {
    let route = get_route_description(gtfs_file, id)?;
    let trips = get_trips(gtfs_file, id);
    let associated_stop_times = get_stop_times(gtfs_file, &trips);
    let associated_stops = get_stops(gtfs_file, &associated_stop_times);

    let route_info = FullRoute {
        route_description: route,
        trips,
        associated_stop_times,
        associated_stops
    };

    Some(route_info)
}

fn get_stops(gtfs_file: &mut gtfs::GtfsFile, stop_times: &[StopTime]) -> Vec<Stop> {
    let stops: Vec<Stop> = gtfs_file.read_vec();
    let mut stops_map = stopid_stops_hash(&stops);

    let mut matching_stops: BTreeMap<String, Stop> = BTreeMap::new();
    for stop_time in stop_times {
        let stop_id = stop_time.stop_id.clone().unwrap();
        let stop = stops_map.remove_entry(&stop_id);
        if let Some(stop_entry) = stop {
            matching_stops.insert(stop_entry.0, stop_entry.1);
        }
    }
    matching_stops.into_values().collect()
}

pub fn stopid_stops_hash(stops: &Vec<Stop>) -> BTreeMap<String, Stop> {
    let mut stops_map: BTreeMap<String, Stop> = BTreeMap::new();
    for stop in stops {
        stops_map.insert(stop.stop_id.to_owned(), stop.clone());
    }
    stops_map
}

fn get_stop_times(gtfs_file: &mut gtfs::GtfsFile, trips: &[Trip]) -> Vec<StopTime> {
    let stop_times: Vec<StopTime> = gtfs_file.read_vec();

    let mut trip_ids: Vec<String> = Vec::new();
    for trip in trips {
        trip_ids.push(trip.trip_id.to_owned());
    }

    let mut matching_stop_times: Vec<StopTime> = Vec::new();
    for stop_time in stop_times {
        if trip_ids.contains(&stop_time.trip_id) {
            matching_stop_times.push(stop_time);
        }
    }

    matching_stop_times
}

fn get_trips(gtfs_file: &mut gtfs::GtfsFile, route_id: &str) -> Vec<Trip> {
    let trips: Vec<Trip> = gtfs_file.read_vec();
    let mut route_trips: Vec<Trip> = Vec::new();

    for trip in trips {
        if trip.route_id == route_id.to_owned() {
                route_trips.push(trip)
        }
    }

    route_trips
}

fn get_route_description(gtfs_file: &mut gtfs::GtfsFile, id: &String) -> Option<Route> {
    let routes: Vec<Route> = gtfs_file.read_vec();
    let routes_map: BTreeMap<String, Route> = routes_hash(routes);
    let route = routes_map.get(id)?;
    Some(route.clone())
}

fn routes_hash(routes: Vec<Route>) -> BTreeMap<String, Route> {
    let mut routes_map = BTreeMap::new();
    for route in routes {
        routes_map.insert(route.route_id.to_owned(), route);
    }
    routes_map
}
