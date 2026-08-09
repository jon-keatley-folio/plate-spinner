use std::any::Any;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use crate::actions::{Commands, InfoItem};

pub trait PSPanel {
    fn has_focus(&self) -> bool;
    fn set_focus(&mut self, focus: bool);
    fn key_input(&mut self, event: KeyEvent) -> Commands;
    fn get_actions(&self) -> &[InfoItem];
    fn render(&self, frame: &mut Frame)
    where
        Self: Sized;
    fn as_any(&mut self) -> &mut dyn Any;
}

macro_rules! impl_ps_panel {
    ($T:ident) => {
        impl PSPanel for $T {
            fn has_focus(&self) -> bool {
                self.has_focus
            }

            fn set_focus(&mut self, focus: bool) {
                self.has_focus = focus;
            }

            fn key_input(&mut self, event: KeyEvent) -> Commands {
                self.handle_key_input(event)
            }

            fn get_actions(&self) -> &[InfoItem] {
                self.actions.as_slice()
            }

            fn render(&self, frame: &mut Frame) {
                self.draw(frame);
            }

            fn as_any(&mut self) -> &mut dyn Any {
                self
            }
        }
    };
}

pub(crate) use impl_ps_panel;
