use egui::{ColorImage, FontData, FontDefinitions, FontFamily};
use log::info;
use plot_helper::data::sample::key::SerieKey;
use plot_helper::data::sample::Sample;
use plot_helper::data::sample_serie::memory_sample_serie::MemorySampleSerie;
use plot_helper::plotter::get_global_size;
use plot_helper::plotter::layout::Layout;
use plot_helper::plotter::scatter_plot::scatter_plot_with_backend;
use plotters::backend::{PixelFormat, RGBPixel};
use plotters::prelude::BitMapBackend;

use crate::commands::Commands;



pub struct MyApp<S, K>
where
    S: Sample<K>,
    K: SerieKey,
{
    graph : egui::TextureHandle,
    graph_size : (usize, usize),
    pixels_cached : Option<Vec<u8>>,

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

        

        let (w, h) = get_global_size(&Layout::new(1, 1));

        Self { 
            data: data,
            graph: cc.egui_ctx.load_texture(
                "graph",
                egui::ColorImage::example(),
                egui::TextureOptions::NEAREST,
            ),
            graph_size: (w as usize, h as usize),
            pixels_cached: None,
            command: Commands::default(),
        }
    }


    fn draw_graph(&mut self) {
        // see https://github.com/bluurryy/noise-functions-demo/blob/e23b3eb6cb670412f0433fb06fcd9f97cc43e221/src/app.rs#L420
        let mut buffer = vec![0; self.graph_size.0 * self.graph_size.1 * RGBPixel::PIXEL_SIZE];

        let backend = BitMapBackend::with_buffer(
            &mut buffer,
            (self.graph_size.0 as u32, self.graph_size.1 as u32),
        );


        scatter_plot_with_backend(
            &self.data, 
            None, 
            backend, 
            &Layout::new(1, 1),
            vec![
                (self.command.get_x_axis().unwrap(), self.command.get_y_axis(), None)
            ], 
            false
        ).expect("Error while plotting the graph");

        let texture = &mut self.graph;

        texture.set(
            ColorImage::from_rgba_premultiplied(
                [self.graph_size.0, self.graph_size.1],
                &buffer,
            ),
            egui::TextureOptions::NEAREST,
        );
        
        self.pixels_cached = Some(buffer);

    }

    fn is_graph_drawn(&self) -> bool {
        self.pixels_cached.is_some()
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
                ui.label(format!("Number of samples : {}", self.data.nb_samples()));
                // ------------------ toolbar ------------------
                self.command.display_in_ui(ui);
            });


        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Graph :");
            ui.vertical(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui.button("Draw graph").clicked() {
                        self.draw_graph();
                    }
                });

                ui.separator();

                if self.is_graph_drawn() {

                } else {
                    ui.label("No graph to display");
                }

                
            });
        });
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