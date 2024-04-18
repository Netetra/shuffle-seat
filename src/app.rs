use crate::{seat::Seats, tui};
use color_eyre::eyre::Context;
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, Borders, Paragraph},
};
use std::io;

pub struct App {
    seats: Seats,
    exit: bool,
}

impl App {
    pub fn new(seats: Seats) -> Self {
        App { seats, exit: false }
    }
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size())
    }
    fn handle_events(&mut self) -> io::Result<()> {
        if let event::Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                self.handle_key_event(key_event);
            }
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.exit(),
            KeyCode::Char(' ') => {
                self.seats.shuffle();
            }
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title_top(Line::from(" Shuffle Seats ".yellow()).centered())
            .title_bottom(
                Line::from(vec![
                    " Shuffle ".yellow(),
                    "<Space>".blue(),
                    " Exit ".yellow(),
                    "<Q> or <ESC> ".blue(),
                ])
                .centered(),
            )
            .borders(Borders::all())
            .border_style(Style::new());
        let inner = block.inner(area);
        let layouts = self.seats.layout(inner);

        for (layout, seats_line) in layouts.iter().zip(self.seats.0.clone().into_iter()) {
            for (rect, seat) in layout.iter().zip(seats_line.into_iter()) {
                seat.render(*rect, buf);
            }
        }

        Paragraph::default().block(block).render(area, buf);
    }
}
