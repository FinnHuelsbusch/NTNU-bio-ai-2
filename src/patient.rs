use serde::Deserialize;



#[derive(Debug, Deserialize)]
pub struct Patient {
    pub demand: u8,
    pub start_time: u32,
    pub end_time: u32,
    pub care_time: u32,

    pub x_coord: u32,
    pub y_coord: u32,
}
