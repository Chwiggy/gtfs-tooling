use std::fs::File;

pub use zip::read::ZipArchive;

pub struct GtfsFile {
    pub(crate) archive: ZipArchive<File>
}

impl GtfsFile {
    pub fn new(filepath: &String) -> Self {
        let reader: File = std::fs::File::open(filepath).unwrap();
        let archive: ZipArchive<File> = ZipArchive::new(reader).unwrap();
        GtfsFile { archive }
    }

    pub fn list_files(mut self) -> Vec<String> {
    
        let mut files: Vec<String> = vec![];
    
        for i in 0..self.archive.len() {
            let file: zip::read::ZipFile = self.archive.by_index(i).unwrap();
            let file_name: String = String::from(file.name());
            files.push(file_name);
        }
    
        files
    }
}

#[test]
fn test_new_gtfsfile() {
    let data: Vec<&str> = vec!["agency.txt", "calendar.txt", "calendar_dates.txt", "feed_info.txt", "frequencies.txt", "levels.txt", "pathways.txt", "routes.txt", "shapes.txt", "stop_times.txt", "stops.txt", "transfers.txt", "trips.txt", "Beschreibung_DELFI-Datensatz_GTFS_20240513.pdf"];
    let mut expected: Vec<String> = Vec::new();
    for file in data {
        expected.push(String::from(file));
    }

    let path: String = String::from("test_data/20240513_fahrplaene_gesamtdeutschland_gtfs.zip");
    let result: Vec<String> = GtfsFile::new(&path).list_files();

    assert_eq!(expected, result);
}
