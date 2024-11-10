use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

use rubiks_lab_rs::prelude::*;
use rubiks_lab_rs::cubelet;
use rubiks_lab_rs::view::DisplayCube;

#[derive(Default)]
struct App {
    tabs: Vec<Tab>,
    active_tab: usize,
    // store: Option<Store>,
    // input: String,
}

enum Tab {
    Moves(Word<Move>),
    Turns(Word<Turn>),
    QuarterTurns(Word<QuarterTurn>),
}

impl Tab {
    fn inner(&self) -> (&Cube<Position>, &Vec<Move>) {
        match self {
            Tab::Moves(Word { cube, actions, .. }) => (cube, actions),
            Tab::Turns(Word { cube, actions, .. }) => (cube, actions),
            Tab::QuarterTurns(Word { cube, actions, .. }) => (cube, actions),
        }
    }

    fn inner_mut(&mut self) -> (&mut Cube<Position>, &mut Vec<Move>) {
        match self {
            Tab::Moves(Word { cube, actions, .. }) => (cube, actions),
            Tab::Turns(Word { cube, actions, .. }) => (cube, actions),
            Tab::QuarterTurns(Word { cube, actions, .. }) => (cube, actions),
        }
    }

    fn make_move(&mut self, m: Move) {
        let (cube, actions) = self.inner_mut();
        *cube = cube.clone().make_move(m);
        actions.push(m);
    }

    fn pop(&mut self) -> Option<Move> {
        let (cube, actions) = self.inner_mut();
        if let Some(last_move) = actions.pop() {
            *cube = cube.clone().make_move(last_move.inverse());
            Some(last_move)
        } else {
            None
        }
    }

    fn cube(&self) -> &Cube<Position> {
        self.inner().0
    }

    fn to_string(&self) -> String {
        match self {
            Tab::Moves(word) => word.to_string(),
            Tab::Turns(word) => word.to_string(),
            Tab::QuarterTurns(word) => word.to_string(),
        }
    }

    fn normalize(&mut self) {
        match self {
            Tab::Moves(word) => { *word = word.clone().normal_form(); }
            Tab::Turns(word) => { *word = word.clone().normal_form(); }
            Tab::QuarterTurns(word) => { *word = word.clone().normal_form(); }
        }
    }
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
    app.tabs.push(Tab::Moves(Word::new()));

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
                    // Quit
                    KeyCode::Esc => { return Ok(true); }
                    // Make a move
                    KeyCode::Char('q') => {
                        app.active_mut().make_move(Move(cubelet::Axis::Y, 3, 0));
                        app.active_mut().normalize();
                    }
                    KeyCode::Char('Q') => {
                        app.active_mut().make_move(Move(cubelet::Axis::Y, 0, 3));
                        app.active_mut().normalize();
                    }
                    KeyCode::Char('e') => {
                        app.active_mut().make_move(Move(cubelet::Axis::Y, 1, 0));
                        app.active_mut().normalize();
                    }
                    KeyCode::Char('E') => {
                        app.active_mut().make_move(Move(cubelet::Axis::Y, 0, 1));
                        app.active_mut().normalize();
                    }
                    KeyCode::Char('w') => {
                        app.active_mut().make_move(Move(cubelet::Axis::X, 3, 0));
                        app.active_mut().normalize();
                    }
                    KeyCode::Char('W') => {
                        app.active_mut().make_move(Move(cubelet::Axis::X, 0, 3));
                        app.active_mut().normalize();
                    }
                    KeyCode::Char('s') => {
                        app.active_mut().make_move(Move(cubelet::Axis::X, 1, 0));
                        app.active_mut().normalize();
                    }
                    KeyCode::Char('S') => {
                        app.active_mut().make_move(Move(cubelet::Axis::X, 0, 1));
                        app.active_mut().normalize();
                    }
                    KeyCode::Char('a') => {
                        app.active_mut().make_move(Move(cubelet::Axis::Z, 3, 0));
                        app.active_mut().normalize();
                    }
                    KeyCode::Char('A') => {
                        app.active_mut().make_move(Move(cubelet::Axis::Z, 0, 3));
                        app.active_mut().normalize();
                    }
                    KeyCode::Char('d') => {
                        app.active_mut().make_move(Move(cubelet::Axis::Z, 1, 0));
                        app.active_mut().normalize();
                    }
                    KeyCode::Char('D') => {
                        app.active_mut().make_move(Move(cubelet::Axis::Z, 0, 1));
                        app.active_mut().normalize();
                    }
                    // undo
                    KeyCode::Char('u') => {
                        app.active_mut().pop();
                    }
                    // reset
                    KeyCode::Char('R') => {
                        *app.active_mut() = Tab::Moves(Word::new());
                    }
                    // new tab
                    KeyCode::Char('N') => {
                        app.tabs.push(Tab::Moves(Word::new()));
                        app.active_tab = app.tabs.len() - 1;
                    }
                    // Delete (kill) the current tab
                    KeyCode::Char('K') => {
                        app.tabs.remove(app.active_tab);
                        if app.tabs.is_empty() {
                            app.tabs.push(Tab::Moves(Word::new()));
                            app.active_tab = 0;
                        } else if app.active_tab >= app.tabs.len() {
                            app.active_tab = app.tabs.len() - 1;
                        }
                    }
                    // Change to moves
                    KeyCode::Char('m') => {
                        let (cube, actions) = app.active().inner();
                        *app.active_mut() = Tab::Moves(Word::from_parts_unchecked(cube.clone(), actions.clone()));
                    }
                    // Change to turns
                    KeyCode::Char('t') => {
                        let (cube, actions) = app.active().inner();
                        *app.active_mut() = Tab::Turns(Word::from_parts_unchecked(cube.clone(), actions.clone()));
                    }
                    // Change to quarter turns
                    KeyCode::Char('T') => {
                        let (cube, actions) = app.active().inner();
                        *app.active_mut() = Tab::QuarterTurns(Word::from_parts_unchecked(cube.clone(), actions.clone()));
                    }
                    // Move one tab to the left, wrapping around
                    KeyCode::Left => {
                        if app.active_tab == 0 {
                            app.active_tab = app.tabs.len() - 1;
                        } else {
                            app.active_tab -= 1;
                        }
                    }
                    // Move one tab to the right, wrapping around
                    KeyCode::Right => {
                        app.active_tab += 1;
                        if app.active_tab >= app.tabs.len() {
                            app.active_tab = 0;
                        }
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
            Paragraph::new(format!("{}", tab.cube()))
                .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
            *rect
        );
    });

    // Cube viewer
    frame.render_widget(
        Paragraph::new(format!("{}", DisplayCube(app.active().cube().clone())))
            .block(Block::default().title("Current state").borders(Borders::ALL)),
        inner_layout[0]
    );

    // Cubix instructions
    frame.render_widget(
        Paragraph::new(include_str!("instructions.txt"))
            .block(Block::default().title("Instructions").borders(Borders::ALL)),
        inner_layout[1]
    );

    // Word
    frame.render_widget(
        Paragraph::new(app.active().to_string())
            .block(
                Block::default().title("Word").borders(Borders::ALL)
            )
            .wrap(Wrap { trim: true }),
        big_layout[1]
    );
}
