use layout::Layout;

use crate::params::{LABEL_HORIZONTAL_SIZE, ONE_FIG_SIZE};




pub mod scatter_plot;
pub mod line_plot;


pub mod utils;
pub mod layout;
pub mod plot_data;


fn get_global_size(layout : &Layout) -> (u32, u32) {
    (layout.width as u32 * ONE_FIG_SIZE.0 + LABEL_HORIZONTAL_SIZE, layout.height as u32 * ONE_FIG_SIZE.1)
}