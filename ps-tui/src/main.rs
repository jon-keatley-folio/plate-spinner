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
mod actions;
mod panels {
    mod panel;
    mod plate_list;
}

use std::io;

use crossterm::event;

use ratatui::{DefaultTerminal, Frame, layout::Rect, widgets::Clear};

use ps_core::plate_data::{Action, DBError, List, connect};

struct PlateSpinnerApp {
    exit: bool,
}

impl PlateSpinnerApp {
    fn new() -> Result<PlateSpinnerApp, String> {
        Ok(PlateSpinnerApp { exit: false })
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {}

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
