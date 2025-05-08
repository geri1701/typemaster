mod models;
use models::{Model, Page};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

trait Sandbox {
    fn view(&self, frame: &mut Frame);
    fn subscription(&mut self) -> bool {
        false
    }
    fn update(&mut self, event: Event) -> Option<bool>;
    fn run(&mut self) {
        let mut terminal = ratatui::init();
        let _ = terminal.draw(|frame| self.view(frame));
        loop {
            let mut update = self.subscription();
            if event::poll(std::time::Duration::from_millis(16)).unwrap() {
                if let Some(value) = self.update(event::read().unwrap()) {
                    update = update || value;
                } else {
                    ratatui::restore();
                    break;
                }
            }
            if update {
                let _ = terminal.draw(|frame| self.view(frame));
            }
        }
    }
}

impl Sandbox for models::Model {
    fn subscription(&mut self) -> bool {
        if let Page::Main = self.page {
            self.step_game();
            return true;
        };
        false
    }
    fn update(&mut self, event: Event) -> Option<bool> {
        if let Event::Key(keyevent) = event {
            return match keyevent.code {
                KeyCode::Tab => {
                    if let Page::Welcome = self.page {
                        self.level = self.level.switch();
                        return Some(true);
                    };
                    Some(false)
                }
                KeyCode::F(10) => {
                    if let Page::Welcome = self.page {
                        self.lang = self.lang.switch();
                        return Some(true);
                    };
                    Some(false)
                }
                KeyCode::F(12) => {
                    if let Page::Welcome = self.page {
                        self.page = Page::Licence;
                        return Some(true);
                    };
                    Some(false)
                }
                KeyCode::Char(value) => {
                    if let Page::Main = self.page {
                        self.letter(value);
                        return Some(true);
                    };
                    Some(false)
                }
                KeyCode::Enter => {
                    if let Page::Welcome = self.page {
                        self.page = Page::Main;
                        self.words = self.lang.wordlist();
                        self.step_game();
                        return Some(true);
                    };
                    Some(false)
                }
                KeyCode::Esc => {
                    if let Page::Main = self.page {
                        self.save_highscore();
                    };
                    self.page = self.page.esc();
                    if let Page::Quit = self.page {
                        return None;
                    };
                    Some(true)
                }
                _ => Some(false),
            };
        };
        Some(false)
    }
    fn view(&self, frame: &mut Frame) {
        match self.page {
            Page::Quit => {}
            Page::Licence => frame.render_widget(
                Paragraph::new(include_str!("../LICENSE"))
                    .style(Style::default().fg(Color::Yellow))
                    .block(
                        Block::default()
                            .padding(Padding::new(5, 10, 1, 2))
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Yellow))
                            .title(Line::from("[ LICENSE ]".bold()).centered())
                            .title_bottom(
                                Line::from(Vec::from([
                                    " Press ".into(),
                                    "<ESC>".blue().bold(),
                                    " to go back to Menu! ".into(),
                                ]))
                                .centered(),
                            )
                            .border_type(BorderType::Rounded),
                    ),
                frame.area(),
            ),
            Page::Welcome => {
                let outer = Block::bordered()
                    .border_style(Style::default().fg(Color::Yellow))
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .title(Line::from("[ WELCOME ]".bold()).centered());
                let [title, body] = Layout::vertical([Constraint::Length(6), Constraint::Min(0)])
                    .areas(outer.inner(frame.area()));
                let [_left, center, _right] = Layout::horizontal([
                    Constraint::Min(0),
                    Constraint::Length(45),
                    Constraint::Min(0),
                ])
                .areas(body);
                frame.render_widget(outer, frame.area());
                frame.render_widget(
                    Paragraph::new(
                        figleter::FIGfont::standard()
                            .unwrap()
                            .convert(Self::NAME)
                            .unwrap()
                            .to_string(),
                    )
                    .style(Style::default().fg(Color::Red))
                    .centered(),
                    title,
                );
                frame.render_widget(
                    Paragraph::new(format!(
                        r#"
Press <ENTER> to start
      <TAB>   to toggle difficulty: {}
      <F10>   to toggle language:   {}
      <F12>   to read license
      <ESC>   to quit

 Your Highscore:   {},
      Highest Cpm: {},
"#,
                        self.level.to_str(),
                        self.lang.to_str(),
                        self.highscore.0,
                        self.highscore.1,
                    ))
                    .style(Style::default().fg(Color::Green))
                    .block(Block::default().borders(Borders::NONE)),
                    center,
                );
            }
            Page::Main => {
                let outer = Block::bordered()
                    .border_style(Style::default().fg(Color::Yellow))
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .title(Line::from("[ PLAY ]".bold()).centered())
                    .title_bottom(
                        Line::from(format!(
                            "[ Score: {}; Chars / minute: {}; Words / minute: {}; (Highscore: {} Max-cpm: {}); Words {}; Letters {}; ]",
                            self.typed_chars,
                            self.cpm,
                            self.wpm,
                            self.highscore.0,
                            self.highscore.1,
                            self.words.len(),
                            self.scope.keys().len(),
                        ))
                        .centered(),
                    );
                frame.render_widget(outer, frame.area());
                for (word, x, y) in &self.list {
                    frame.render_widget(
                        Paragraph::new(Line::from(vec![
                            word[0..1].to_string().green().bold(),
                            word[1..word.len()].to_string().into(),
                        ])),
                        Rect::new(*x as u16, *y as u16, word.len() as u16, 1),
                    );
                }
            }
        };
    }
}

fn main() {
    Model::default().run();
}
