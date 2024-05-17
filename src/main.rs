use std::fs::File;

pub use zip::read::ZipArchive;

pub struct GtfsFile {
    archive: ZipArchive<File>
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

fn main() {
    let gtfs_path: String = String::from("/home/lotte/code/github.com/chwiggy/gtfs-tooling/test_data/20240513_fahrplaene_gesamtdeutschland_gtfs.zip");

    let gtfs_file: GtfsFile = GtfsFile::new(&gtfs_path);
    let file_list: Vec<String> = gtfs_file.list_files();

    println!("GTFS archive contains: {:?}", file_list);


}

// TODO Reading in GTFS File
// TODO option: clipping by bounds
// TODO option: filtering by various categories
// TODO option: clipping by time

// TODO think about visualisations