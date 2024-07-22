use std::io::{self, stdout};

use crate::{
    components::Component,
    pacman::{self, Package, Pacman},
};
use ratatui::{
    crossterm::{
        event::{KeyCode, KeyEvent, KeyModifiers},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand as _,
    },
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style, Stylize as _},
    widgets::{Block, Row, Table, TableState},
    Frame,
};

use crate::theme::Theme;

pub struct PackagesTable {
    // TODO
    // widget: Table<'a>,
    pacman: Pacman,
    pub state: TableState,
    pub packages: Vec<Package>,
    theme: Theme,
    pub active: bool,
    package: String,
}

impl Default for PackagesTable {
    fn default() -> Self {
        Self {
            // TODO: accept event that table is now active and remove this with selected stuff
            // in the event call select if any packages
            state: TableState::default().with_selected(Some(0)),
            pacman: Default::default(),
            packages: Default::default(),
            theme: Default::default(),
            active: Default::default(),
            package: Default::default(),
        }
    }
}

impl PackagesTable {
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.packages.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.packages.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn reset(&mut self) {
        self.state.select(Some(0));
    }

    fn install_package(&self) {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        pacman::install(
            &self
                .packages
                .get(self.state.selected().unwrap())
                .unwrap()
                .name,
        );
        stdout().execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
    }
}

impl Component for PackagesTable {
    fn handle_key_event(&mut self, event: KeyEvent) {
        if !self.active {
            match event {
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code,
                    ..
                } => {
                    // tODO: remain on the same package if it exists
                    self.reset();
                    match code {
                    KeyCode::Char(c) => {
                        self.package.push(c);
                    }
                    KeyCode::Backspace => {
                        self.package.pop();
                    }
                    _ => {}
                }
                },
                _ => {}
            }

            self.packages = self.pacman.search(&self.package);
            return;
        }

        match event {
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code,
                ..
            } => match code {
                KeyCode::Char('j') => self.next(),
                KeyCode::Char('k') => self.previous(),
                KeyCode::Char('i') => self.install_package(),
                _ => {}
            },
            _ => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> io::Result<()> {
        let mut rows = Vec::new();
        for package in &self.packages {
            rows.push(Row::new(vec![
                package.name.clone(),
                package.description.clone().unwrap_or("".to_string()),
            ]));
        }
        let widths = [Constraint::Percentage(25), Constraint::Percentage(65)];
        let header =
            Row::new(["name", "description"]).style(Style::new().bold().fg(Color::Magenta));
        let border_color = if self.active {
            self.theme.active
        } else {
            self.theme.inactive
        };
        let output = Table::new(rows, widths)
            .header(header)
            .block(Block::bordered().border_style(Style::default().fg(border_color)))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED));
        frame.render_stateful_widget(output, area, &mut self.state);
        Ok(())
    }
}
