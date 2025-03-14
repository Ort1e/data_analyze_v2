use std::fmt::{Display, Formatter};

use plot_helper::generate_plot_key;
use serde_derive::{Deserialize, Serialize};
use plot_helper::data::sample::key::SerieKey;

generate_plot_key!(
   
    TestKey[
        Test1Num { "Test 1 num", Numeric },
        Test1Str { "Test 1 str", String },
        Test2Num { "Test 2 num", Numeric },
        Test2Str { "Test 2 str", String }
    ]

);