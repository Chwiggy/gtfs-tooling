use core::fmt;
use std::path::PathBuf;
use std::{fs::File, usize};

use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as,BoolFromInt};

use chrono::NaiveDate;

use csv;

pub use zip::read::ZipArchive;

pub struct GtfsFile {
    archive: ZipArchive<File>
}

pub type Iter<'a, T> = csv::DeserializeRecordsIntoIter<zip::read::ZipFile<'a>, T>;

impl GtfsFile {
    pub fn new(filepath: &PathBuf) -> Result<Self, GtfsSpecError> {
        let reader: File = std::fs::File::open(filepath).expect("unable to read file");
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

    pub fn into_iter<T>(&mut self) -> Iter<T>
    where
        T: for <'a> GtfsObject + for<'de> serde::Deserialize<'de>
    {
        let file = self.archive.by_name(T::FILE).unwrap();
        let reader = csv::Reader::from_reader(file);
        reader.into_deserialize::<T>()
    }

    pub fn read_vec<T>(&mut self) -> Vec<T>
    where
        T: for <'a> GtfsObject + for<'de> serde::Deserialize<'de>
    {
        let mut output: Vec<T> = vec![];
        for result in self.into_iter() {
            let record: T = result.unwrap();
            output.push(record);
        }
        output
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
}

pub trait GtfsObject {
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

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StopTime {
    pub trip_id: Option<String>,
    pub arrival_time: Option<Time>,
    pub departure_time: Option<Time>,
    pub stop_id: Option<String>,
    pub stop_sequence: Option<u64>,
    pub stop_headsign: Option<String>,
    pub pickup_type: Option<PickupType>,
    pub drop_off_type: Option<PickupType>,
    pub continuous_pickup: Option<PickupType>,
    pub contiuous_drop_off: Option<PickupType>,
    pub shape_dist_travelled: Option<f64>,
    pub timepoint: Option<TimepointType>,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Time {
    pub h: u64,
    pub m: u64,
    pub s: u64,
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}:{}:{}", self.h, self.m, self.s);
        serializer.serialize_str(&s)
    }
}

// TODO add check for malformed minutes or seconds
impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s: Vec<&str> = s.split(':').collect();
        match s.len() {
            3 => {
                let h = s[0].parse().map_err(serde::de::Error::custom)?;
                let m = s[1].parse().map_err(serde::de::Error::custom)?;
                let s = s[2].parse().map_err(serde::de::Error::custom)?;
                Ok(Time { h, m, s })
            },
            _ => Err(serde::de::Error::custom("Malformatted time")),
        }
    }
}


#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum TimepointType {
    Approximate = 0,
    Exact = 1,
}

impl GtfsObject for StopTime {
    const FILE: &'static str = "stop_times.txt";
}


#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Calendar {
    pub service_id: String,
    #[serde_as(as = "BoolFromInt")]
    pub monday: bool,
    #[serde_as(as = "BoolFromInt")]
    pub tuesday: bool,
    #[serde_as(as = "BoolFromInt")]
    pub wednesday: bool,
    #[serde_as(as = "BoolFromInt")]
    pub thursday: bool,
    #[serde_as(as = "BoolFromInt")]
    pub friday: bool,
    #[serde_as(as = "BoolFromInt")]
    pub saturday: bool,
    #[serde_as(as = "BoolFromInt")]
    pub sunday: bool,
    #[serde(with = "date")]
    pub start_date: NaiveDate,
    #[serde(with = "date")]
    pub end_date: NaiveDate,
}

mod date {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y%m%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }

}

impl GtfsObject for Calendar {
    const FILE: &'static str = "calendar.txt";
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CalendarDates {
    pub service_id: String,
    #[serde(with = "date")]
    pub date: NaiveDate,
    pub exception_type: CalendarException,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CalendarException {
    Added = 1,
    Removed = 2,
}

impl GtfsObject for CalendarDates {
    const FILE: &'static str = "calendar_dates.txt";
}

#[test]
fn test_new_gtfsfile_loading() {
    let expected_data: Vec<&str> = vec!["agency.txt", "calendar.txt", "calendar_dates.txt", "fare_attributes.txt", "fare_rules.txt", "frequencies.txt", "routes.txt", "shapes.txt", "stop_times.txt", "stops.txt", "trips.txt"];
    let mut expected: Vec<String> = Vec::new();
    for file_name in expected_data {
        expected.push(String::from(file_name))
    }

    let path: PathBuf = PathBuf::from("test_data/sample-feed-1.zip");
    let result: Vec<String> = GtfsFile::new(&path).unwrap().list_files();

    assert_eq!(expected, result);
}

#[test]
fn test_new_broken_gtfs() {
    let path: PathBuf = PathBuf::from("test_data/sample-feed-1-broken.zip");
    let _gtfs_file = match GtfsFile::new(&path) {
        Ok(_) => panic!("This gtfs file should have been rejected as invalid"),
        Err(error) => println!("correctly rejected invalid gtfs file, with error {}", error)
    };
}