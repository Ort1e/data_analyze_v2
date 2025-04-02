use egui::Ui;
use plot_helper::data::sample::key::SerieKey;
use plot_helper::stat::stats_serie::MetricName;

use crate::app::get_str_from_opt_key;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphType {
    Line(MetricName),
    Scatter,
}

impl Default for GraphType {
    fn default() -> Self {
        GraphType::Scatter
    }
}

impl GraphType {
    pub fn display_in_ui(&mut self, ui : &mut Ui) {
        ui.vertical(|ui| {
            ui.label("Graph type :");
            egui::ComboBox::from_label("Select graph type!")
                .selected_text(format!("{}", self.get_display_type()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(self, GraphType::Scatter, "Scatter");
                    ui.selectable_value(self, GraphType::Line(MetricName::Mean), "Line");
                });

            match self {
                GraphType::Line(m) => {
                    ui.label("Metric :");
                    egui::ComboBox::from_label("Select metric!")
                        .selected_text(format!("{}", m))
                        .show_ui(ui, |ui| {
                            for all_m in MetricName::get_all() {
                                ui.selectable_value(m, all_m, format!("{}", all_m));
                            }
                        });
                },
                _ => {}
            }

            
        });
    }

    fn change_metric(&mut self, metric : MetricName) {
        if let GraphType::Line(m) = self {
            *m = metric;
        }
    }

    fn get_display_type(&self) -> String {
        match self {
            GraphType::Line(_) => "Line".to_string(),
            GraphType::Scatter => "Scatter".to_string(),
        }
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphCommands<K>
where
K: SerieKey,
{
    x_axis: Option<K>,
    y_axis: Option<K>,
    legend: Option<K>,
    graph_type: GraphType,

    automatic_redraw: bool,
}

impl<K> GraphCommands<K>
where
K: SerieKey,
{
    pub fn new(x_axis: Option<K>, y_axis: Option<K>, legend : Option<K>, graph_type : GraphType, automatic_redraw : bool) -> Self {
        GraphCommands {
            x_axis,
            y_axis,
            legend,
            graph_type,
            automatic_redraw
        }
    }

    /// return true if the command has changed
    pub fn display_in_ui(&mut self, ui : &mut Ui) -> bool{
        let old_self = self.clone();

        ui.horizontal(|ui| {
            // ----------------------------- graph basis -----------------------------
            ui.vertical(|ui| {
                // legend
                ui.horizontal(|ui| {
                    ui.label("Legend :");

                    egui::ComboBox::from_label("Select legend!")
                        .selected_text(format!("{}", get_str_from_opt_key(&self.legend)))
                        .show_ui(ui, |ui| {
                            for k in K::get_possible_values() {
                                if k.is_string() {
                                    ui.selectable_value(&mut self.legend, Some(k), format!("{}", k));
                                }
                            }
                            ui.selectable_value(&mut self.legend, None, "None");
                        }
                    );
                });
                // x axis
                ui.horizontal(|ui| {
                    ui.label("X axis :");

                    egui::ComboBox::from_label("Select X axis!")
                        .selected_text(format!("{}", get_str_from_opt_key(&self.x_axis)))
                        .show_ui(ui, |ui| {
                            for k in K::get_possible_values() {
                                if k.is_numeric() {
                                    ui.selectable_value(&mut self.x_axis, Some(k), format!("{}", k));
                                }
                            }
                        }
                    );
                });
                // y axis
                ui.horizontal(|ui| {
                    ui.label("Y axis :");

                    egui::ComboBox::from_label("Select Y axis!")
                        .selected_text(format!("{}", get_str_from_opt_key(&self.y_axis)))
                        .show_ui(ui, |ui| {
                            for k in K::get_possible_values() {
                                if k.is_numeric() {
                                    ui.selectable_value(&mut self.y_axis, Some(k), format!("{}", k));
                                }
                            }
                            ui.selectable_value(&mut self.y_axis, None, "None");
                        }
                    );
                });
            });

            ui.separator();

            // ----------------------------- graph type -----------------------------
            self.graph_type.display_in_ui(ui);
        });

        self != &old_self
    }

    pub fn get_x_axis(&self) -> Option<K> {
        self.x_axis
    }

    pub fn get_y_axis(&self) -> Option<K> {
        self.y_axis
    }

    pub fn get_graph_type(&self) -> GraphType {
        self.graph_type
    }

    pub fn get_legend(&self) -> Option<K> {
        self.legend
    }

    pub fn get_mut_automatic_redraw(&mut self) -> &mut bool {
        &mut self.automatic_redraw
    }

    pub fn get_automatic_redraw(&self) -> bool {
        self.automatic_redraw
    }
}

impl<K> Default for GraphCommands<K> 
where
K: SerieKey,
{
    fn default() -> Self {
        GraphCommands {
            x_axis: None,
            y_axis: None,
            legend: None,
            graph_type: GraphType::default(),
            automatic_redraw: false,
        }
    }
}