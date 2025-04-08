use egui::Ui;
use graph_commands::GraphCommands;
use plot_helper::data::sample::key::SerieKey;
use plot_helper::data::sample::Sample;
use plot_helper::data::sample_serie::memory_sample_serie::MemorySampleSerie;
use plot_helper::plotter::get_global_size;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::toggle_ui;

#[cfg(target_arch = "wasm32")]
use crate::{create_canvas, get_canvas, remove_canvas, update_canvas_style};

use super::HasCommands;

pub const GRAPH_IMAGE_TYPE: &str = "png";
pub const GRAPH_CANVAS_ID: &str = "graph_canvas";

pub mod graph_commands;


#[derive(Debug, Clone)]
pub struct GraphApp<K>
where
    K: SerieKey,
{
    graph_size : (usize, usize),

    graph_cached : Option<Vec<u8>>,
    drawn_error : Option<String>,

    automatic_redraw: bool,
    command: GraphCommands<K>,
}

impl<K> GraphApp<K>
where
    K: SerieKey
{
    /// Draw the graph
    /// Note : erase the previous graph
    #[cfg(target_arch = "wasm32")]
    fn draw_graph<S>(&mut self, data: &MemorySampleSerie<S, K>)
    where
        S: Sample<K>,
    {
        // see https://github.com/bluurryy/noise-functions-demo/blob/e23b3eb6cb670412f0433fb06fcd9f97cc43e221/src/app.rs#L420

        use crate::sub_app::graph_app::graph_commands::GraphType;
        
        use plot_helper::data::filtering::Filters;
        use plot_helper::plotter::scatter_plot::scatter_plot_with_backend;
        use plot_helper::plotter::line_plot::line_plot_with_backend;
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
        

        let layout = self.command.get_layout();
        {
            let (w, h) = get_global_size(&layout);
            self.graph_size = (w as usize, h as usize);
        }

        let canvas = create_canvas(GRAPH_CANVAS_ID, self.graph_size.0, self.graph_size.1);

        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();

        

        match self.command.get_graph_type() {
            GraphType::Scatter => {
                scatter_plot_with_backend(
                    data, 
                    self.command.get_legend(), 
                    backend, 
                    &layout,
                    series, 
                    self.command.get_outlier(),
                ).expect("Error while plotting the graph");
            },
            GraphType::Line(metric) => {
                line_plot_with_backend(
                    data, 
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

    pub fn draw_ui<S>(&mut self, ui : &mut Ui, data : &MemorySampleSerie<S, K>) 
    where 
        S: Sample<K>,
    {
        let mut total_height = 0.0;

        let resp =  ui.vertical(|ui| {
            ui.label(format!("Number of samples : {}", data.nb_samples()));
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
                        self.draw_graph(data);
                    }
                } else {
                    if ui.button("Draw").clicked() {
                        #[cfg(target_arch = "wasm32")]
                        self.draw_graph(data);
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

        total_height += resp.response.rect.height();

        #[cfg(target_arch = "wasm32")]
        if self.is_graph_drawn() {
            total_height += 50.0; // add some space for the image
            update_canvas_style(GRAPH_CANVAS_ID, self.graph_size.0, self.graph_size.1, (total_height as usize, 0));
        }
    }



    fn is_graph_drawn(&self) -> bool {
        self.graph_cached.is_some()
    }

    fn is_automatic_redraw(&self) -> bool {
        self.automatic_redraw
    }

    pub fn get_command(&self) -> &GraphCommands<K> {
        &self.command
    }
}

impl<K> HasCommands<GraphCommands<K>> for GraphApp<K>
where
    K: SerieKey + Serialize + DeserializeOwned,
{
    fn get_commands(&self) -> &GraphCommands<K> {
        &self.command
    }

    fn get_app_storage_key() -> String {
        "graph_app".to_string()
    }

    fn from_commands(commands: GraphCommands<K>) -> Self {
        let (w, h) = get_global_size(&commands.get_layout());
        Self {
            graph_size: (w as usize, h as usize),
            graph_cached: None,
            drawn_error: None,
            command : commands,
            automatic_redraw: false,
        }
    }
}