use key::SerieKey;

pub mod key;


/// Define a sample linked to a key
pub trait Sample<Key>
    where 
        Self : Clone + Sized + Send + Sync,
        Key : SerieKey
{

    /// Get the value of data (as f32), associated to the given key
    fn get_numeric_value(&self, key : &Key) -> f32;

    /// Get the value of data (as string), associated to the given key
    fn get_string_value(&self, key : &Key) -> String;

    /// Load a sample from a file path
    fn new_from_file_path(file_path : &str) -> Result<Vec<Self>, Box<dyn std::error::Error>>;

}