mod sync_loop;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::{
    io::stdout,
    path::PathBuf,
    sync::{Arc, RwLock},
};

fn main() -> Result<()> {
    let uri = restore_uri()?;
    let state = Arc::new(RwLock::new(State {
        uri: uri.unwrap_or_default(),
        page: Page::Idle,
        sync_loop_error: None,
    }));

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    std::thread::spawn({
        let state = state.clone();
        move || match sync_loop::sync_loop(state.clone()) {
            Ok(_) => {}
            Err(e) => {
                let mut state = state.write().unwrap();
                state.sync_loop_error = Some(e.to_string());
            }
        }
    });

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        {
            let state = state.read().unwrap();
            terminal.draw(|frame| {
                ui(frame, &state);
            })?;
        }
        if event::poll(std::time::Duration::from_millis(50))? {
            let event = event::read()?;
            let mut state = state.write().unwrap();
            handle_event(&mut state, event)?;

            if matches!(state.page, Page::Exit) {
                break;
            }
        }
    }

    let state = state.read().unwrap();
    save_uri(&state)?;
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

struct State {
    uri: String,
    page: Page,
    sync_loop_error: Option<String>,
}

enum Page {
    Idle,
    RemoteuriInput { input: String },
    Exit,
}

fn handle_event(state: &mut State, event: Event) -> Result<()> {
    if let Event::Key(key) = event {
        if key.kind == event::KeyEventKind::Press
            && key.code == KeyCode::Char('c')
            && key.modifiers == event::KeyModifiers::CONTROL
        {
            state.page = Page::Exit;
            return Ok(());
        }
    }

    match &mut state.page {
        Page::Idle => {
            if let Event::Key(key) = event {
                if key.kind == event::KeyEventKind::Release && key.code == KeyCode::Char('1') {
                    state.page = Page::RemoteuriInput {
                        input: String::new(),
                    };
                    return Ok(());
                }
            }
        }
        Page::RemoteuriInput { input } => {
            if let Event::Key(key) = event {
                if key.kind != event::KeyEventKind::Release {
                    return Ok(());
                }
                match key.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        state.uri = input.clone();
                        state.page = Page::Idle;
                    }
                    _ => {}
                }
            }
        }
        Page::Exit => {}
    }
    Ok(())
}

fn ui(frame: &mut Frame, state: &State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.size());

    let block = Block::default()
        .title("Namui Remote Start")
        .borders(Borders::ALL);
    frame.render_widget(block, frame.size());

    match &state.page {
        Page::Idle => {
            let text = Text::from(vec![
                Line::from(Span::from("Press 1 to connect to a remote ratatui server")),
                Line::from(Span::from(format!("Remote uri: {}", state.uri))),
            ]);
            let paragraph = Paragraph::new(text).block(Block::default());
            frame.render_widget(paragraph, chunks[0]);
        }
        Page::RemoteuriInput { input } => {
            let text = Text::from(vec![
                Line::from(Span::from("Enter remote uri")),
                Line::from(Span::from(format!("Remote uri: {}", input))),
            ]);
            let paragraph = Paragraph::new(text).block(Block::default());
            frame.render_widget(paragraph, chunks[0]);
        }
        Page::Exit => {
            let text = Text::from(vec![Line::from(Span::from("Exiting..."))]);
            let paragraph = Paragraph::new(text).block(Block::default());
            frame.render_widget(paragraph, chunks[0]);
        }
    }

    if let Some(error) = &state.sync_loop_error {
        let text = Text::from(vec![Line::from(Span::from(error.clone()))]);
        let paragraph = Paragraph::new(text).block(Block::default());
        frame.render_widget(paragraph, chunks[1]);
    }
}

const URI_FILE: &str = "uri.txt";

fn save_uri(state: &State) -> Result<()> {
    let path = PathBuf::from(URI_FILE);
    std::fs::write(path, state.uri.clone())?;
    Ok(())
}

fn restore_uri() -> Result<Option<String>> {
    let path = PathBuf::from(URI_FILE);
    if !path.exists() {
        return Ok(None);
    }
    let uri = std::fs::read_to_string(path)?;
    Ok(Some(uri))
}
