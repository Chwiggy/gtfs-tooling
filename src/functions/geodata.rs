use serde::Serialize;

use super::gtfs::{LocationType, Shape, Stops, WheelchairAccessibility};

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
    pub wheelchair_boarding: Option<WheelchairAccessibility>,
    pub level_id: Option<String>,
    pub platform_code: Option<String>,
}

impl StopsJson {
    pub fn from_stop(stop: Stops) -> Option<StopsJson> {
        if let Some(x_coord) = stop.stop_lon {
            if let Some(y_coord) = stop.stop_lat {
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
                        wheelchair_boarding: stop.wheelchair_boarding,
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
}


#[derive(Serialize, Debug, PartialEq)]
pub struct GeoShapePoint {
    pub shape_id: String,
    #[serde(serialize_with = "geojson::ser::serialize_geometry")]
    pub shape_point: geo_types::Point,
    pub shape_pt_sequence: u64,
    pub shape_dist_travelled: Option<f64>,
}

impl GeoShapePoint {
    pub fn from_shape(shape: Shape) -> GeoShapePoint {
        GeoShapePoint {
            shape_id: shape.shape_id,
            shape_point: geo_types::Point::new(shape.shape_pt_lon, shape.shape_pt_lat),
            shape_pt_sequence: shape.shape_pt_sequence,
            shape_dist_travelled: shape.shape_dist_travelled
        }
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct GeoShapeLine {
    pub shape_id: String,
    #[serde(serialize_with = "geojson::ser::serialize_geometry")]
    pub shape_line: geo_types::LineString
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
        wheelchair_boarding: Some(WheelchairAccessibility::Unknown),
        level_id: None,
        platform_code: None
    };

    let result: StopsJson = StopsJson::from_stop(stop).unwrap();
    assert_eq!(result, expected_stop)
}