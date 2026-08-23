use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::actions::{Commands, InfoItem};

pub trait PSPanel {
    fn has_focus(&self) -> bool;
    fn set_focus(&mut self, focus: bool);
    fn key_input(&mut self, event: KeyEvent) -> Commands;
    fn get_actions(&self) -> Option<&[InfoItem]>;
    fn render(&self, frame: &mut Frame, bounds: Rect);
}

macro_rules! impl_ps_panel {
    ($T:ident) => {
        impl PSPanel for $T {
            fn has_focus(&self) -> bool {
                self.has_focus()
            }

            fn set_focus(&mut self, focus: bool) {
                self.set_focus(focus);
            }

            fn key_input(&mut self, event: KeyEvent) -> Commands {
                self.key_input(event)
            }

            fn get_actions(&self) -> Option<&[InfoItem]> {
                self.get_actions()
            }

            fn render(&self, frame: &mut Frame, bounds: Rect) {
                self.render(frame, bounds);
            }
        }
    };
}

pub(crate) use impl_ps_panel;
