use core::fmt;
use std::fs::File;
use std::io::Read;

use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use csv;

pub use zip::read::ZipArchive;

pub struct GtfsFile {
    pub(crate) archive: ZipArchive<File>
}


impl GtfsFile {
    pub fn new(filepath: &String) -> Result<Self, GtfsSpecError> {
        let reader: File = std::fs::File::open(filepath).unwrap();
        let archive: ZipArchive<File> = ZipArchive::new(reader).unwrap();
        let mut gtfs_candidate = GtfsFile { archive };
        match gtfs_candidate.check_file_validity() {
            Ok(_) => return Ok(gtfs_candidate),
            Err(error) => {
                println!("{}", error);
                return Err(error)
            }
        }
    }

    pub fn list_files(&mut self) -> Vec<String> {
    
        let mut files: Vec<String> = vec![];
    
        for i in 0..self.archive.len() {
            let file: zip::read::ZipFile = self.archive.by_index(i).unwrap();
            files.push(String::from(file.name()));
        }
    
        files
    }

    fn check_file_validity(&mut self) -> Result<bool, GtfsSpecError> {
        let files: Vec<String> = self.list_files();
        
        if !files.contains(&String::from("agency.txt")) {
            return Err(GtfsSpecError);
        } else if !files.contains(&String::from("stops.txt")){
            return Err(GtfsSpecError);
        } else if !files.contains(&String::from("routes.txt")) {
            return Err(GtfsSpecError);
        } else if !files.contains(&String::from("trips.txt")) {
            return Err(GtfsSpecError);
        } else if !files.contains(&String::from("stop_times.txt")) {
            return Err(GtfsSpecError);
        } else if !files.contains(&String::from("calendar.txt")) || !files.contains(&String::from("calendar_dates.txt")) {
            return Err(GtfsSpecError);
        } 

        Ok(true)
    }
    
    pub fn extract_by_name(&mut self, name: &str) -> String {
        let mut buffer = String::new();
        match self.archive.by_name(name).unwrap().read_to_string(&mut buffer) {
            Ok(_) => buffer,
            _ => panic!("Could not extract file {}", name)
        }
    }

}

pub trait GtfsObject {
    fn from_gtfs_file(gtfs_file: &mut GtfsFile) -> Vec<Self> where Self: Sized;

    const FILE: &'static str;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Agency {
    pub agency_id: Option<String>,
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: Option<String>,
    pub agency_phone: Option<String>,
    pub agency_fare_url: Option<String>,
    pub agency_email: Option<String>,
}

impl GtfsObject for Agency {
    fn from_gtfs_file(gtfs_file: &mut GtfsFile) -> Vec<Self> {
        let agency_text = gtfs_file.extract_by_name(Self::FILE);

        let mut reader = csv::Reader::from_reader(agency_text.as_bytes());
        let iter = reader.deserialize();
        let mut agencies: Vec<Agency> = Vec::new();
        for result in iter {
            let record: Agency = result.unwrap();
            agencies.push(record)
        }
        agencies
    }

    const FILE: &'static str = "agency.txt";
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Stops {
    pub stop_id: String,
    pub stop_code: Option<String>,
    pub stop_name: Option<String>,
    pub tts_stop_name: Option<String>,
    pub stop_desc: Option<String>,
    pub stop_lat: Option<f64>,
    pub stop_lon: Option<f64>,
    pub zone_id: Option<String>,
    pub stop_url: Option<String>,
    pub location_type: Option<LocationType>,
    pub parent_station: Option<String>,
    pub stop_timezone: Option<String>,
    pub wheelchair_boarding: Option<WheelchairAccessibility>,
    pub level_id: Option<String>,
    pub platform_code: Option<String>,
}

impl GtfsObject for Stops {
    fn from_gtfs_file(gtfs_file: &mut GtfsFile) -> Vec<Self> {
        let stops_text = gtfs_file.extract_by_name(Self::FILE);

        let mut reader = csv::Reader::from_reader(stops_text.as_bytes());
        let iter = reader.deserialize();
        let mut stops: Vec<Stops> = Vec::new();
        for result in iter {
            let record: Stops = result.unwrap();
            stops.push(record)
        }
        stops
    }

    const FILE: &'static str = "stops.txt";
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum LocationType {
    Stop = 0,
    Station = 1,
    EntranceExit = 2,
    GenericNode = 3,
    BoardingArea = 4,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum WheelchairAccessibility {
    // NB: These depend on other fields in the stop field and are a bit of a mess. Please consider
    // them to be -ish
    Unknown = 0,
    Yes = 1,
    No = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    pub route_id: String,
    pub agency_id: Option<String>,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub route_desc: Option<String>,
    pub route_type: Option<RouteType>,
    pub route_url: Option<String>,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>,
    pub route_sort_order: Option<u64>,
    pub continuous_pickup: Option<PickupType>,
    pub continuous_drop_off: Option<PickupType>,
    pub network_id: Option<String>,
}


impl GtfsObject for Route {
    fn from_gtfs_file(gtfs_file: &mut GtfsFile) -> Vec<Self> {
        let routes_text = gtfs_file.extract_by_name(Self::FILE);

        let mut reader = csv::Reader::from_reader(routes_text.as_bytes());
        let iter = reader.deserialize();
        let mut routes: Vec<Route> = Vec::new();
        for result in iter {
            let record: Route = result.unwrap();
            routes.push(record)
        }
        routes
    }

    const FILE: &'static str = "routes.txt";
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RouteType {
    Tram = 0,
    Metro = 1,
    Rail = 2,
    Bus = 3,
    Ferry = 4,
    CableTram = 5,
    AerialLift = 6,
    Funicular = 7,
    Trolleybus = 11,
    Monorail = 12,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PickupType {
    RegularSchedule = 0,
    NoPickup = 1,
    PhoneAgency = 2,
    CoordinateWithDriver = 3,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Trip {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: Option<String>,
    pub trip_short_name: Option<String>,
    pub direction_id: Option<Direction>,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
    pub wheelchair_accessible: Option<WheelchairAccessibility>,
    pub bikes_allowed: Option<BikesAllowed>,
}

impl GtfsObject for Trip {
    const FILE: &'static str = "trips.txt";

    fn from_gtfs_file(gtfs_file: &mut GtfsFile) -> Vec<Self> {
        let trip_text = gtfs_file.extract_by_name(Self::FILE);

        let mut reader = csv::Reader::from_reader(trip_text.as_bytes());
        let iter = reader.deserialize();
        let mut trips: Vec<Trip> = Vec::new();
        for result in iter {
            let record: Trip = result.unwrap();
            trips.push(record)
        }
        trips
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Direction {
    Outbound = 0,
    Inbound = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BikesAllowed {
    Unknown = 0,
    Yes = 1,
    No = 2,
}



pub struct GtfsSpecError;

impl fmt::Display for GtfsSpecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The files provided do not match GTFS specifications")
    }
}

impl fmt::Debug for GtfsSpecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}




#[test]
fn test_new_gtfsfile_loading() {
    let data: Vec<&str> = vec!["agency.txt", "calendar.txt", "calendar_dates.txt", "fare_attributes.txt", "fare_rules.txt", "frequencies.txt", "routes.txt", "shapes.txt", "stop_times.txt", "stops.txt", "trips.txt"];
    let mut expected: Vec<String> = Vec::new();
    for file in data {
        expected.push(String::from(file))
    }

    let path: String = String::from("test_data/sample-feed-1.zip");
    let result: Vec<String> = GtfsFile::new(&path).unwrap().list_files();

    assert_eq!(expected, result);
}

#[test]
fn test_new_broken_gtfs() {
    let path = String::from("test_data/sample-feed-1-broken.zip");
    let _gtfs_file = match GtfsFile::new(&path) {
        Ok(_) => panic!("This gtfs file should have been rejected as invalid"),
        Err(error) => println!("correctly rejected invalid gtfs file, with error {}", error)
    };
}

#[test]
fn test_different_parsings() {
    let path: String = String::from("test_data/sample-feed-1.zip");
    let mut gtfs_file = GtfsFile::new(&path).unwrap();
    let stops = Stops::from_gtfs_file(&mut gtfs_file);

    let result = &stops[0];

    let expected = &Stops {
        stop_id: String::from("FUR_CREEK_RES"),
        stop_code: None,
        stop_name: Some(String::from("Furnace Creek Resort (Demo)")),
        tts_stop_name: None,
        stop_desc: None,
        stop_lat: Some(36.425288),
        stop_lon: Some(-117.133162),
        zone_id: None,
        stop_url: None,
        location_type: None,
        parent_station: None,
        stop_timezone: None,
        wheelchair_boarding: None,
        level_id: None,
        platform_code: None
    };

    assert_eq!(result, expected)
    
}