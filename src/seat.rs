use rand::seq::SliceRandom;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Seat {
    member: Option<String>,
}

#[derive(Clone)]
pub struct Seats(pub Vec<Vec<Seat>>);

impl Seat {
    pub fn new(member: Option<String>) -> Self {
        Seat { member }
    }
    pub fn get_member(&self) -> Option<String> {
        self.member.clone()
    }
}

impl Seats {
    pub fn new(seats: Vec<Vec<Seat>>) -> Self {
        Seats(seats)
    }
    pub fn layout(&self, area: Rect) -> Vec<Rc<[Rect]>> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1); self.0.len()])
            .split(area);
        self.0
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Fill(1); v.len()])
                    .split(layout[i])
            })
            .collect::<Vec<Rc<[Rect]>>>()
    }
    pub fn shuffle(&mut self) {
        let mut members: Vec<String> = self
            .0
            .clone()
            .iter()
            .flatten()
            .filter_map(|x| x.member.clone())
            .collect();
        let mut rng = rand::thread_rng();
        members.shuffle(&mut rng);
        for (y, seats_line) in self.0.clone().iter().enumerate() {
            for (x, seat) in seats_line.iter().enumerate() {
                if seat.member.is_none() {
                    continue;
                }
                self.0[y][x] = Seat::new(Some(members.pop().unwrap()));
            }
        }
    }
}

impl Widget for Seat {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(member) = self.member {
            let text = Text::from(Line::from(vec![member.green()]));
            Paragraph::new(text)
                .block(Block::default().borders(Borders::all()))
                .render(area, buf);
        } else {
            Paragraph::default()
                .block(Block::default().borders(Borders::empty()))
                .render(area, buf);
        }
    }
}
