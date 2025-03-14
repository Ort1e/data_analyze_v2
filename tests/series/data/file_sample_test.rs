use plot_helper::data::sample::file_sample::FileSample;
use plot_helper::data::sample::Sample;
use serde_derive::{Deserialize, Serialize};

use super::key::TestKey;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileTestSample {
    test1_num: f32,
    test1_str: String,
    test2_num: f32,
    test2_str: String,
}

impl Sample<TestKey> for FileTestSample {
    fn get_numeric_value(&self, key : &TestKey) -> f32 {
        match key {
            TestKey::Test1Num => self.test1_num,
            TestKey::Test1Str => unreachable!("test 1 str is not numeric"),
            TestKey::Test2Num => self.test2_num,
            TestKey::Test2Str => unreachable!("test 2 str is not numeric"),
        }
    }

    fn get_string_value(&self, key : &TestKey) -> String {
        match key {
            TestKey::Test1Num => unreachable!("test 1 num is not string"),
            TestKey::Test1Str => self.test1_str.clone(),
            TestKey::Test2Num => unreachable!("test 2 num is not string"),
            TestKey::Test2Str => self.test2_str.clone(),
        }
    }

    
}

impl FileSample<TestKey> for FileTestSample {    
    fn new_from_file_path(file_path : &str) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(file_path)?;
        let res : Self = serde_json::from_str(data.as_str())?;
        Ok(vec![res])
    }
}
