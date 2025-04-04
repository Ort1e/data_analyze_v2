use egui::Ui;
use plot_helper::data::filtering::Filters;
use plot_helper::data::sample::key::SerieKey;
use plot_helper::stat::stats_serie::MetricName;

use crate::app::get_str_from_opt_key;

use crate::toggle_ui;

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
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Select graph type!")
                .selected_text(format!("{}", self.get_display_type()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(self, GraphType::Scatter, "Scatter");
                    ui.selectable_value(self, GraphType::Line(MetricName::Mean), "Line");
                });
            match self {
                GraphType::Line(m) => {
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




#[derive(Debug, Clone, PartialEq)]
pub struct GraphCommands<K>
where
K: SerieKey,
{
    axis : Vec<(Option<K>, Option<K>, Filters<K>)>,
    legend: Option<K>,
    graph_type: GraphType,
    outlier: bool,
    
}

impl<K> GraphCommands<K>
where
K: SerieKey,
{
    /// return true if the command has changed
    pub fn display_in_ui(&mut self, ui : &mut Ui) -> bool{
        let old_self = self.clone();

        ui.vertical(|ui| {
            // ----------------------------- graph basis -----------------------------
            // Base of the graph
            ui.heading("Graph basis : ");
            ui.horizontal(|ui| {
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
                // ----------------------------- graph type -----------------------------
                ui.separator();
                self.graph_type.display_in_ui(ui);
               
                // ----------------------------- outlier -----------------------------
                ui.separator();
                
                ui.label("Outlier :");
                toggle_ui(ui, &mut self.outlier);

            });

            ui.separator();

            // each drawn
            ui.heading("Graph axis : ");
            self.draw_axis(ui);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                if ui.button("Add".to_string()).clicked() {
                    self.axis.push((None, None, Filters::default()));
                }

                if self.axis.len() > 1 {
                    if ui.button("remove".to_string()).clicked() {
                        self.axis.pop();
                    }
                }
            });
        });

        self != &old_self
    }

    pub fn draw_axis(&mut self, ui : &mut Ui) {
        for (n, (x_axis, y_axis, _)) in self.axis.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                // x axis
                egui::ComboBox::from_label(format!("Select X axis for {}", n + 1))
                    .selected_text(format!("{}", get_str_from_opt_key(x_axis)))
                    .show_ui(ui, |ui| {
                        for k in K::get_possible_values() {
                            if k.is_numeric() {
                                ui.selectable_value(x_axis, Some(k), format!("{}", k));
                            }
                        }
                    }
                );
                ui.separator();
                // y axis
                egui::ComboBox::from_label(format!("Select Y axis for {}", n + 1))
                    .selected_text(format!("{}", get_str_from_opt_key(y_axis)))
                    .show_ui(ui, |ui| {
                        for k in K::get_possible_values() {
                            if k.is_numeric() {
                                ui.selectable_value(y_axis, Some(k), format!("{}", k));
                            }
                        }
                        ui.selectable_value(y_axis, None, "None");
                    }
                );
            });
            ui.separator();
        }
    }

    pub fn get_axis(&self) -> &Vec<(Option<K>, Option<K>, Filters<K>)> {
        &self.axis
    }

    pub fn get_n_axis(&self, n : usize) -> &(Option<K>, Option<K>, Filters<K>) {
        &self.axis[n]
    }

    pub fn get_mut_n_axis(&mut self, n : usize) -> &mut (Option<K>, Option<K>, Filters<K>) {
        self.axis.get_mut(n).unwrap()
    }

    pub fn get_graph_type(&self) -> GraphType {
        self.graph_type
    }

    pub fn get_legend(&self) -> Option<K> {
        self.legend
    }

    pub fn get_outlier(&self) -> bool {
        self.outlier
    }
}

impl<K> Default for GraphCommands<K> 
where
K: SerieKey,
{
    fn default() -> Self {
        GraphCommands {
            axis: vec![(None, None, Filters::default())],
            legend: None,
            graph_type: GraphType::default(),
            outlier: false,
        }
    }
}