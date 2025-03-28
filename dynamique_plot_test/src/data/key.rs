

use std::fmt::{Display, Formatter};

use plot_helper::generate_plot_key;
use serde_derive::{Deserialize, Serialize};
use plot_helper::data::sample::key::SerieKey;

generate_plot_key!(
   
    FileKey[
        Language { "Language", String },
        NbLine { "nb of ligne in the file", Numeric },
        NbChar { "nb of character in the file", Numeric },
        FileName { "file name", String }
    ]

);