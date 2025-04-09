use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod graph_app;
pub mod array_app;


pub trait HasCommands<C> : Sized
    where
        C: DeserializeOwned + Serialize + Default + Clone + Send + Sync
{
    fn get_commands(&self) -> &C;

    fn get_app_storage_key() -> String;

    fn from_commands(commands: C) -> Self;

    fn load_from_context(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        
        let command = if let Some(storage) = cc.storage {
            eframe::get_value(storage, &Self::get_app_storage_key()).unwrap_or_else(|| C::default())
        } else {
            C::default()
        };

        Self::from_commands(command)
    }        
}