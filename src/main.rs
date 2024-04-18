mod app;
mod errors;
mod seat;
mod tui;

use app::App;
use color_eyre::Result;
use seat::{Seat, Seats};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Json {
    seats: Vec<Vec<Option<String>>>,
}

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    let seats = read_seats("seats-map.json");
    App::new(seats).run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}

fn read_seats(path: &str) -> Seats {
    let file = fs::read_to_string(path).unwrap();
    let json: Json = serde_json::from_str(&file).unwrap();
    let seats: Vec<Vec<Seat>> = json
        .seats
        .into_iter()
        .map(|seat_line| {
            seat_line
                .into_iter()
                .map(|member| Seat::new(member))
                .collect()
        })
        .collect();
    Seats::new(seats)
}
