use std::fmt::Display;

use egui::{FontData, FontDefinitions, FontFamily};
use log::info;
use plot_helper::data::sample::key::SerieKey;
use plot_helper::data::sample::Sample;
use plot_helper::data::sample_serie::memory_sample_serie::MemorySampleSerie;
use plot_helper::plotter::get_global_size;
use plot_helper::plotter::layout::Layout;



use crate::commands::graph_commands::GraphCommands;
use crate::toggle_ui;

#[cfg(target_arch = "wasm32")]
use crate::{get_canvas, create_canvas, update_canvas_style, remove_canvas};


pub const GRAPH_IMAGE_TYPE: &str = "png";
pub const GRAPH_CANVAS_ID: &str = "graph_canvas";

/// The main application state
pub struct MyApp<S, K>
where
    S: Sample<K>,
    K: SerieKey,
{
    graph_size : (usize, usize),
    graph_cached : Option<Vec<u8>>,
    
    drawn_error : Option<String>,
    automatic_redraw: bool,

    data: MemorySampleSerie<S, K>,
    command: GraphCommands<K>,
}

impl<S, K> MyApp<S, K>
where
    S: Sample<K>,
    K: SerieKey,
{
    /// Called once before the first frame.
    /// - graph_canvas_id : the id of the canvas where the graph will be drawn (the canvas must be in the html file, hidden)
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

        egui_extras::install_image_loaders(&cc.egui_ctx);

        

        let (w, h) = get_global_size(&Layout::new(1, 1));
        

        Self {
            data: data,
            graph_size: (w as usize, h as usize),
            graph_cached: None,
            drawn_error: None,
            command: GraphCommands::default(),
            automatic_redraw: false,
        }
    }

    /// Draw the graph
    /// Note : erase the previous graph
    #[cfg(target_arch = "wasm32")]
    fn draw_graph(&mut self) {
        // see https://github.com/bluurryy/noise-functions-demo/blob/e23b3eb6cb670412f0433fb06fcd9f97cc43e221/src/app.rs#L420

        use crate::commands::graph_commands::GraphType;
        use plot_helper::data::filtering::Filters;
        use plot_helper::plotter::scatter_plot::scatter_plot_with_backend;
        use plot_helper::plotter::line_plot::line_plot_with_backend;
        use plotters::backend::{PixelFormat, RGBPixel};
        use plotters::prelude::BitMapBackend;
        use plotters_canvas::CanvasBackend;
        use base64::prelude::{BASE64_STANDARD, Engine as _};

        // remove the previous canvas
        remove_canvas(GRAPH_CANVAS_ID);
        self.graph_cached = None;
        self.drawn_error = None;

        // prepare the series
        let series: Result<Vec<(K, Option<K>, Filters<K>)>, String> = self.command.get_series();
        if series.is_err() {
            self.drawn_error = Some(series.err().unwrap());
            return;
        }
        let series = series.unwrap();
        

        let layout = Layout::new(1, series.len());
        {
            let (w, h) = get_global_size(&layout);
            self.graph_size = (w as usize, h as usize);
        }

        let canvas = create_canvas(GRAPH_CANVAS_ID, self.graph_size.0, self.graph_size.1);

        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();

        

        match self.command.get_graph_type() {
            GraphType::Scatter => {
                scatter_plot_with_backend(
                    &self.data, 
                    self.command.get_legend(), 
                    backend, 
                    &layout,
                    series, 
                    self.command.get_outlier(),
                ).expect("Error while plotting the graph");
            },
            GraphType::Line(metric) => {
                line_plot_with_backend(
                    &self.data, 
                    self.command.get_legend(), 
                    backend, 
                    &layout,
                    series, 
                    self.command.get_outlier(),
                    metric
                ).expect("Error while plotting the graph");
            },
        };

       
        
        let canvas = get_canvas(GRAPH_CANVAS_ID);
        let data_url_image_str = canvas.to_data_url_with_type(format!("image/{}", GRAPH_IMAGE_TYPE).as_str()).unwrap();
        dbg!(&data_url_image_str);

        let pattern_to_isolate = format!("data:image/{};base64,", GRAPH_IMAGE_TYPE);
        let bytes_str = data_url_image_str.trim_start_matches(pattern_to_isolate.as_str());

        let bytes = BASE64_STANDARD.decode(bytes_str).unwrap();      

        self.graph_cached = Some(bytes);
    }

    fn is_graph_drawn(&self) -> bool {
        self.graph_cached.is_some()
    }

    fn is_automatic_redraw(&self) -> bool {
        self.automatic_redraw
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

        let mut total_height = 0.0;

        let resp = egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            
            // "number of sample" at the top of the screen
            ui.vertical(|ui| {
                ui.label(format!("Number of samples : {}", self.data.nb_samples()));
                ui.separator();
                // ------------------ toolbar ------------------
                let should_redraw = self.command.display_in_ui(ui);

                // ------------------ graph control ------------------

                ui.separator();
                ui.heading("Display :");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if self.is_automatic_redraw() {
                        if should_redraw {
                            #[cfg(target_arch = "wasm32")]
                            self.draw_graph();
                        }
                    } else {
                        if ui.button("Draw").clicked() {
                            #[cfg(target_arch = "wasm32")]
                            self.draw_graph();
                        }
                    } 

                    toggle_ui(ui, &mut self.automatic_redraw);
                    ui.label("Automatically redraw :");
                });

                if !self.is_graph_drawn() {
                    ui.separator();
                    if let Some(e) = self.drawn_error.as_ref() {
                        ui.label(format!("Error while drawing the graph : {}", e));
                    } else {
                        ui.label("No graph to display");
                    }
                }

            });
        });

        total_height += resp.response.rect.height();

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            
        });

        #[cfg(target_arch = "wasm32")]
        if self.is_graph_drawn() {
            total_height += 50.0; // add some space for the image
            update_canvas_style(GRAPH_CANVAS_ID, self.graph_size.0, self.graph_size.1, (total_height as usize, 0));
        }
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
