use egui::{ColorImage, FontData, FontDefinitions, FontFamily, Frame, Image, Sense, Ui};
use image::ImageDecoder;
use log::info;
use plot_helper::data::sample::key::SerieKey;
use plot_helper::data::sample::Sample;
use plot_helper::data::sample_serie::memory_sample_serie::MemorySampleSerie;
use plot_helper::plotter::get_global_size;
use plot_helper::plotter::layout::Layout;
use plot_helper::plotter::scatter_plot::scatter_plot_with_backend;
use plotters::backend::{PixelFormat, RGBPixel};
use plotters::prelude::BitMapBackend;
use plotters_canvas::CanvasBackend;
use wasm_rs_dbg::dbg;

use image::codecs::png::PngDecoder;


use crate::commands::Commands;
#[cfg(target_arch = "wasm32")]
use crate::get_canvas;


/// The main application state
pub struct MyApp<S, K>
where
    S: Sample<K>,
    K: SerieKey,
{
    graph_texture: egui::TextureHandle,
    graph_canvas_id : String,
    graph_size : (usize, usize),
    graph_cached : Option<Vec<u8>>,

    data: MemorySampleSerie<S, K>,
    command: Commands<K>,
}

impl<S, K> MyApp<S, K>
where
    S: Sample<K>,
    K: SerieKey,
{
    /// Called once before the first frame.
    /// - graph_canvas_id : the id of the canvas where the graph will be drawn (the canvas must be in the html file, hidden)
    pub fn new(cc: &eframe::CreationContext<'_>, data : MemorySampleSerie<S, K>, graph_canvas_id : String) -> Self {
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
            graph_texture: cc.egui_ctx.load_texture(
                "graph_texture",
                egui::ColorImage::example(),
                egui::TextureOptions::NEAREST,
            ),
            data: data,
            graph_canvas_id,
            graph_size: (w as usize, h as usize),
            graph_cached: None,
            command: Commands::default(),
        }
    }


    fn draw_graph(&mut self) {
        // see https://github.com/bluurryy/noise-functions-demo/blob/e23b3eb6cb670412f0433fb06fcd9f97cc43e221/src/app.rs#L420
        let backend = CanvasBackend::new(&self.graph_canvas_id).unwrap();

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
        
        let canvas = get_canvas(&self.graph_canvas_id);
        let image_str = canvas.to_data_url().unwrap();
        dbg!("image_str : {}", &image_str);
        let image = Image::new(image_str);

        self.graph_texture.set(
            ColorImage::from_rgba_premultiplied(
                [self.graph_size.0, self.graph_size.1],
                &buffer,
            ),
            egui::TextureOptions::NEAREST,
        );
    }

    fn is_graph_drawn(&self) -> bool {
        self.graph_cached.is_some()
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
                    let size = self.graph_texture.size_vec2();
                    let sized_texture = egui::load::SizedTexture::new(self.graph_texture.id(), size);
                    ui.add(egui::Image::new(sized_texture).fit_to_exact_size(size));
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