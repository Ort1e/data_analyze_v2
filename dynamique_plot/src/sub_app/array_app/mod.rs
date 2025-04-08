use array_command::ArrayCommands;
use plot_helper::data::sample::key::SerieKey;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::HasCommands;

pub mod array_command;


#[derive(Debug, Clone)]
pub struct ArrayApp<K>
where
    K: SerieKey,
{
    array_commands: ArrayCommands<K>,
}

impl<K> ArrayApp<K>
where
    K: SerieKey,
{
    
    pub fn get_commands(&self) -> &ArrayCommands<K> {
        &self.array_commands
    }
}

impl<K> HasCommands<ArrayCommands<K>> for ArrayApp<K>
where
    K: SerieKey + Serialize + DeserializeOwned,
{
    fn get_commands(&self) -> &ArrayCommands<K> {
        &self.array_commands
    }
    
    fn get_app_storage_key() -> String {
        "array_app".to_string()
    }
    
    fn from_commands(commands: ArrayCommands<K>) -> Self {
        Self {
            array_commands: commands,
        }
    }
}