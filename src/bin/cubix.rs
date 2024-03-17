use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

use rubiks::{view::MovesList, cube::{CubePath, Move}, cubelet::Axis};

#[derive(Default)]
struct App {
    cube: CubePath,
    // input: String,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::default();

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| ui(frame, &app))?;
        should_quit = handle_events(&mut app)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(app: &mut App) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => { return Ok(true); }
                    KeyCode::Char('q') => {
                        app.cube.make_move(Move(3, 0, Axis::Y));
                    }
                    KeyCode::Char('Q') => {
                        app.cube.make_move(Move(0, 3, Axis::Y));
                    }
                    KeyCode::Char('e') => {
                        app.cube.make_move(Move(1, 0, Axis::Y));
                    }
                    KeyCode::Char('E') => {
                        app.cube.make_move(Move(0, 1, Axis::Y));
                    }
                    KeyCode::Char('w') => {
                        app.cube.make_move(Move(3, 0, Axis::X));
                    }
                    KeyCode::Char('W') => {
                        app.cube.make_move(Move(0, 3, Axis::X));
                    }
                    KeyCode::Char('s') => {
                        app.cube.make_move(Move(1, 0, Axis::X));
                    }
                    KeyCode::Char('S') => {
                        app.cube.make_move(Move(0, 1, Axis::X));
                    }
                    KeyCode::Char('a') => {
                        app.cube.make_move(Move(3, 0, Axis::Z));
                    }
                    KeyCode::Char('A') => {
                        app.cube.make_move(Move(0, 3, Axis::Z));
                    }
                    KeyCode::Char('d') => {
                        app.cube.make_move(Move(1, 0, Axis::Z));
                    }
                    KeyCode::Char('D') => {
                        app.cube.make_move(Move(0, 1, Axis::Z));
                    }
                    KeyCode::Char('u') => {
                        if app.cube.moves.pop().is_some() {
                            app.cube.cubes.pop();
                        }
                    }
                    KeyCode::Char('R') => {
                        app.cube = CubePath::default();
                    }
                    _ => { }
                }
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, app: &App) {
    let main_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Length(350), Constraint::Min(40)]
    )
    .split(frame.size());
    let inner_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(80), Constraint::Min(15)]
    )
    .split(main_layout[0]);

    frame.render_widget(
        Paragraph::new(format!("{}", &app.cube.cubes[app.cube.cubes.len()-1]))
            .block(Block::default().title("Current state").borders(Borders::ALL)),
        inner_layout[0]
    );

    frame.render_widget(
        Paragraph::new(include_str!("instructions.txt"))
            .block(Block::default().title("Instructions").borders(Borders::ALL)),
        inner_layout[1]
    );

    frame.render_widget(
        Paragraph::new(format!("{}", MovesList(&app.cube.moves))).block(
            Block::default().title("Path").borders(Borders::ALL)
        ),
        main_layout[1]
    );
}
