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
