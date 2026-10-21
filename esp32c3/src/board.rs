//! Boards around the ESP32-C3. None modelled yet beyond the bare module; a board implements
//! `esp_soc::BoardModel` and gets a name here.
use esp_soc::{Board, NoBoard};

pub fn make_board(name: &str) -> Option<Board> {
    match name {
        "none" | "bare" => Some(Box::new(NoBoard)),
        _ => None,
    }
}
