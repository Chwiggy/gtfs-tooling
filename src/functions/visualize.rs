use std::collections::HashMap;

use petgraph;

use crate::objects::{self, route::stopid_stops_hash};

use super::gtfs::{self, StopTime};

pub fn route_to_dot(route_id: String, gtfs_file: &mut gtfs::GtfsFile) -> String {
    
    let route = objects::route::extract_route_info(gtfs_file, &route_id).unwrap();
    println!("Extracting Graph for line {:?}", route.route_description.route_short_name);
    // let parents = stopid_stops_hash(&route.parent_stops(gtfs_file));
    let stops = stopid_stops_hash(&route.associated_stops);
    
    let mut trips_map: HashMap<String, Vec<StopTime>> = HashMap::new();
    for stop_time in &route.associated_stop_times {
        let stop_pos = stop_time.stop_sequence.unwrap() as usize;
        match trips_map.get_mut(&stop_time.trip_id) {
            Some(stop_time_vec) => {
                stop_time_vec.insert(stop_pos, stop_time.to_owned());
            },
            None => {
                let mut stop_time_vec: Vec<StopTime> = Vec::new();
                let trip_id = stop_time.trip_id.to_owned();
                stop_time_vec.insert(stop_pos, stop_time.to_owned());
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
                    let edge = (prev_stop_id.to_owned(), stop_id.clone());
                    match edges.get(&edge) {
                        None => {edges.insert(edge, 1);},
                        Some(count) => {edges.insert(edge, count + 1);}
                    }
                    previous_stop_id = Some(stop_id);                                  
                }
            }
        }
    }                                         
    
    let mut graph: petgraph::prelude::GraphMap<&str, &u64, petgraph::Directed> = petgraph::graphmap::GraphMap::new();
    
    let parent_stations = route.all_stops(gtfs_file);
    let parents = stopid_stops_hash(&parent_stations);
    let mut fancy_edges: HashMap<(String, String), u64> = HashMap::new();
    for (edge, count) in edges.iter() {
        let name_a;
        let name_b;
        if let Some(name) = &parents.get(&edge.0).unwrap().stop_name {
            name_a=name.to_owned();
        } else {
            name_a= edge.0.clone();
        }
        if let Some(name) = &parents.get(&edge.1).unwrap().stop_name {
            name_b=name.to_owned();
        } else {
            name_b= edge.1.clone();
        }
        fancy_edges.insert((name_a,name_b), *count);
    }


    for (edge, count) in fancy_edges.iter() {
        graph.add_edge(edge.0.as_ref(), edge.1.as_ref(), count);
    }

    
    format!("{:?}", petgraph::dot::Dot::new(&graph))

}
