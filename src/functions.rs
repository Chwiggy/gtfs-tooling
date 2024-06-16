mod gtfs;
mod geodata;
use geojson::ser;
use crate::functions::geodata::from_stop;


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

pub fn simple_stops_json(gtfs_file: &mut gtfs::GtfsFile) -> String {
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

