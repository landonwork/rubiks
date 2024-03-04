use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

use rubiks::rubiks::Cube;

#[derive(Default)]
struct App {
    cube: Cube,
    input: String
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let app = App::default();

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui(frame, &app))?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, app: &App) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(40), Constraint::Min(15)]
    )
        .split(frame.size());
    frame.render_widget(
        Paragraph::new("Hello World!").block(
            Block::default().title("Greeting").borders(Borders::ALL)
        ),
        main_layout[0],
    );
}
