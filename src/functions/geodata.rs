use serde::Serialize;

use super::gtfs::{LocationType, Stops, WheelchairAccessibility};

#[derive(Serialize, Debug, PartialEq)]
pub struct StopsJson {
    pub id: String,
    pub stop_code: Option<String>,
    pub stop_name: Option<String>,
    pub tts_stop_name: Option<String>,
    pub stop_desc: Option<String>,
    #[serde(serialize_with = "geojson::ser::serialize_geometry")]
    pub geometry: geo_types::Point,
    pub zone_id: Option<String>,
    pub stop_url: Option<String>,
    pub location_type: Option<LocationType>,
    pub parent_station: Option<String>,
    pub stop_timezone: Option<String>,
    pub wheelchair_boarding: Option<bool>,
    pub level_id: Option<String>,
    pub platform_code: Option<String>,
}

pub fn from_stop(stop: Stops) -> Option<StopsJson> {
    if let Some(x_coord) = stop.stop_lon {
        if let Some(y_coord) = stop.stop_lat {
            let wheelchair_access: Option<bool>;
            match stop.wheelchair_boarding {
                None => wheelchair_access = None,
                Some(WheelchairAccessibility::Unknown) => wheelchair_access = None,
                Some(WheelchairAccessibility::No) => wheelchair_access = Some(false),
                Some(WheelchairAccessibility::Yes) => wheelchair_access = Some(true)
            }
            
            Some(
                StopsJson {
                    id: stop.stop_id,
                    stop_code: stop.stop_code,
                    stop_name: stop.stop_name,
                    tts_stop_name: stop.tts_stop_name,
                    stop_desc: stop.stop_desc,
                    geometry: geo_types::Point::new(x_coord, y_coord),
                    zone_id: stop.zone_id,
                    stop_url: stop.stop_url,
                    location_type: stop.location_type,
                    parent_station: stop.parent_station,
                    stop_timezone: stop.stop_timezone,
                    wheelchair_boarding: wheelchair_access,
                    level_id: stop.level_id,
                    platform_code: stop.platform_code
                }
            )
        } else {
            None
        }
    } else {
        None
    }

}

#[test]
fn test_from_stop() {
    let stop = Stops{
        stop_id: String::from("test"),
        stop_code: None,
        stop_name: Some(String::from("test")),
        tts_stop_name: None,
        stop_desc: None,
        stop_lat: Some(8.0),
        stop_lon: Some(48.23),
        zone_id: None,
        stop_url: None,
        location_type: Some(LocationType::Station),
        parent_station: Some(String::from("Test")),
        stop_timezone: None,
        wheelchair_boarding: Some(WheelchairAccessibility::Unknown),
        level_id: None,
        platform_code: None
    };

    let expected_stop = StopsJson{
        id: String::from("test"),
        stop_code: None,
        stop_name: Some(String::from("test")),
        tts_stop_name: None,
        stop_desc: None,
        geometry: geo_types::Point::new(48.23,8.0),
        zone_id: None,
        stop_url: None,
        location_type: Some(LocationType::Station),
        parent_station: Some(String::from("Test")),
        stop_timezone: None,
        wheelchair_boarding: None,
        level_id: None,
        platform_code: None
    };

    let result = from_stop(stop).unwrap();
    assert_eq!(result, expected_stop)
}