use std::fmt::Display;
use egui::{FontData, FontDefinitions, FontFamily};
use log::info;
use plot_helper::data::sample::key::SerieKey;
use plot_helper::data::sample::Sample;
use plot_helper::data::sample_serie::memory_sample_serie::MemorySampleSerie;
use serde::de::DeserializeOwned;
use serde::Serialize;



use crate::sub_app::array_app::ArrayApp;
use crate::sub_app::graph_app::GraphApp;
use crate::sub_app::HasCommands;




/// The main application state
pub struct MyApp<S, K>
where
    S: Sample<K>,
    K: SerieKey + DeserializeOwned + Serialize,
{
    sample : MemorySampleSerie<S, K>,
    graph_app : GraphApp<K>,
    array_app : ArrayApp<K>,
}

impl<S, K> MyApp<S, K>
where
    S: Sample<K>,
    K: SerieKey + DeserializeOwned + Serialize,
{
    /// Called once before the first frame.
    /// - graph_canvas_id : the id of the canvas where the graph will be drawn (the canvas must be in the html file, hidden)
    pub fn new(cc: &eframe::CreationContext<'_>, data : MemorySampleSerie<S, K>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
 
        

        // Set up your own fonts:
        {
            let mut fonts = FontDefinitions::default();

            // Install my own font (maybe supporting non-latin characters):
            fonts.font_data.insert("ARIAL".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                FontData::from_static(include_bytes!("../../ressources/ARIAL.TTF"))
                )
            );

            // Put my font first (highest priority):
            fonts.families.insert(FontFamily::Name("sans-serif".into()), vec!["ARIAL".to_owned()]);
            //fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "ARIAL".to_owned());

            cc.egui_ctx.set_fonts(fonts);
        }

        egui_extras::install_image_loaders(&cc.egui_ctx);


        Self {
            sample : data,
            graph_app: GraphApp::load_from_context(cc),
            array_app: ArrayApp::load_from_context(cc),
        }
    }
}



// ---------------------------------- drawing -------------------------------

impl<S, K> eframe::App for MyApp<S, K>
where 
    S: Sample<K>,
    K: SerieKey + DeserializeOwned + Serialize,
{
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        info!("Saving state");
        eframe::set_value(storage, &GraphApp::<K>::get_app_storage_key(), &self.graph_app.get_command());
        eframe::set_value(storage, &ArrayApp::<K>::get_app_storage_key(), &self.array_app.get_commands());
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            
            self.graph_app.draw_ui(ui, &self.sample);
            
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            
        });

        
    }

    
}

pub fn get_str_from_opt_key<K>(key: &Option<K>) -> String
where
    K: Display,
{
    match key {
        Some(k) => format!("{}", k),
        None => "None".to_string()
    }
}
