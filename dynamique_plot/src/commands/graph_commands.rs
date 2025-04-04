use egui::{Direction, Layout, Ui};
use plot_helper::data::filtering::{Filter, Filters, Operator};
use plot_helper::data::sample::key::SerieKey;
use plot_helper::stat::stats_serie::MetricName;

use crate::app::get_str_from_opt_key;

use crate::toggle_ui;





#[derive(Debug, Clone, PartialEq)]
pub struct GraphCommands<K>
where
K: SerieKey,
{
    axis : Vec<(Option<K>, Option<K>, Vec<UiFilterData<K>>)>,
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
            ui.heading("Graph details : ");
            self.draw_axis(ui);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                if ui.button("Add grah".to_string()).clicked() {
                    self.axis.push((None, None, Vec::new()));
                }
            });
        });

        self != &old_self
    }

    pub fn draw_axis(&mut self, ui : &mut Ui) {
        let mut axis_to_remove = Vec::new();
        let nb_graphs = self.axis.len();

        for (graph_n, (x_axis, y_axis, ui_filters)) in self.axis.iter_mut().enumerate() {
            ui.heading(format!("Graph {} : ", graph_n + 1));
            ui.horizontal(|ui| {
                // x axis
                egui::ComboBox::from_label(format!("Select X axis for {}", graph_n + 1))
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
                egui::ComboBox::from_label(format!("Select Y axis for {}", graph_n + 1))
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
                ui.separator();
                // adding filters
                if ui.button("Add numeric filter").clicked() {
                    let filter = UiFilterData::Numeric(None, None, 0.0);
                    ui_filters.push(filter);
                }
            });
            ui.separator();
            if ui_filters.len() > 0 {
                ui.label("Filters : ");
            }
            let mut filter_to_remove = Vec::new();
            for (n, ui_filter) in ui_filters.iter_mut().enumerate() {
                ui_filter.draw_ui(ui, graph_n, n);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui.button("Remove filter").clicked() {
                        filter_to_remove.push(n);
                    }
                });
            }

            for n in filter_to_remove.iter().rev() {
                ui_filters.remove(*n);
            }
            
            if nb_graphs > 1 {
                
                if ui.button("Remove graph").clicked() {
                    axis_to_remove.push(graph_n);
                }
            }

            ui.separator();
        }

        for n in axis_to_remove.iter().rev() {
            self.axis.remove(*n);
        }
    }

    pub fn get_axis(&self) -> &Vec<(Option<K>, Option<K>, Vec<UiFilterData<K>>)> {
        &self.axis
    }

    pub fn get_n_axis(&self, n : usize) -> &(Option<K>, Option<K>, Vec<UiFilterData<K>>) {
        &self.axis[n]
    }

    pub fn get_mut_n_axis(&mut self, n : usize) -> &mut (Option<K>, Option<K>, Vec<UiFilterData<K>>) {
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

    pub fn get_series(&self) -> Result<Vec<(K, Option<K>, Filters<K>)>, String> {
        let mut series: Vec<(K, Option<K>, Filters<K>)> = Vec::new();
        
        for (i, (x_axis, y_axis, ui_filters)) in self.get_axis().iter().enumerate() {
            if x_axis.is_none() {
                return Err(format!("No x axis selected for graph part {}", i + 1));
            }

            let mut filter = Filters::empty();
            for ui_filter in ui_filters {
                let f = ui_filter.into_filter()?;
                filter.add_filter(f);
            }

            series.push((x_axis.unwrap().clone(), y_axis.clone(), filter.into()));
        }

        Ok(series)
    }
}

impl<K> Default for GraphCommands<K> 
where
K: SerieKey,
{
    fn default() -> Self {
        GraphCommands {
            axis: vec![(None, None, Vec::new())],
            legend: None,
            graph_type: GraphType::default(),
            outlier: false,
        }
    }
}


// ----------------- ui filter data -----------------

#[derive(Debug, Clone, PartialEq)]
enum UiFilterData<K>
where
    K: SerieKey,
{
    Numeric(Option<K>, Option<Operator>, f32),
    String(Option<K>, Option<Operator>, String),
}

impl<K> UiFilterData<K>
where
    K: SerieKey,
{

    pub fn get_key(&self) -> Option<K> {
        match self {
            UiFilterData::Numeric(k, _, _) => *k,
            UiFilterData::String(k, _, _) => *k,
        }
    }

    pub fn get_operator(&self) -> Option<Operator> {
        match self {
            UiFilterData::Numeric(_, o, _) => *o,
            UiFilterData::String(_, o, _) => *o,
        }
    }

    pub fn get_numeric_value(&self) -> f32 {
        match self {
            UiFilterData::Numeric(_, _, v) => *v,
            UiFilterData::String(_, _, _) => panic!("Not a numeric filter"),
        }
    }

    pub fn get_string_value(&self) -> String{
        match self {
            UiFilterData::Numeric(_, _, _) => panic!("Not a string filter"),
            UiFilterData::String(_, _, v) => v.clone(),
        }
    }

    pub fn draw_ui(&mut self, ui : &mut Ui, graph_n : usize, filter_n : usize) {
        match self {
            UiFilterData::Numeric(k, o, v) => {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_salt(format!("Select key for filter {} for graph {}", filter_n + 1, graph_n + 1))
                        .selected_text(format!("{}", get_str_from_opt_key(k)))
                        .show_ui(ui, |ui| {
                            for ref_k in K::get_possible_values() {
                                if ref_k.is_numeric() {
                                    ui.selectable_value(k, Some(ref_k), format!("{}", ref_k));
                                }
                            }
                        });
                    egui::ComboBox::from_id_salt(format!("Select operator for filter {} for graph {}", filter_n + 1, graph_n + 1))
                        .selected_text(format!("{}", get_str_from_opt_key(o)))
                        .show_ui(ui, |ui| {
                            for op in Operator::get_all() {
                                ui.selectable_value(o, Some(op), format!("{}", op));
                            }
                        });
                    ui.add(egui::DragValue::new(v).speed(0.1));
                });
            }
            UiFilterData::String(k, o, v) => {
                todo!()
            }
        }
    }

    pub fn into_filter(&self) -> Result<Filter<K>, String> {
        match self {
            UiFilterData::Numeric(k, o, v) => {
                if let (Some(k), Some(o), v) = (k, o, v) {
                    Ok(Filter::new_number(*k, *o, *v))
                } else {
                    Err("Incomplete numeric filter".to_string())
                }
            }
            UiFilterData::String(k, o, v) => {
                todo!()
            }
        }
    }
}



// ------------ graph type ------------
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
