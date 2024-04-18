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
}

impl Widget for Seat {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(member) = self.member {
            let text = Text::from(Line::from(vec![member.green().into()]));
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
