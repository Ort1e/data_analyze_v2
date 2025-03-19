use std::fs;
use std::path::Path;

use plot_helper::data::sample_serie::file_sample_serie::FileSampleSerie;
use plot_helper::plotter::layout::Layout;
use plot_helper::plotter::scatter_plot::scatter_plot;
use series::data::file_sample_test::FileTestSample;
use series::data::key::TestKey;



mod series;


const FILE_SAMPLES_DIR_PATH : &'static str = "tests/ressources/file_samples/";

const OUPUT_DIR_PATH : &'static str = "tests/ressources/output/";

#[test]
fn memory_sample_test() {

    let output_file_path = Path::new(OUPUT_DIR_PATH).join("memory_sample_test.png");

    // get all the json files in the directory
    let entries = fs::read_dir(FILE_SAMPLES_DIR_PATH).unwrap();
    let mut file_paths = Vec::new();
    for entry in entries {
        let entry = entry.unwrap();
        let file_path = entry.path();
        if file_path.is_file() {
            file_paths.push(file_path.to_str().unwrap().to_string());
        }
    }

    let mut plot: FileSampleSerie<FileTestSample, TestKey> = FileSampleSerie::new(file_paths);

    
    scatter_plot(
        &mut plot, 
        Some(TestKey::Test1Str), 
        output_file_path.as_os_str().to_str().unwrap(), 
        &Layout::new(2, 1), 
        vec![
            (TestKey::Test1Num, Some(TestKey::Test2Num), None),
            (TestKey::Test1Num, Some(TestKey::Test2Num), None)
        ], 
        false
    ).unwrap();
}