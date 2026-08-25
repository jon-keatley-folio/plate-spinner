use crate::{
    actions::{Commands, InfoItem},
    panels::panel::{PSPanel, impl_ps_panel},
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Clone)]
pub struct Title {}

impl_ps_panel!(Title);

impl Title {
    pub fn new() -> Title {
        Title {}
    }

    pub fn has_focus(&self) -> bool {
        false
    }

    pub fn set_focus(&mut self, focus: bool) {}

    pub fn key_input(&mut self, event: KeyEvent) -> Commands {
        Commands::NoAction
    }

    pub fn get_actions(&self) -> Option<&[InfoItem]> {
        None
    }

    fn render(&self, frame: &mut Frame, bounds: Rect) {
        //check frame size
        //draw border with title
        //render plates
        let border_style = Style::new().on_black().fg(Color::Cyan);

        let block = Block::bordered()
            .border_set(border::PLAIN)
            .border_style(border_style);
        frame.render_widget(block, bounds);
    }
}
