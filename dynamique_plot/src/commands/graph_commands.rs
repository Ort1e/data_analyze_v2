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
    graph_type: GraphType,
}

impl<K> GraphCommands<K>
where
K: SerieKey,
{
    pub fn new(x_axis: Option<K>, y_axis: Option<K>, graph_type : GraphType) -> Self {
        GraphCommands {
            x_axis,
            y_axis,
            graph_type,
        }
    }

    pub fn display_in_ui(&mut self, ui : &mut Ui) {
        ui.horizontal(|ui| {
            // ----------------------------- graph basis -----------------------------
            ui.vertical(|ui| {
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
                        }
                    );
                });
            });

            ui.separator();

            // ----------------------------- graph type -----------------------------
            self.graph_type.display_in_ui(ui);
        });
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
}

impl<K> Default for GraphCommands<K> 
where
K: SerieKey,
{
    fn default() -> Self {
        GraphCommands {
            x_axis: None,
            y_axis: None,
            graph_type: GraphType::default(),
        }
    }
}