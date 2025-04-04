

use std::fs;
use std::path::Path;

use plot_helper::data::filtering::{Filter, Filters};
use plot_helper::data::sample_serie::file_sample_serie::FileSampleSerie;
use plot_helper::data::sample_serie::memory_sample_serie::MemorySampleSerie;
use plot_helper::plotter::layout::Layout;
#[cfg(all(not(target_arch = "wasm32")))]
use plot_helper::plotter::line_plot::line_plot;
#[cfg(all(not(target_arch = "wasm32")))]
use plot_helper::plotter::scatter_plot::scatter_plot;
use plot_helper::stat::stats_serie::MetricName;
use series::data::file_sample_test::FileTestSample;
use series::data::key::TestKey;



mod series;


const FILE_SAMPLES_DIR_PATH : &'static str = "tests/ressources/file_samples/";

const OUPUT_DIR_PATH : &'static str = "tests/ressources/output/";

#[test]
#[cfg(all(not(target_arch = "wasm32")))]
fn memory_sample_test() {
    use plot_helper::data::filtering::Operator;


    let output_scatter_file_path = Path::new(OUPUT_DIR_PATH).join("memory_sample_scatter_test.png");
    if output_scatter_file_path.exists() {
        fs::remove_file(&output_scatter_file_path).unwrap();
    }

    let output_line_file_path = Path::new(OUPUT_DIR_PATH).join("memory_sample_line_test.png");
    if output_line_file_path.exists() {
        fs::remove_file(&output_line_file_path).unwrap();
    }

    let output_filtered_line_file_path = Path::new(OUPUT_DIR_PATH).join("memory_sample_filtered_line_test.png");
    if output_filtered_line_file_path.exists() {
        fs::remove_file(&output_filtered_line_file_path).unwrap();
    }

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

    let plot: FileSampleSerie<FileTestSample, TestKey> = FileSampleSerie::new(file_paths);
    let plot: MemorySampleSerie<FileTestSample, TestKey> = plot.into();

    
    scatter_plot::<FileTestSample, TestKey, _>(
        &plot, 
        Some(TestKey::Test1Str), 
        output_scatter_file_path.as_os_str().to_str().unwrap(), 
        &Layout::new(2, 1), 
        vec![
            (TestKey::Test1Num, Some(TestKey::Test2Num), None),
            (TestKey::Test1Num, Some(TestKey::Test2Num), None)
        ], 
        false
    ).unwrap();

    assert!(output_scatter_file_path.exists());

    line_plot(
        &plot, 
        Some(TestKey::Test1Str), 
        output_line_file_path.as_os_str().to_str().unwrap(), 
        &Layout::new(2, 1), 
        vec![
            (TestKey::Test1Num, Some(TestKey::Test2Num), None),
            (TestKey::Test1Num, Some(TestKey::Test2Num), None)
        ], 
        false,
        MetricName::Additive
    ).unwrap();

    assert!(output_line_file_path.exists());

    let filter = Filters::new(vec![Filter::new_number(
        TestKey::Test2Num,
        Operator::LessThanOrEqual,
        1.0
    )]);

    scatter_plot::<FileTestSample, TestKey, _>(
        &plot, 
        Some(TestKey::Test1Str), 
        output_filtered_line_file_path.as_os_str().to_str().unwrap(), 
        &Layout::new(2, 1), 
        vec![
            (TestKey::Test1Num, Some(TestKey::Test2Num), None),
            (TestKey::Test1Num, Some(TestKey::Test2Num), Some(&filter))
        ], 
        false
    ).unwrap();

    assert!(output_filtered_line_file_path.exists());
}