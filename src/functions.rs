mod gtfs;
mod geodata;
use std::collections::HashMap;

use geo_types::{geometry, Coord};
use geodata::{GeoShapeLine, GeoShapePoint, StopsJson};
use geojson::ser;


pub fn load_gtfs_file(gtfs_path: std::path::PathBuf) -> gtfs::GtfsFile {
    let gtfs_file = match gtfs::GtfsFile::new(&gtfs_path) {
        Ok(gtfs_file) =>  gtfs_file,
        Err(error) => panic!("{}", error)
    };
    gtfs_file
}    

pub fn file_list(gtfs_file: &mut gtfs::GtfsFile) {   
    let file_list: Vec<String> = gtfs_file.list_files();
        
    println!("GTFS archive contains: {:?}", file_list);
}

pub fn agency_out(gtfs_file: &mut gtfs::GtfsFile) {      
    let agencies: gtfs::Iter<gtfs::Agency> = gtfs_file.into_iter();
    for agency in agencies {
        println!("{:?}", agency.unwrap())
    }
}

pub fn stops_out(gtfs_file: &mut gtfs::GtfsFile) {      
    let stops: gtfs::Iter<gtfs::Stops> = gtfs_file.into_iter();
    for stop in stops {
        println!("{:?}", stop.unwrap())
    }
}

pub fn routes_out(gtfs_file: &mut gtfs::GtfsFile) {
    let routes: gtfs::Iter<gtfs::Route> = gtfs_file.into_iter();
    for route in routes {
        println!("{:?}", route.unwrap())
    } 
}

pub fn trips_out(gtfs_file: &mut gtfs::GtfsFile) {
    let trips: gtfs::Iter<gtfs::Trip> = gtfs_file.into_iter();
    for trip in trips {
        println!("{:?}", trip.unwrap())
    } 
}

pub fn stop_times_out(gtfs_file: &mut gtfs::GtfsFile) {
    let stop_times: gtfs::Iter<gtfs::StopTime> = gtfs_file.into_iter();
    for stop_time in stop_times {
        match stop_time {
            Ok(result) => println!("{:?}", result),
            Err(e) => {
                match e.kind() {
                    csv::ErrorKind::UnequalLengths { pos, ..} => {
                        if let Some(position) = pos {
                            if position.record() == 16 {
                                println!("You seem to be using the Sample GTFS feed. Yes it's broken: {}", e)
                            } else {
                                println!("{}", e)
                            }
                        } else {
                            println!("{}", e)
                        }
                    },
                    _ => println!("{}", e)
                }
            }
            
        }
    } 
}

pub fn calendar_out(gtfs_file: &mut gtfs::GtfsFile) { 
    let calendar: gtfs::Iter<gtfs::Calendar> = gtfs_file.into_iter();
    for calendar_entry in calendar {
        println!("{:?}", calendar_entry.unwrap())
    }
}

pub fn calendar_dates_out(gtfs_file: &mut gtfs::GtfsFile) {
    let calendar_dates: gtfs::Iter<gtfs::CalendarDates> = gtfs_file.into_iter();
    for calendar_date in calendar_dates {
        println!("{:?}", calendar_date.unwrap())
    }
}

pub fn fare_attributes_out(gtfs_file: &mut gtfs::GtfsFile) {
    let fare_attributes: gtfs::Iter<gtfs::FareAttributes> = gtfs_file.into_iter();
    for fare_attribute in fare_attributes {
        match fare_attribute {
            Ok(result) => println!("{:?}", result),
            Err(error) => println!("{}", error)
        }
    }
}

pub fn fare_rules_out(gtfs_file: &mut gtfs::GtfsFile) {
    let fare_rules: gtfs::Iter<gtfs::FareRule> = gtfs_file.into_iter();
    for fare_rule in fare_rules {
        println!("{:?}", fare_rule.unwrap())
    }
}

pub fn timeframes_out(gtfs_file: &mut gtfs::GtfsFile) {
    let timeframes: gtfs::Iter<gtfs::Timeframe> = gtfs_file.into_iter();
    for timeframe in timeframes {
        println!("{:?}", timeframe.unwrap())
    }
}

pub fn fare_media_out(gtfs_file: &mut gtfs::GtfsFile) {
    let fare_media: gtfs::Iter<gtfs::FareMedium> = gtfs_file.into_iter();
    for fare_medium in fare_media {
        println!("{:?}", fare_medium.unwrap())
    }
}

pub fn fare_products_out(gtfs_file: &mut gtfs::GtfsFile) {
    let fare_products: gtfs::Iter<gtfs::FareProduct> = gtfs_file.into_iter();
    for fare_product in fare_products {
        println!("{:?}", fare_product.unwrap())
    }
}

pub fn fare_leg_rules_out(gtfs_file: &mut gtfs::GtfsFile) {
    let fare_leg_rules: gtfs::Iter<gtfs::FareLegRule> = gtfs_file.into_iter();
    for fare_leg_rule in fare_leg_rules {
        println!("{:?}", fare_leg_rule.unwrap())
    }
}

pub fn fare_tranfer_rules_out(gtfs_file: &mut gtfs::GtfsFile) {
    let fare_transfer_rules: gtfs::Iter<gtfs::FareTransferRule> = gtfs_file.into_iter();
    for fare_transfer_rule in fare_transfer_rules {
        println!("{:?}", fare_transfer_rule.unwrap())
    }
}

pub fn areas_out(gtfs_file: &mut gtfs::GtfsFile) {
    let areas: gtfs::Iter<gtfs::Areas> = gtfs_file.into_iter();
    for area in areas {
        println!("{:?}", area.unwrap())
    }
}

pub fn stop_areas_out(gtfs_file: &mut gtfs::GtfsFile) {
    let stop_areas: gtfs::Iter<gtfs::StopArea> = gtfs_file.into_iter();
    for stop_area in stop_areas {
        println!("{:?}", stop_area.unwrap())
    }
}

pub fn networks_out(gtfs_file: &mut gtfs::GtfsFile) {
    let networks: gtfs::Iter<gtfs::Network> = gtfs_file.into_iter();
    for network in networks {
        println!("{:?}", network.unwrap())
    }
}

pub fn route_networks_out(gtfs_file: &mut gtfs::GtfsFile) {
    let route_networks: gtfs::Iter<gtfs::RouteNetwork> = gtfs_file.into_iter();
    for route_network in route_networks {
        println!("{:?}", route_network.unwrap())
    }
}

pub fn shapes_out(gtfs_file: &mut gtfs::GtfsFile) {
    let shapes: gtfs::Iter<gtfs::Shape> = gtfs_file.into_iter();
    for shape in shapes {
        println!("{:?}", shape.unwrap())
    }
}

pub fn simple_stops_json(gtfs_file: &mut gtfs::GtfsFile) -> String {
    let stops: Vec<gtfs::Stops> = gtfs_file.read_vec();

    let mut json_stops: Vec<geodata::StopsJson> = vec![];
    for stop in stops {
        match StopsJson::from_stop(stop) {
            Some(json_stop) => {
                json_stops.push(json_stop)
            },
            None => continue
        }
    }
    let output_geojson: String = ser::to_feature_collection_string(&json_stops).unwrap();
    output_geojson
}

pub fn shapes_json(gtfs_file: &mut gtfs::GtfsFile) -> String  {
    let shapes: Vec<gtfs::Shape> = gtfs_file.read_vec();
    let mut shapes_map: HashMap<String, Vec<Coord<f64>>> = HashMap::new();

    for shape in shapes {
        let geoshape: GeoShapePoint = GeoShapePoint::from_shape(shape);
        if let Some(vec) = shapes_map.get_mut(geoshape.shape_id.as_str()) {
            let coord: Coord<f64> = geoshape.shape_point.into();
            vec.push(coord);
        } else {
            let key: String = geoshape.shape_id.as_str().to_owned();
            let coord: Coord<f64> = geoshape.shape_point.into();
            let vec: Vec<Coord> = vec![coord];
            shapes_map.insert(key, vec);
        }
    }

    let mut shape_vec: Vec<GeoShapeLine> = Vec::new();
    for k in shapes_map {
        let id: String = k.0;
        let points: Vec<Coord> =  k.1;
        let line: geometry::LineString = geo_types::LineString::new(points);
        let geo_shape_line: GeoShapeLine = GeoShapeLine {
            shape_id: id,
            shape_line: line,
        };
        shape_vec.push(geo_shape_line);
        
    }
    println!("{:?}", shape_vec.first().unwrap());
    let geojson = ser::to_feature_collection_string(&shape_vec);
    match geojson {
        Ok(output) => return output,
        Err(error) => {
            println!("{:?}", error);
            return String::new();
        }
    }

}

