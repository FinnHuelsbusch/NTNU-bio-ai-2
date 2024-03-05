use serde::Deserialize;


    #[derive(Debug, Deserialize)]
    pub struct Depot {
        pub x_coord: u32,
        pub y_coord: u32,
        pub return_time: u32,
    }




