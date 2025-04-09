use array_command::ArrayCommands;
use egui::Ui;
use plot_helper::data::sample::key::SerieKey;
use plot_helper::data::sample::Sample;
use plot_helper::data::sample_serie::memory_sample_serie::MemorySampleSerie;
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
    pub fn draw_ui<S>(&mut self, ui : &mut Ui, data : &MemorySampleSerie<S, K>) 
    where 
        S: Sample<K>,
    {
        
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