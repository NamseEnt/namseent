use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::{io::stdout, path::PathBuf};

fn main() -> Result<()> {
    let ip = restore_ip()?;
    let mut state = State {
        ip: ip.unwrap_or_default(),
        page: Page::Idle,
    };

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    while !matches!(state.page, Page::Exit) {
        terminal.draw(|frame| {
            ui(frame, &state);
        })?;
        handle_events(&mut state)?;
    }

    save_ip(&state)?;
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

struct State {
    ip: String,
    page: Page,
}

enum Page {
    Idle,
    RemoteIpInput { input: String },
    Exit,
}

fn handle_events(state: &mut State) -> Result<()> {
    if !event::poll(std::time::Duration::from_millis(50))? {
        return Ok(());
    }
    let event = event::read()?;

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
                    state.page = Page::RemoteIpInput {
                        input: String::new(),
                    };
                    return Ok(());
                }
            }
        }
        Page::RemoteIpInput { input } => {
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
                        state.ip = input.clone();
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
                Line::from(Span::from(format!("Remote IP: {}", state.ip))),
            ]);
            let paragraph = Paragraph::new(text).block(Block::default());
            frame.render_widget(paragraph, chunks[0]);
        }
        Page::RemoteIpInput { input } => {
            let text = Text::from(vec![
                Line::from(Span::from("Enter remote IP")),
                Line::from(Span::from(format!("Remote IP: {}", input))),
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
}

const IP_FILE: &str = "ip.txt";

fn save_ip(state: &State) -> Result<()> {
    let path = PathBuf::from(IP_FILE);
    std::fs::write(path, state.ip.clone())?;
    Ok(())
}

fn restore_ip() -> Result<Option<String>> {
    let path = PathBuf::from(IP_FILE);
    if !path.exists() {
        return Ok(None);
    }
    let ip = std::fs::read_to_string(path)?;
    Ok(Some(ip))
}
