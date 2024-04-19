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
    let path = "seats-map.json";
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    let seats = read_seats(path);
    let mut app = App::new(seats);
    app.run(&mut terminal)?;
    tui::restore()?;
    write_seats(path, app.get_seats())?;
    Ok(())
}

fn read_seats(path: &str) -> Seats {
    let file = fs::read_to_string(path).unwrap();
    let json: Json = serde_json::from_str(&file).unwrap();
    let seats: Vec<Vec<Seat>> = json
        .seats
        .into_iter()
        .map(|seat_line| seat_line.into_iter().map(Seat::new).collect())
        .collect();
    Seats::new(seats)
}

fn write_seats(path: &str, seats: Seats) -> Result<()> {
    let json = Json {
        seats: seats
            .0
            .into_iter()
            .map(|seats_line| {
                seats_line
                    .into_iter()
                    .map(|seat| seat.get_member())
                    .collect()
            })
            .collect(),
    };
    let file = serde_json::to_string_pretty(&json)?;
    let _ = fs::write(path, file);
    Ok(())
}
