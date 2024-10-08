mod geodata;
pub mod gtfs;
pub mod visualize;
use std::collections::HashMap;

use geo_types::{geometry, Coord};
use geodata::{GeoShapeLine, GeoShapePoint, StopsJson};
use geojson::ser;

pub fn load_gtfs_file(gtfs_path: std::path::PathBuf) -> gtfs::GtfsFile {
    let gtfs_file = match gtfs::GtfsFile::new(&gtfs_path) {
        Ok(gtfs_file) => gtfs_file,
        Err(error) => panic!("{}", error),
    };
    gtfs_file
}

pub fn file_list(gtfs_file: &mut gtfs::GtfsFile) {
    let file_list: Vec<String> = gtfs_file.list_files();

    println!("GTFS archive contains: {:?}", file_list);
}

// Not sure what to do with this special case to handle the sample gtfs
pub fn stop_times_out(gtfs_file: &mut gtfs::GtfsFile) {
    let stop_times: gtfs::Iter<gtfs::StopTime> = gtfs_file.into_iter();
    for stop_time in stop_times {
        match stop_time {
            Ok(result) => println!("{:?}", result),
            Err(e) => {
                match e.kind() {
                    csv::ErrorKind::UnequalLengths { pos, .. } => {
                        if let Some(position) = pos {
                            if position.record() == 16 {
                                println!("You seem to be using the Sample GTFS feed. Yes it's broken: {}", e)
                            } else {
                                println!("{}", e)
                            }
                        } else {
                            println!("{}", e)
                        }
                    }
                    _ => println!("{}", e),
                }
            }
        }
    }
}

// Another odd special case
pub fn fare_attributes_out(gtfs_file: &mut gtfs::GtfsFile) {
    let fare_attributes: gtfs::Iter<gtfs::FareAttributes> = gtfs_file.into_iter();
    for fare_attribute in fare_attributes {
        match fare_attribute {
            Ok(result) => println!("{:?}", result),
            Err(error) => println!("{}", error),
        }
    }
}

// JSON output to file

pub fn simple_stops_json(gtfs_file: &mut gtfs::GtfsFile) -> String {
    let stops: Vec<gtfs::Stop> = gtfs_file.read_vec();

    let mut json_stops: Vec<geodata::StopsJson> = vec![];
    for stop in stops {
        match StopsJson::from_stop(stop) {
            Some(json_stop) => json_stops.push(json_stop),
            None => continue,
        }
    }
    let output_geojson: String = ser::to_feature_collection_string(&json_stops).unwrap();
    output_geojson
}

pub fn simple_shapes_json(gtfs_file: &mut gtfs::GtfsFile) -> String {
    let shapes: Vec<gtfs::Shape> = gtfs_file.read_vec();
    let mut shapes_map: HashMap<String, Vec<Coord<f64>>> = HashMap::new();

    // Create a hashmap of coord vectors for each shape id
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

    // Combine Vectors of points into a line string for a GeoShapeLine struct
    let mut shape_vec: Vec<GeoShapeLine> = Vec::new();
    for k in shapes_map {
        let id: String = k.0;
        let points: Vec<Coord> = k.1;
        let line: geometry::LineString = geo_types::LineString::new(points);
        let geo_shape_line: GeoShapeLine = GeoShapeLine {
            shape_id: id,
            geometry: line,
        };
        shape_vec.push(geo_shape_line);
    }

    let geojson = ser::to_feature_collection_string(&shape_vec);
    match geojson {
        Ok(output) => return output,
        Err(error) => {
            println!("{:?}", error);
            return String::new();
        }
    }
}
