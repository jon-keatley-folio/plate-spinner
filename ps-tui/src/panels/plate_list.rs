use crate::{
    actions::{Commands, InfoItem},
    panels::panel::{PSPanel, impl_ps_panel},
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

pub struct PlateList {}

impl_ps_panel!(PlateList);

impl PlateList {
    pub fn has_focus(&self) -> bool {
        false
    }

    pub fn set_focus(&mut self, focus: bool) {}

    pub fn key_input(&mut self, event: KeyEvent) -> Commands {
        Commands::CreatePlate
    }

    pub fn get_actions(&self) -> Option<&[InfoItem]> {
        None
    }

    fn render(&self, frame: &mut Frame) {}
}
