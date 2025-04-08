use plot_helper::data::sample::key::SerieKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayCommands<K>
where
K: SerieKey,
{
    headers : Vec<K>,
}


impl <K> ArrayCommands<K>
where
    K: SerieKey,
{
    pub fn new(headers : Vec<K>) -> Self {
        Self { headers }
    }

    pub fn get_headers(&self) -> &Vec<K> {
        &self.headers
    }
}

impl<K> Default for ArrayCommands<K>
where
    K: SerieKey,
{
    fn default() -> Self {
        Self {
            headers: Vec::new(),
        }
    }
}