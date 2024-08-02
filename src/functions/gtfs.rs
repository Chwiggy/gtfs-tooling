use core::{fmt, panic};
use std::fmt::Debug;
use std::path::PathBuf;
use std::{fs::File, usize};

use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{serde_as,BoolFromInt};

use chrono::NaiveDate;

use csv;

pub use zip::read::ZipArchive;
use zip::result::ZipError;

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
        let file_result = self.archive.by_name(T::FILE);
        match file_result {
            Ok(file) => {
                let reader = csv::Reader::from_reader(file);
                reader.into_deserialize::<T>()
            },
            Err(ZipError::FileNotFound) => {
                if !T::REQUIRED {
                    panic!("The file requested is optional and missing")
                } else {
                    panic!("A required file seems to be missing")
                }
            },
            Err(error) => panic!("{}", error)
        }

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

    pub fn to_stdout<T>(&mut self)
    where
        T: for <'a> GtfsObject + for <'de> serde::Deserialize<'de> + for <'a> Debug
    {
        let entries: Iter::<T> = self.into_iter();
        for entry in entries {
            println!("{:?}", entry.unwrap())
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
}

pub trait GtfsObject {
    const FILE: &'static str;
    const REQUIRED: bool;
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
    const REQUIRED: bool = true;
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Stop {
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

impl GtfsObject for Stop {
    const FILE: &'static str = "stops.txt";
    const REQUIRED: bool = true;
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
    const REQUIRED: bool = true;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RouteType {
    Standard(StandardRouteType),
    HVT(HVTRouteType)
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StandardRouteType {
    Tram = 0,
    Metro = 1,
    Rail = 2,
    Bus = 3,
    Ferry = 4,
    CableTram = 5,
    AerialLift = 6,
    Funicular = 7,
    TrolleyBus = 11,
    Monorail = 12,
}


#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum HVTRouteType {    
    /*
    The following route types are an extension ba-ed on Hierarchical Vehicle Types
    see https://developers.google.com/transit/gtfs/reference/extended-route-types
    Excluding the unsupported ones
    */
    Rail = 100,
    HighSpeedRail = 101,
    LongDistanceRail = 102,
    InterRegionalRail = 103,
    SleeperRail = 105,
    RegionalRail = 106,
    TouristRail = 107,
    RailShuttle = 108,
    SuburbanRail = 109,
    Coach = 200,
    IntrenationalCoach = 201,
    NationalCoach = 202,
    RegionalCoach = 204,
    UrbanRail0 = 400,
    Metro = 401,
    UndergroundRail = 402,
    UrbanRail1 = 403,
    MonoRail = 405,
    Bus = 700,
    RegionalBus = 701,
    ExpressBus = 702,
    LocalBus = 704,
    DemandResponseBus = 715,
    TrolleyBus = 800,
    Tram = 900,
    WaterTransport = 1000,
    Ferry = 1200,
    AerialLift = 1300,
    Telecabin = 1301,
    Funicular = 1400,
    CommunalTaxi = 1501,
    Miscellaneous = 1700,
    HorseDrawnCarriage = 1702
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
    const REQUIRED: bool = true;
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

// TODO figure out how to account for different time semantics in stop_times and timeframes

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
    const REQUIRED: bool = true;
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
    const REQUIRED: bool = false;
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
pub struct CalendarDate {
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

impl GtfsObject for CalendarDate {
    const FILE: &'static str = "calendar_dates.txt";
    const REQUIRED: bool = false;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FareAttributes {
    pub fare_id: String,
    pub price: f64,
    pub currency_type: String,
    pub payment_method: PaymentMethod,
    // This being a none implies unlimited transfers, not none
    pub transfers: Option<TransferCount>,
    pub agency_id: Option<String>,
    pub transfer_duration: Option<u64>
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PaymentMethod {
    // To accommodate the sample gtfs feed, this enum include a meaningless zero
    WhatEverTheFuck = 0,
    OnBoard = 1,
    BeforeBoarding = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TransferCount {
    None = 0,
    One = 1,
    Two = 2,
}

impl GtfsObject for FareAttributes {
    const FILE: &'static str = "fare_attributes.txt";
    const REQUIRED: bool = false;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FareRule {
    fare_id: String,
    route_id: Option<String>,
    origin_id: Option<String>,
    destination_id: Option<String>,
    contains_id: Option<String>,
}

impl GtfsObject for FareRule {
    const FILE: &'static str = "fare_rules.txt";
    const REQUIRED: bool = false;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Timeframe {
    pub timeframe_group_id: String,
    pub start_time: Option<Time>,
    pub end_time: Option<Time>,
    pub service_id: String,
}

impl GtfsObject for Timeframe {
    const FILE: &'static str = "timeframes.txt";
    const REQUIRED: bool = false;
}


#[derive(Debug, Deserialize, Serialize)]
pub struct FareMedium {
    pub fare_media_id: String,
    pub fare_media_name: Option<String>,
    pub fare_media_type: FareMediaType,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FareMediaType {
    None = 0,
    Paper = 1,
    Card = 2,
    Cemv = 3,
    App = 4,
}

impl GtfsObject for FareMedium {
    const FILE: &'static str = "fare-media.txt";
    const REQUIRED: bool = false;
}


#[derive(Debug, Deserialize, Serialize)]
pub struct FareProduct {
    pub fare_product_id: String,
    pub fare_product_name: Option<String>,
    pub fare_media_id: Option<String>,
    pub amount: f64,
    pub currency: String,
}
impl GtfsObject for FareProduct {
    const FILE: &'static str = "fare_products.txt";
    const REQUIRED: bool = false;
}


#[derive(Debug, Deserialize, Serialize)]
pub struct FareLegRule {
    pub leg_group_id: Option<String>,
    pub network_id: Option<String>,
    pub from_area_id: Option<String>,
    pub to_area_id: Option<String>,
    pub from_timeframe_group_id: Option<String>,
    pub to_timeframe_group_id: Option<String>,
    pub fare_product_id: String,
    pub rule_priority: Option<u64>
}

impl GtfsObject for FareLegRule {
    const FILE: &'static str = "fare_leg_rules.txt";
    const REQUIRED: bool = false;
}




#[derive(Debug, Deserialize, Serialize)]
pub struct FareTransferRule {
    pub from_leg_group_id: Option<String>,
    pub to_leg_group_id: Option<String>,
    pub transfer_count: Option<i64>,
    pub duration_limit: Option<u64>,
    pub duration_limit_type: Option<DurationLimitType>,
    pub fare_transfer_type: FareTransferType,
    pub fare_product_id: Option<String>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DurationLimitType {
    DepartureAndArrival = 0,
    DepartureAndDeparture = 1,
    ArrivalAndDeparture = 2,
    ArrivalAndArrival = 3,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FareTransferType {
    // The fuck does this mean
    AAB = 0,
    AABB = 1,
    AB = 2
}

impl GtfsObject for FareTransferRule {
    const FILE: &'static str = "fare_transfer_rules.txt";
    const REQUIRED: bool = false;
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Area {
    pub area_id: String,
    pub area_name: Option<String>,
}

impl GtfsObject for Area {
    const FILE: &'static str = "areas.txt";
    const REQUIRED: bool = false;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StopArea {
    pub area_id: String,
    pub stop_id: String,
}

impl GtfsObject for StopArea {
    const FILE: &'static str = "stop_areas.txt";
    const REQUIRED: bool = false;
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Network {
    pub network_id: String,
    pub network_name: Option<String>
}

impl GtfsObject for Network {
    const FILE: &'static str = "networks.txt";
    const REQUIRED: bool = false;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RouteNetwork {
    pub network_id: String,
    pub route_id: String,
}

impl GtfsObject for RouteNetwork {
    const FILE: &'static str = "route_networks.txt";
    const REQUIRED: bool = false;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Shape {
    pub shape_id: String,
    pub shape_pt_lat: f64,
    pub shape_pt_lon: f64,
    pub shape_pt_sequence: u64,
    pub shape_dist_travelled: Option<f64>,
}

impl GtfsObject for Shape {
    const FILE: &'static str = "shapes.txt";
    const REQUIRED: bool = false;
}


#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Frequency {
    pub trip_id: String,
    pub start_time: Time,
    pub end_time: Time,
    pub headway_secs: u64,
    #[serde_as(as = "Option<BoolFromInt>")]
    pub exact_times: Option<bool>,
}

impl GtfsObject for Frequency {
    const FILE: &'static str = "frequencies.txt";
    const REQUIRED: bool = false;
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Transfer {
    pub from_stop_id: Option<String>,
    pub to_stop_id: Option<String>,
    pub from_route_id: Option<String>,
    pub to_route_id: Option<String>,
    pub from_trip_id: Option<String>,
    pub to_trip_id: Option<String>,
    pub transfer_type: TransferType,
    pub min_transfer_time: Option<u64>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TransferType {
    Recommended = 0,
    Timed = 1,
    MinTime = 2,
    NotPossible = 3,
    InSeat = 4,
    Reboard = 5,
}

impl GtfsObject for Transfer {
    const FILE: &'static str = "transfers.txt";
    const REQUIRED: bool = false;
}


#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Pathway {
    pub pathway_id: String,
    pub from_stop_id: String,
    pub to_stop_id: String,
    pub pathway_mode: PathwayMode,
    #[serde_as(as = "BoolFromInt")]
    pub is_bidirectional: bool,
    pub length: Option<f64>,
    pub traversal_time: Option<u64>,
    pub stair_count: Option<i64>,
    pub max_slope: Option<f64>,
    pub min_width: Option<f64>,
    pub signposted_as: Option<String>,
    pub reversed_signposted_as: Option<String>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PathwayMode {
    Walkway = 1,
    Stairs = 2,
    MovingSidewalk = 3,
    Escalator = 4,
    Elevator = 5,
    FareGate = 6,
    ExitGate = 7,
}

impl GtfsObject for Pathway {
    const FILE: &'static str = "pathways.txt";
    const REQUIRED: bool = false;
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Level {
    pub level_id: String,
    pub level_index: f64,
    pub level_name: Option<String>,
}
impl GtfsObject for Level {
    const FILE: &'static str = "levels.txt";
    const REQUIRED: bool = false;
}


#[derive(Debug, Deserialize, Serialize)]
pub struct LocationGroup {
    pub location_group_id: String,
    pub location_group_name: Option<String>
}

impl GtfsObject for LocationGroup {
    const FILE: &'static str = "location_group.txt";
    const REQUIRED: bool = false;
}


#[derive(Debug, Deserialize, Serialize)]
pub struct LocationGroupStop {
    pub location_group_id: String,
    pub stop_id: String
}

impl GtfsObject for LocationGroupStop {
    const FILE: &'static str = "location_group_stops.txt";
    const REQUIRED: bool = false;
}

// TODO deal with locations.json
// pub struct Locations {
//     pub id: String,

// }

#[derive(Debug, Deserialize, Serialize)]
pub struct BookingRule {
    pub booking_rule_id: String,
    pub booking_type: BookingType,
    pub prior_notice_duration_min: Option<i64>,
    pub prior_notice_duration_max: Option<i64>,
    pub prior_notice_last_day: Option<i64>,
    pub prior_notice_last_time: Option<Time>,
    pub prior_notice_start_day: Option<i64>,
    pub prior_notice_start_time: Option<Time>,
    pub prior_notice_service_id: Option<String>,
    pub message: Option<String>,
    pub pickup_message: Option<String>,
    pub drop_off_message: Option<String>,
    pub phone_number: Option<String>,
    pub info_url: Option<String>,
    pub booking_url: Option<String>
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BookingType {
    RealTime = 0,
    SameDay = 1,
    PriorDay = 2,
}

impl GtfsObject for BookingRule {
    const FILE: &'static str = "booking_rules.txt";
    const REQUIRED: bool = false;
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Translation {
    // This enum is confusingly documented:
    pub table_name: String,
    pub field_name: String,
    pub language: String,
    pub translation: String,
    pub record_id: Option<String>,
    pub record_sub_id: Option<String>,
    pub field_value: Option<String>,
}

impl GtfsObject for Translation {
    const FILE: &'static str = "Translations.txt";
    const REQUIRED: bool = false;
}


#[derive(Debug, Deserialize, Serialize)]
pub struct FeedInfo {
    pub feed_publisher_name: String,
    pub feed_publisher_url: String,
    pub feed_lang: String,
    pub default_lang: Option<String>,
    #[serde(default)]
    #[serde(with = "opt_date")]
    pub feed_start_date: Option<NaiveDate>,
    #[serde(default)]
    #[serde(with = "opt_date")]
    pub feed_end_date: Option<NaiveDate>,
    pub feed_version: Option<String>,
    pub feed_contact_email: Option<String>,
    pub feed_contact_url: Option<String>,
}

mod opt_date {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y%m%d";

    pub fn serialize<S>(dt: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match dt {
            Some(dt) => serializer.serialize_str(&format!("{}", dt.format(FORMAT))),
            None => serializer.serialize_str(""),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => Ok(Some(
                NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?,
            )),
            None => Ok(None),
        }
    }
}

impl GtfsObject for FeedInfo {
    const FILE: &'static str = "feed_info.txt";
    const REQUIRED: bool = false;
}


#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Attributions {
    pub attribution_id: Option<String>,
    pub agency_id: Option<String>,
    pub route_id: Option<String>,
    pub trip_id: Option<String>,
    pub organization_name: String,
    #[serde_as(as = "Option<BoolFromInt>")]
    pub is_producer: Option<bool>,
    #[serde_as(as = "Option<BoolFromInt>")]
    pub is_operator: Option<bool>,
    #[serde_as(as = "Option<BoolFromInt>")]
    pub is_authority: Option<bool>,
    pub attribution_url: Option<String>,
    pub attribution_email: Option<String>,
    pub attribution_phone: Option<String>,
}

impl GtfsObject for Attributions {
    const FILE: &'static str = "attributions.txt";
    const REQUIRED: bool = false;
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