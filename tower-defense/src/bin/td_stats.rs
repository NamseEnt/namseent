use anyhow::{Context, Result};
use clap::Parser;
use crossterm::event::{Event as CEvent, KeyCode, poll, read};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap};
use ratatui::{Frame, Terminal};
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::time::Duration;
use tower_defense::set_headless;
use tower_defense::simulator::stats::{Database, DetailStats, StrategyStats, SummaryRow};

#[derive(Parser)]
#[command(
    name = "td-stats",
    about = "Interactive SQLite statistics explorer for td-simulator"
)]
struct Cli {
    /// SQLite database path for recording results
    #[arg(short, long, default_value = "sim_results.db")]
    db: PathBuf,
}

#[derive(Clone, Copy, Debug)]
enum Tab {
    Items,
    Upgrades,
    Treasures,
    Strategies,
}

impl Tab {
    fn titles() -> [&'static str; 4] {
        ["Items", "Upgrades", "Treasures", "Strategies"]
    }
}

struct App {
    tab: Tab,
    selection: usize,
    items: Vec<SummaryRow>,
    upgrades: Vec<SummaryRow>,
    treasures: Vec<SummaryRow>,
    strategies: Vec<StrategyStats>,
    detail: Option<DetailStats>,
    list_state: ListState,
}

impl App {
    fn new(db: &Database) -> Result<Self> {
        let items = db.list_items()?;
        let upgrades = db.list_upgrades()?;
        let treasures = db.list_treasures()?;
        let strategies = db.list_strategy_win_rates()?;
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let mut app = Self {
            tab: Tab::Items,
            selection: 0,
            items,
            upgrades,
            treasures,
            strategies,
            detail: None,
            list_state,
        };
        app.refresh_detail(db)?;
        Ok(app)
    }

    fn current_list(&self) -> &[SummaryRow] {
        match self.tab {
            Tab::Items => &self.items,
            Tab::Upgrades => &self.upgrades,
            Tab::Treasures => &self.treasures,
            Tab::Strategies => &[],
        }
    }

    fn current_strategy(&self) -> Option<&StrategyStats> {
        self.strategies.get(self.selection)
    }

    fn current_list_len(&self) -> usize {
        match self.tab {
            Tab::Items => self.items.len(),
            Tab::Upgrades => self.upgrades.len(),
            Tab::Treasures => self.treasures.len(),
            Tab::Strategies => self.strategies.len(),
        }
    }

    fn refresh_detail(&mut self, db: &Database) -> Result<()> {
        self.selection = self
            .selection
            .min(self.current_list_len().saturating_sub(1));
        if self.current_list_len() == 0 {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(self.selection));
        }
        self.detail = match self.tab {
            Tab::Items | Tab::Upgrades => self
                .current_list()
                .get(self.selection)
                .map(|row| db.detail_for_shop_purchase(&row.name))
                .transpose()?,
            Tab::Treasures => self
                .current_list()
                .get(self.selection)
                .map(|row| db.detail_for_treasure(&row.name))
                .transpose()?,
            Tab::Strategies => self
                .current_strategy()
                .map(|strategy| db.detail_for_strategy(&strategy.category, &strategy.name))
                .transpose()?,
        };
        Ok(())
    }

    fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Items => Tab::Upgrades,
            Tab::Upgrades => Tab::Treasures,
            Tab::Treasures => Tab::Strategies,
            Tab::Strategies => Tab::Items,
        };
        self.selection = 0;
        self.list_state.select(Some(0));
    }

    fn move_selection(&mut self, delta: isize) {
        let len = self.current_list_len();
        if len == 0 {
            self.selection = 0;
            self.list_state.select(None);
            return;
        }
        let next = self.selection as isize + delta;
        self.selection = next.clamp(0, len as isize - 1) as usize;
        self.list_state.select(Some(self.selection));
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    set_headless(true);

    let db = Database::open(&cli.db)
        .with_context(|| format!("Failed to open database {}", cli.db.display()))?;
    let mut app = App::new(&db)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app, &db);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn draw_ui(frame: &mut Frame, app: &mut App) {
    let size = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let tab_titles: Vec<Line> = Tab::titles()
        .iter()
        .map(|t| Line::from(Span::styled(*t, Style::default().fg(Color::Cyan))))
        .collect();
    let tabs = Tabs::new(tab_titles)
        .select(match app.tab {
            Tab::Items => 0,
            Tab::Upgrades => 1,
            Tab::Treasures => 2,
            Tab::Strategies => 3,
        })
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .style(Style::default().fg(Color::White));
    frame.render_widget(tabs, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)].as_ref())
        .split(chunks[1]);

    let items: Vec<ListItem> = match app.tab {
        Tab::Strategies => app
            .strategies
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                let label = format!(
                    "{} / {}  avg {:.1}%  win {:.1}% ({}/{})",
                    row.category,
                    row.name,
                    row.avg_clear_rate,
                    row.win_rate * 100.0,
                    row.win_count,
                    row.sample_count,
                );
                let content = vec![Line::from(Span::raw(label))];
                let mut item = ListItem::new(content);
                if idx == app.selection {
                    item = item.style(Style::default().fg(Color::Yellow));
                }
                item
            })
            .collect(),
        _ => app
            .current_list()
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                let label = format!(
                    "{}  avg {:.1}%  win {:.1}%",
                    row.name,
                    row.avg_clear_rate,
                    row.win_rate * 100.0
                );
                let content = vec![Line::from(Span::raw(label))];
                let mut item = ListItem::new(content);
                if idx == app.selection {
                    item = item.style(Style::default().fg(Color::Yellow));
                }
                item
            })
            .collect(),
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Summary"))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_stateful_widget(list, body_chunks[0], &mut app.list_state);

    let detail_text = if let Some(detail) = &app.detail {
        build_detail_text(detail)
    } else if let Tab::Strategies = app.tab {
        vec![Line::from(Span::raw(
            "Select a strategy on the left to view clear rate statistics.",
        ))]
    } else {
        vec![Line::from(Span::raw("No detail available."))]
    };

    let detail = Paragraph::new(detail_text)
        .block(Block::default().borders(Borders::ALL).title("Detail"))
        .wrap(Wrap { trim: true });
    frame.render_widget(detail, body_chunks[1]);
}

fn build_detail_text(detail: &DetailStats) -> Vec<Line<'_>> {
    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(
        detail.name.to_string(),
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::raw(format!(
        "Win rate: {:.1}%",
        detail.win_rate * 100.0
    ))));
    lines.push(Line::from(Span::raw(format!(
        "Selected simulations: {}/{}",
        detail.selected_simulations, detail.total_simulations
    ))));
    lines.push(Line::from(Span::raw(format!(
        "Total purchases: {}",
        detail.total_purchases
    ))));
    lines.push(Line::from(Span::raw(format!(
        "Average clear rate: {:.2}%",
        detail.avg_clear_rate
    ))));
    lines.push(Line::from(Span::raw(format!(
        "Clear rate variance: {:.2}",
        detail.clear_rate_variance
    ))));
    lines.push(Line::from(Span::raw(format!(
        "Clear rate samples: {}",
        detail.clear_rate_samples
    ))));
    lines.push(Line::from(Span::raw(format!(
        "Zero picks: {} (win {:.1}%)",
        detail.zero_pick_samples,
        detail.zero_pick_win_rate * 100.0
    ))));
    lines.push(Line::from(Span::raw("")));
    lines.push(Line::from(Span::styled(
        "Clear rate distribution",
        Style::default().add_modifier(Modifier::UNDERLINED),
    )));

    if detail.clear_rate_distribution.is_empty() {
        lines.push(Line::from(Span::raw("No clear rate samples.")));
    } else {
        let max_count = detail
            .clear_rate_distribution
            .iter()
            .map(|bin| bin.sample_count)
            .max()
            .unwrap_or(1)
            .max(1);
        let bar_width = 18;
        for bin in &detail.clear_rate_distribution {
            let bar_len = (bin.sample_count * bar_width + max_count / 2) / max_count;
            let bar = "█".repeat(bar_len);
            let padded_bar = format!("{:<width$}", bar, width = bar_width);
            lines.push(Line::from(Span::raw(format!(
                "{} | {} {}",
                bin.label, padded_bar, bin.sample_count
            ))));
        }
    }

    if !detail.distribution.is_empty() {
        lines.push(Line::from(Span::raw("")));
        lines.push(Line::from(Span::styled(
            "Pick count distribution",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )));

        for bin in &detail.distribution {
            lines.push(Line::from(Span::raw(format!(
                "{} picks: {:>4} sims, win {:.1}%",
                bin.count,
                bin.sample_count,
                bin.win_rate * 100.0
            ))));
        }
    }

    lines
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
    db: &Database,
) -> Result<()> {
    loop {
        terminal.draw(|f| draw_ui(f, app))?;

        if poll(Duration::from_millis(100))?
            && let CEvent::Key(key_event) = read()?
        {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Tab => {
                    app.next_tab();
                    app.refresh_detail(db)?;
                }
                KeyCode::Down => {
                    app.move_selection(1);
                    app.refresh_detail(db)?;
                }
                KeyCode::Up => {
                    app.move_selection(-1);
                    app.refresh_detail(db)?;
                }
                _ => {}
            }
        }
    }
}
