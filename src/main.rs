use std::io;
use std::time::Duration;
use ratatui::{prelude::*, widgets::*};
use crossterm::event::{self, Event, KeyCode};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Paused,
    Done,
}

#[derive(Debug)]
struct Model {
    // full_text: String,
    // main_word: String,
    leading: String,
    highlight: String,
    follow_word: String,
    // speed: u32,
    paused: bool,
    running_state: RunningState, 
    //options: // for parsing arguments and .config results
}

impl Default for Model {
    fn default() -> Self {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        Self {
            // full_text: buffer,
            // main_word: String::from("main word"),
            leading: "ma".to_string(),
            highlight: "i".to_string(),
            follow_word: "n".to_string(),
            // speed: 200,
            paused: false,
            running_state: RunningState::Running,
        }
    }
}


#[derive(PartialEq)]
enum Message {
    Read,
    Pause,
    // Reset,
    // Finished,
    Quit,
}

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut model = Model::default();
    println!("{:?}", model);
    let mut terminal = tui::init_terminal()?;

    while model.running_state != RunningState::Done {
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }

        if !model.paused {
            update(&mut model, Message::Read);
        }
    }

    tui::restore_terminal()?;

    Ok(())
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        // Match each possible message and decide how the model should change
        Message::Pause => {
            model.paused = !model.paused;
            model.running_state = if model.paused {
                RunningState::Paused
            } else {
                RunningState::Running
            };
        }
        Message::Read => {
            model.paused = false;
            // Get a random string from the vector
            let mut rng = thread_rng();
            let words = ["HELLO", "Something", "Else", "were getting the hang"];
            if let Some(random_str) = words.choose(&mut rng) {
                model.leading = random_str.to_string(); 
            }
        }
        Message::Quit => {
            // You can handle cleanup and exit here
            model.running_state = RunningState::Done;
            println!("q pressed");
        }
        // Message::Reset => {
        //     model.main_word = String::from("yay"); 
        // }
        // Message::Finished => {
        //     model.running_state = RunningState::Done;
        // }
        // Return a new model reflecting those changes
    }
    None
}

fn view(model: &mut Model, f: &mut Frame) {
    //... use `ratatui` functions to draw your UI based on the model's state
    // if model.running_state == RunningState::Running {
    let line = Line::from(vec![Span::raw(&model.leading), Span::styled(&model.highlight, Style::default().fg(Color::Red)), Span::raw(&model.follow_word)]);
    let text = Text::from(line);
    f.render_widget(
        Paragraph::new(text),
        f.size(),
    );
}

fn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        //  KeyCode::Char('j') => Some(Message::Increment),
        KeyCode::Char(' ') => Some(Message::Pause),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => Some(Message::Read),
    }
}

mod tui {
    use crossterm::{
        terminal::{
            disable_raw_mode,
            enable_raw_mode,
            EnterAlternateScreen,
            LeaveAlternateScreen,
        },
        ExecutableCommand
    };

    use ratatui::prelude::*;

    use std::{io::stdout, panic};

    pub fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}
