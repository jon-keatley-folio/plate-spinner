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

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

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

const TERM_WIDTHS: [u16; 5] = [80, 106, 132, 158, 184];
const TERM_HEIGHTS: [u16; 4] = [24, 28, 32, 36];

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
        let main_panel_height: u16 = *TERM_HEIGHTS
            .iter()
            .filter(|&x| *x < frame.area().height)
            .max()
            .unwrap_or(&frame.area().height);

        let main_panel_width: u16 = *TERM_WIDTHS
            .iter()
            .filter(|&x| *x < frame.area().width)
            .max()
            .unwrap_or(&frame.area().width);

        let main_panel_y = frame.area().height - main_panel_height;

        let main_rect = Rect::new(
            frame.area().x,
            main_panel_y,
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
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit = true,
            _ => {}
        }
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
