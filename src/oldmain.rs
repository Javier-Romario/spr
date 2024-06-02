use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = version(), about = "ratatui template with crossterm and tokio")]
struct Args {
  /// App tick rate
  #[arg(short, long, default_value_t = 1000)]
  app_tick_rate: u64,
}

use std::error::Error;
use std::io;

use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};


use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};


use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::terminal::Terminal;


use std::error::Error;
use std::io;


mod app;
mod ui;

use app::*;
use ui::*;

fn main() -> Result<(), Box<dyn Error>>  {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {

        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::read()? {
                continue;
            }
        }

        match app.current_screen {

        }

    }
}
