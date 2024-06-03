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
            leading: String::from("ma"),
            highlight: String::from("i"),
            follow_word: String::from("n"),
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


    // for debug
    // let model = Model::default();


    while model.running_state != RunningState::Done {
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
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

// fn render_app(frame: &mut Frame) {
//     let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
//     frame.render_widget(greeting, frame.size());
// }

fn view(model: &mut Model, f: &mut Frame) {
    //... use `ratatui` functions to draw your UI based on the model's state
    // if model.running_state == RunningState::Running {
    f.render_widget(
        Paragraph::new(format!("lead: {} highlight: {} follow:{}", model.leading, model.highlight, model.follow_word)),
        f.size(),
    );
    // }
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

// let text = vec![
//         Line::from(vec![
//             Span::raw("First"),
//             Span::styled("line", Style::new().green().italic()),
//             ".".into(),
//         ]),
//         Line::from("Second line".red()),
//         "Third line".into(),
//     ];
//
//     Paragraph::new(text)
//         .block(Block::bordered().title("Paragraph"))
//         .style(Style::new().white().on_black())
//         .alignment(Alignment::Center)
//         .wrap(Wrap { trim: true });
//
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
