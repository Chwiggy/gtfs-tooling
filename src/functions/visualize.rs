use std::collections::HashMap;

use petgraph;

use crate::objects::{self, route::stopid_stops_hash};

use super::gtfs::{self, StopTime};

pub fn route_to_dot(route_id: String, gtfs_file: &mut gtfs::GtfsFile) -> String {
    
    let route = objects::route::extract_route_info(gtfs_file, &route_id).unwrap();
    // let parents = stopid_stops_hash(&route.parent_stops(gtfs_file));
    let stops = stopid_stops_hash(&route.associated_stops);
    
    let mut trips_map: HashMap<String, Vec<StopTime>> = HashMap::new();
    for stop_time in route.associated_stop_times {
        let stop_pos = stop_time.stop_sequence.unwrap() as usize;
        match trips_map.get_mut(&stop_time.trip_id) {
            Some(stop_time_vec) => {
                stop_time_vec.insert(stop_pos, stop_time);
            },
            None => {
                let mut stop_time_vec: Vec<StopTime> = Vec::new();
                let trip_id = stop_time.trip_id.to_owned();
                stop_time_vec.insert(stop_pos, stop_time);
                trips_map.insert(trip_id, stop_time_vec);
            }
        }
    }

    let mut edges: HashMap<(String, String), u64> = HashMap::new();
    for stop_time_vec in trips_map.into_values() {
        let mut previous_stop_id: Option<String> = None;
        for stop_time in stop_time_vec {
            let stop = stops.get(&stop_time.clone().stop_id.unwrap()).unwrap();
            match previous_stop_id {
                None => {
                    if let Some(parent) = &stop.parent_station {
                        previous_stop_id = Some(parent.to_owned());
                    } else {
                        previous_stop_id = stop_time.stop_id
                    }
                },
                Some(ref prev_stop_id) => {
                    let stop_id: String;
                    if let Some(parent) = &stop.parent_station {
                        stop_id = parent.to_owned();
                    } else {
                        stop_id = stop_time.clone().stop_id.unwrap();
                    }
                    let edge = (prev_stop_id.to_owned(), stop_time.clone().stop_id.unwrap());
                    match edges.get(&edge) {
                        None => {edges.insert(edge, 1);},
                        Some(count) => {edges.insert(edge, count + 1);}
                    }
                    previous_stop_id = Some(stop_id);                                  
                }
            }
        }
    }

    // for edge in edges.keys() {
    //     if let Some(reverse_count) = edges.remove(&(edge.1, edge.0)) {
    //         let count = edges.get(edge).unwrap();
    //         edges.insert(edge.clone(), count + reverse_count);
    //     }
    // }
    
    let mut graph: petgraph::prelude::GraphMap<&str, &u64, petgraph::Undirected> = petgraph::graphmap::UnGraphMap::new();
    
    for (edge, count) in edges.iter() {
        graph.add_edge(edge.0.as_ref(), edge.1.as_ref(), count);
    }

    
    format!("{:?}", petgraph::dot::Dot::new(&graph))

}
