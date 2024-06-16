# GTFS Tools
This is the attempt to build up a toolset to parse, query, and edit gtfs datasets.

## Installation / Dev-Environment
Currently this is a personal project, it has a nix dev environment defined in flake.nix. And requires a rust 1.77.0 toolchain.

## Usage
Currently functionality is limited. It can check if all required files according to gtfs spec are present in an Archive, and it can parse all required files of a GTFS dataset, and some optional ones too.
```console
$ cargo run -- echo <gtfs.zip> stops
...
Stops { stop_id: "de:08125:9427:0:binsw", stop_code: None, stop_name: Some("Neckarsulm, Binswang. Str. West"), tts_stop_name: None, stop_desc: Some("Neckarsulm, Binswang. Str. West"), stop_lat: Some(49.188502), stop_lon: Some(9.230585), zone_id: None, stop_url: None, location_type: Some(Stop), parent_station: None, stop_timezone: None, wheelchair_boarding: Some(Unknown), level_id: Some("2"), platform_code: Some("binsw") }
...
```
The only operation it can perform so far is to reformat stops into `GeoJSON` format:
```console
$ cargo run -- geo-json <gtfs.zip> <output>.json stops
```

## TODOs
- [x] basic serde frame work to deserialise gtfs records
- [x] add all required tables
- [ ] Validation
    - [x] Check file presence
    - [ ] Internal Validation and Error Handling 
- [ ] add all optional tables
- [ ] geojson outputs
    - [x] stops to geojson conversion
    - [ ] other data
- [ ] queries
- [ ] file output
    - [ ] cropping datasets
        - [ ] by time
        - [ ] by extent