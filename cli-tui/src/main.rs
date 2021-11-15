use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::{io, vec};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::{Frame, Terminal};

#[derive(Clone)]

struct Pokemon {
    number: u16,
    name: String,
    types: Vec<String>,
}

#[derive(Clone)]

enum AppMode {
    View,
    Add,
}

#[derive(Clone)]
struct App {
    mode: AppMode,
    pokemons: Vec<Pokemon>,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: AppMode::View,
            pokemons: vec![],
        }
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();

    let res = run_app(&mut terminal, app);

    if let Err(error) = res {
        println!("{:?}", error)
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        app.pokemons = vec![Pokemon {
            number: 25,
            name: "Pikachu".to_string(),
            types: vec!["Electric".to_string()],
        }];

        terminal.draw(|f| ui(f, app.clone()))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: App) {
    let layout = Layout::default()
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .direction(Direction::Vertical)
        .split(f.size());

    let main_body = match app.mode {
        AppMode::View => {
            // Take all the pokemon and render then into a block
            let items: Vec<ListItem> = app
                .pokemons
                .into_iter()
                .map(|p| {
                    let name = Span::from(p.name);
                    ListItem::new(name).style(Style::default().fg(Color::Black).bg(Color::White))
                })
                .collect();

            List::new(items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ")
        }
        AppMode::Add => panic!("Not yet implemented"),
    };

    let toolbar = Block::default().borders(Borders::ALL);

    f.render_stateful_widget(main_body, layout[0], &mut ListState::default());
    f.render_widget(toolbar, layout[1]);
}
