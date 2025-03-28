use log::info;
use plot_helper::data::sample::key::SerieKey;
use plot_helper::data::sample::Sample;
use plot_helper::data::sample_serie::memory_sample_serie::MemorySampleSerie;

use crate::commands::Commands;



pub struct MyApp<S, K>
where
    S: Sample<K>,
    K: SerieKey,
{
    graph : Option<egui::TextureHandle>,
    data: MemorySampleSerie<S, K>,
    command: Commands<K>,
}

impl<S, K> MyApp<S, K>
where
    S: Sample<K>,
    K: SerieKey,
{
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, data : MemorySampleSerie<S, K>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
 
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /*if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_else(|| Self { data: data });
        }*/

        Self { 
            data: data,
            graph: None,
            command: Commands::default(),
        }
    }
}



// ---------------------------------- drawing -------------------------------

impl<S, K> eframe::App for MyApp<S, K>
where 
    S: Sample<K>,
    K: SerieKey,
{
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        info!("no Saving state");
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            
            // "number of sample" at the top of the screen
            ui.vertical(|ui| {
                let has_changed_label = if self.command.is_changed() { "Changed" } else { "Not changed" };

                ui.label(format!("Number of samples : {} ({})", self.data.nb_samples(), has_changed_label));
                // ------------------ toolbar ------------------
                self.command = self.command.display_in_ui(ui);
            });


        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Graph :");
            match &self.graph {
                Some(graph) => {
                    
                },
                None => {
                    ui.label("No graph to display");
                }
            }
           
        });


        // Draw the graph
        if self.command.is_changed() {
            
            // see https://github.com/bluurryy/noise-functions-demo/blob/e23b3eb6cb670412f0433fb06fcd9f97cc43e221/src/app.rs#L420
        }
    }
}

pub fn get_str_from_opt_key<K>(key: &Option<K>) -> String
where
    K: SerieKey,
{
    match key {
        Some(k) => format!("{}", k),
        None => "None".to_string()
    }
}