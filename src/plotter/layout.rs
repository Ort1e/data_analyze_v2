/// define a layout for the subplots
pub struct Layout {
    pub height : usize,
    pub width : usize,
}

impl Layout {
    pub fn new(width : usize, height : usize) -> Self {
        Self {
            height,
            width,
        }
    }
    /// get the plotter layout (row, col)
    pub fn get_plotter_layout(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    pub fn get_nb_of_subplots(&self) -> usize {
        self.height * self.width
    }
}