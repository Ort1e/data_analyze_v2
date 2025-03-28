use super::key::SerieKey;
use super::Sample;




/// Define a sample linked to a key from a file
pub trait FileSample<K> : Sample<K>
where 
    Self : Clone + Sized + Send + Sync,
    K : SerieKey,
{
    /// Load samples from a file path
    fn new_from_file_path(file_path : &str) -> Result<Vec<Self>, Box<dyn std::error::Error>>;
}