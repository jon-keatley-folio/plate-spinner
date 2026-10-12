/* TODO
- [x] basic TUI app
- [ ] Add message panel
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
pub(crate) mod config;
pub(crate) mod panels {
    pub(crate) mod controls;
    pub(crate) mod info;
    pub(crate) mod panel;
    pub(crate) mod plate_list;
    pub(crate) mod title;
}

use std::time::Duration;
use std::{any::Any, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{DefaultTerminal, Frame, layout::Rect, widgets::Clear};

use crate::panels::{
    controls::Controls, info::Info, panel::PSPanel, plate_list::PlateList, title::Title,
};

use crate::config::create_or_get_save_path;

use ps_core::PlateSpinnerCore;
use ps_core::model::Plate;
use ps_core::plate_data::{Action, DBError, List};

const LIST_REFRESH_RATE: usize = 100; // att 100ms that should be 10 seconds

enum Mode {
    List,
    NewPlate,
    EditPlate,
    Message,
}

struct PlateSpinnerApp {
    exit: bool,
    mode: Mode,
    list_panel: PlateList,
    title: Title,
    info: Info,
    controls: Controls,
    core: PlateSpinnerCore,
    refresh_list_ticks: usize,
    plates: Vec<Plate>,
    selected_plate: Option<Plate>,
}

impl PlateSpinnerApp {
    fn new(core: PlateSpinnerCore) -> Result<PlateSpinnerApp, String> {
        Ok(PlateSpinnerApp {
            exit: false,
            mode: Mode::List,
            list_panel: PlateList::new(),
            title: Title::new(),
            info: Info::new(),
            controls: Controls::new(),
            core,
            refresh_list_ticks: LIST_REFRESH_RATE,
            plates: Vec::new(),
            selected_plate: None,
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
        let main_panel_height = f32::floor(frame.area().height as f32 * 0.8f32) as u16;
        let main_panel_width = f32::floor(frame.area().width as f32 * 0.8f32) as u16;

        let main_panel_y = frame.area().height - main_panel_height;
        let right_panel_width = frame.area().width - main_panel_width;

        let title_rect = Rect::new(0, 0, main_panel_width, main_panel_y);

        let controls_rect = Rect::new(main_panel_width, 0, right_panel_width, main_panel_y);

        let main_rect = Rect::new(
            frame.area().x,
            main_panel_y,
            main_panel_width,
            main_panel_height,
        );

        let info_rect = Rect::new(
            main_panel_width,
            main_panel_y,
            right_panel_width,
            main_panel_height,
        );

        self.title.render(frame, title_rect);
        self.info.render(frame, info_rect);
        self.controls.render(frame, controls_rect);
        match self.mode {
            Mode::List => {
                self.list_panel.render(frame, main_rect);
            }
            Mode::Message => {
                //todo add message
            }
            Mode::NewPlate => {}
            Mode::EditPlate => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100)).is_ok() {
            self.refresh_list_ticks += 1;

            if self.refresh_list_ticks >= LIST_REFRESH_RATE {
                //todo reload list
                self.refresh_list_ticks = 0;

                let update_list = List::TopTopples(10);
                let update_result = self.core.execute_list(update_list);
                match update_result {
                    Ok(l) => self.plates = l,
                    Err(e) => {
                        //todo
                        // change to message
                        // display error
                    }
                }
            }

            self.title.tick();

            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => {}
            };
        }

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
    //initialise
    // get save path
    // check for db
    // let the user know if a db is created
    let save_path = match create_or_get_save_path() {
        Ok(sp) => sp,
        Err(e) => {
            println!("Fatal error starting plat spinner! {}", e);
            return;
        }
    };

    let core = match PlateSpinnerCore::new(&save_path.to_string_lossy()) {
        Ok(c) => c,
        Err(e) => {
            println!(
                "Fatal error connecting to db! {}\n{}",
                e,
                save_path.to_string_lossy()
            );
            return;
        }
    };

    let app_result = PlateSpinnerApp::new(core);
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
