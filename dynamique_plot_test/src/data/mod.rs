use plot_helper::data::sample::Sample;
use serde::{Deserialize, Serialize};

pub mod key;

const FILE_DATA_JSON: &'static str = include_str!("../../ressources/data.json");

#[derive(Clone, Serialize, Deserialize)]
pub struct FileInfo {
    language: String,
    nb_line: u32,
    nb_char: u32,
    file_name: String,
}

impl FileInfo {
    pub fn new() -> Vec<Self> {
        serde_json::from_str(FILE_DATA_JSON).unwrap()
    }
}

impl Sample<key::FileKey> for FileInfo {
    
    fn get_numeric_value(&self, key : &key::FileKey) -> f32 {
        match key {
            key::FileKey::NbLine => self.nb_line as f32,
            key::FileKey::NbChar => self.nb_char as f32,
            _ => panic!("The key {:?} is not numeric", key)
        }
    }
    
    fn get_string_value(&self, key : &key::FileKey) -> String {
        match key {
            key::FileKey::Language => self.language.clone(),
            key::FileKey::FileName => self.file_name.clone(),
            _ => panic!("The key {:?} is not a string", key)
        }
    }
}