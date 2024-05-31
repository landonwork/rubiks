use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

use rubiks::{
    cube::{CubePath, Move},
    cubelet::Axis,
    // store::Store,
    view::{DisplayCube, MovesList}
};

#[derive(Default)]
struct App {
    tabs: Vec<Tab>,
    active_tab: usize,
    // store: Option<Store>,
    // input: String,
}

#[derive(Default)]
struct Tab {
    cube: CubePath,
    // simplified_cube: CubePath,
}

impl App {
    fn active(&self) -> &Tab {
        self.tabs.get(self.active_tab).unwrap()
    }

    fn active_mut(&mut self) -> &mut Tab {
        self.tabs.get_mut(self.active_tab).unwrap()
    }
}


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::default();
    app.tabs.push(Tab::default());

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
                        app.active_mut().cube.make_move(Move(3, 0, Axis::Y));
                    }
                    KeyCode::Char('Q') => {
                        app.active_mut().cube.make_move(Move(0, 3, Axis::Y));
                    }
                    KeyCode::Char('e') => {
                        app.active_mut().cube.make_move(Move(1, 0, Axis::Y));
                    }
                    KeyCode::Char('E') => {
                        app.active_mut().cube.make_move(Move(0, 1, Axis::Y));
                    }
                    KeyCode::Char('w') => {
                        app.active_mut().cube.make_move(Move(3, 0, Axis::X));
                    }
                    KeyCode::Char('W') => {
                        app.active_mut().cube.make_move(Move(0, 3, Axis::X));
                    }
                    KeyCode::Char('s') => {
                        app.active_mut().cube.make_move(Move(1, 0, Axis::X));
                    }
                    KeyCode::Char('S') => {
                        app.active_mut().cube.make_move(Move(0, 1, Axis::X));
                    }
                    KeyCode::Char('a') => {
                        app.active_mut().cube.make_move(Move(3, 0, Axis::Z));
                    }
                    KeyCode::Char('A') => {
                        app.active_mut().cube.make_move(Move(0, 3, Axis::Z));
                    }
                    KeyCode::Char('d') => {
                        app.active_mut().cube.make_move(Move(1, 0, Axis::Z));
                    }
                    KeyCode::Char('D') => {
                        app.active_mut().cube.make_move(Move(0, 1, Axis::Z));
                    }
                    // undo
                    KeyCode::Char('u') => {
                        app.active_mut().cube.pop();
                    }
                    // reset
                    KeyCode::Char('R') => {
                        app.active_mut().cube = CubePath::default();
                    }
                    // new tab
                    // KeyCode::Char('N') => {
                    //     app.active_mut().cube = CubePath::default();
                    // }
                    _ => { }
                }
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, app: &App) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Max(1), Constraint::Fill(1)]
    ).split(frame.size());

    let big_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Length(350), Constraint::Min(40)]
    ).split(main_layout[1]);

    let tabs_layout = Layout::new(
        Direction::Horizontal,
        app.tabs.iter().map(|_| Constraint::Max(21))
    ).split(main_layout[0]);

    let inner_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(7), Constraint::Min(15)]
    ).split(big_layout[0]);

    // Tabs
    app.tabs.iter().zip(tabs_layout.iter()).for_each(|(tab, rect)| {
        frame.render_widget(
            Paragraph::new(format!("{}", tab.cube.last_cube()))
                .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
            *rect
        );
    });

    // Cube viewer
    frame.render_widget(
        Paragraph::new(format!("{}", DisplayCube(app.active().cube.last_cube().clone())))
            .block(Block::default().title("Current state").borders(Borders::ALL)),
        inner_layout[0]
    );

    // Cubix instructions
    frame.render_widget(
        Paragraph::new(include_str!("instructions.txt"))
            .block(Block::default().title("Instructions").borders(Borders::ALL)),
        inner_layout[1]
    );

    // Path
    frame.render_widget(
        Paragraph::new(format!("{}", MovesList(&app.active().cube.moves))).block(
            Block::default().title("Path").borders(Borders::ALL)
        ),
        big_layout[1]
    );
}
