use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Patient {
    #[serde(default)]
    pub id: usize,
    pub demand: u32,
    pub start_time: u32,
    pub end_time: u32,
    pub care_time: u32,

    pub x_coord: u32,
    pub y_coord: u32,
}
