use egui::Ui;
use plot_helper::data::sample::key::SerieKey;

use crate::app::get_str_from_opt_key;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Commands<K>
where
K: SerieKey,
{
    changed: bool,
    x_axis: Option<K>,
    y_axis: Option<K>,
}

impl<K> Commands<K>
where
K: SerieKey,
{
    pub fn new(x_axis: Option<K>, y_axis: Option<K>) -> Self {
        Commands {
            changed: false,
            x_axis,
            y_axis,
        }
    }

    pub fn display_in_ui(&self, ui : &mut Ui) -> Self {
        let mut commands = self.clone();
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
                                    ui.selectable_value(&mut commands.x_axis, Some(k), format!("{}", k));
                                }
                            }
                        }
                    );
                });
                ui.separator();
                // y axis
                ui.horizontal(|ui| {
                    ui.label("Y axis :");

                    egui::ComboBox::from_label("Select Y axis!")
                        .selected_text(format!("{}", get_str_from_opt_key(&self.y_axis)))
                        .show_ui(ui, |ui| {
                            for k in K::get_possible_values() {
                                if k.is_numeric() {
                                    ui.selectable_value(&mut commands.y_axis, Some(k), format!("{}", k));
                                }
                            }
                        }
                    );
                });
            });
        });


        commands.changed = self != &commands;

        commands
    }

    pub fn has_changed(&mut self) {
        self.changed = true;
    }

    pub fn is_changed(&self) -> bool {
        self.changed
    }

    pub fn get_x_axis(&self) -> Option<K> {
        self.x_axis
    }

    pub fn get_y_axis(&self) -> Option<K> {
        self.y_axis
    }

    
}

impl<K> Default for Commands<K> 
where
K: SerieKey,
{
    fn default() -> Self {
        Commands {
            x_axis: None,
            y_axis: None,
            changed: false,
        }
    }
}