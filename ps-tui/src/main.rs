/* TODO
- [ ] basic TUI app
- [ ] config for DB location
- [ ] Control panel
- [ ] Add plate
- [ ] Edit plate
- [ ] Listing - with list option
- [ ] Select item from list to spin (or unpause if looking at paused plates)
- [ ] About and other polish
*/

/*
  4 panels
      - Content panel
      - Info panel
      - Actions panel
      - Title
*/
pub(crate) mod actions;
pub(crate) mod panels {
    pub(crate) mod panel;
    pub(crate) mod plate_list;
}

use std::{any::Any, io};

use crossterm::event;

use ratatui::{DefaultTerminal, Frame, layout::Rect, widgets::Clear};

use crate::panels::{panel::PSPanel, plate_list::PlateList};
use ps_core::plate_data::{Action, DBError, List, connect};

enum Mode {
    List,
    NewPlate,
    EditPlate,
}

struct PlateSpinnerApp {
    exit: bool,
    mode: Mode,
    list_panel: PlateList,
}

impl PlateSpinnerApp {
    fn new() -> Result<PlateSpinnerApp, String> {
        Ok(PlateSpinnerApp {
            exit: false,
            mode: Mode::List,
            list_panel: PlateList::new(),
        })
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        //check there is room for the title
        //chunk up space for list, controls, info
        let show_title = frame.area().height > 20;
        let show_extra = frame.area().width > 30;
        let main_panel_width = if show_extra {
            frame.area().width - 28
        } else {
            frame.area().width
        };

        let main_panel_height = if show_title {
            frame.area().height - 18
        } else {
            frame.area().height
        };

        let main_rect = Rect::new(
            frame.area().x,
            frame.area().y,
            main_panel_width,
            main_panel_height,
        );

        match self.mode {
            Mode::List => {
                self.list_panel.render(frame, main_rect);
            }
            Mode::NewPlate => {}
            Mode::EditPlate => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn main() {
    let app_result = PlateSpinnerApp::new();

    match app_result {
        Ok(mut app) => {
            let mut terminal = ratatui::init();
            let _ = app.run(&mut terminal);
            ratatui::restore();
        }
        Err(err) => {
            println!("ERROR! {}", err);
        }
    }
}
