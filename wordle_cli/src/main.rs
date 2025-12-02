use clap::{Parser, ValueEnum};
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use wordle_core::{GameError, GuessResult, Language as CoreLanguage, LetterResult};

const MAX_ATTEMPTS: usize = 6;
const WORD_LENGTH: usize = 5;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Language {
    /// English (default)
    #[value(name = "en")]
    English,
    /// German
    #[value(name = "de")]
    German,
}

impl From<Language> for CoreLanguage {
    fn from(lang: Language) -> Self {
        match lang {
            Language::English => CoreLanguage::English,
            Language::German => CoreLanguage::German,
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "wordle_cli")]
#[command(about = "Play Wordle in the terminal", long_about = None)]
struct Args {
    /// Language to play in
    #[arg(short, long, value_enum, default_value_t = Language::English)]
    language: Language,
}

enum GameOutcome {
    Won,
    Lost { solution: [char; 5] },
}

struct App {
    game: wordle_core::Game,
    guesses: Vec<([char; 5], [LetterResult; 5])>,
    current_input: Vec<char>,
    error_message: Option<String>,
    outcome: Option<GameOutcome>,
}

impl App {
    fn new_game(language: CoreLanguage) -> Result<Self> {
        let game = wordle_core::Game::new(MAX_ATTEMPTS, language)
            .map_err(|_| color_eyre::eyre::eyre!("Failed to create game"))?;

        Ok(Self {
            game,
            guesses: Vec::new(),
            current_input: Vec::new(),
            error_message: None,
            outcome: None,
        })
    }

    fn is_playing(&self) -> bool {
        self.outcome.is_none()
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char(c) if c.is_alphabetic() && self.is_playing() => {
                if self.current_input.len() < WORD_LENGTH {
                    self.current_input
                        .push(c.to_lowercase().next().unwrap_or(c));
                    self.error_message = None;
                }
            }
            KeyCode::Backspace if self.is_playing() => {
                self.current_input.pop();
                self.error_message = None;
            }
            KeyCode::Enter if self.is_playing() => {
                self.submit_guess();
            }
            KeyCode::Char('r') | KeyCode::Char('R') if !self.is_playing() => {
                let language = self.game.language();
                *self = Self::new_game(language).unwrap();
            }
            _ => {}
        }
    }

    fn submit_guess(&mut self) {
        if self.current_input.len() != WORD_LENGTH {
            self.error_message = Some("Word must be 5 letters".to_string());
            return;
        }

        let guess: [char; 5] = self.current_input[..5].try_into().unwrap();

        match self.game.take_guess(&guess) {
            Ok(GuessResult::Continue(result)) => {
                self.guesses.push((guess, result));
                self.current_input.clear();
                self.error_message = None;
            }
            Ok(GuessResult::Won(result)) => {
                self.guesses.push((guess, result));
                self.current_input.clear();
                self.error_message = None;
                self.outcome = Some(GameOutcome::Won);
            }
            Ok(GuessResult::Lost {
                last_guess,
                solution,
            }) => {
                self.guesses.push((guess, last_guess));
                self.current_input.clear();
                self.error_message = None;
                self.outcome = Some(GameOutcome::Lost { solution });
            }
            Err(GameError::WordNotInList) => {
                self.error_message = Some("Word not in list".to_string());
            }
        }
    }
}

/// Display a character in uppercase, but preserve ß instead of converting to SS
fn uppercase_display(c: char) -> char {
    if c == 'ß' {
        c
    } else {
        c.to_uppercase().next().unwrap_or(c)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let language = args.language.into();

    let terminal = ratatui::init();
    let result = run(terminal, language);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, language: CoreLanguage) -> Result<()> {
    let mut app = App::new_game(language)?;

    loop {
        terminal.draw(|frame| render(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') && !app.is_playing()
            {
                break Ok(());
            }
            if key.code == KeyCode::Esc {
                break Ok(());
            }
            app.handle_key(key);
        }
    }
}

fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let layout = Layout::vertical([
        Constraint::Length(3), // Title
        Constraint::Min(15),   // Game board
        Constraint::Length(5), // Status/help
    ])
    .split(area);

    // Title
    let title = Paragraph::new("WORDLE")
        .style(Style::default().fg(Color::White).bold())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, layout[0]);

    // Game board
    render_game_board(frame, app, layout[1]);

    // Status and help
    render_status(frame, app, layout[2]);
}

fn render_game_board(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();

    // Render previous guesses
    for (guess, results) in &app.guesses {
        let spans: Vec<Span> = guess
            .iter()
            .zip(results.iter())
            .map(|(&ch, &result)| {
                let color = match result {
                    LetterResult::Correct => Color::Green,
                    LetterResult::Misplaced => Color::Yellow,
                    LetterResult::Absent => Color::DarkGray,
                };
                Span::styled(
                    format!(" {} ", uppercase_display(ch)),
                    Style::default().fg(Color::Black).bg(color).bold(),
                )
            })
            .collect();
        lines.push(Line::from(spans));
        lines.push(Line::from(""));
    }

    // Render current input (if still playing)
    if app.is_playing() {
        let mut current_spans = Vec::new();
        for i in 0..WORD_LENGTH {
            let ch = app.current_input.get(i).unwrap_or(&' ');
            current_spans.push(Span::styled(
                format!(" {} ", uppercase_display(*ch)),
                Style::default().fg(Color::White).bg(Color::DarkGray),
            ));
        }
        lines.push(Line::from(current_spans));
        lines.push(Line::from(""));
    }

    // Render remaining empty rows
    let remaining_rows = app
        .game
        .max_attempts()
        .saturating_sub(app.guesses.len() + if app.is_playing() { 1 } else { 0 });
    for _ in 0..remaining_rows {
        let empty_spans: Vec<Span> = (0..WORD_LENGTH)
            .map(|_| Span::styled("   ", Style::default().fg(Color::DarkGray).bg(Color::Black)))
            .collect();
        lines.push(Line::from(empty_spans));
        lines.push(Line::from(""));
    }

    let board = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Board"));

    frame.render_widget(board, area);
}

fn render_status(frame: &mut Frame, app: &App, area: Rect) {
    let text = match &app.outcome {
        Some(GameOutcome::Won) => {
            vec![
                Line::from(Span::styled(
                    "Congratulations! You won!",
                    Style::default().fg(Color::Green).bold(),
                )),
                Line::from(""),
                Line::from("Press 'R' to restart or 'Q' to quit"),
            ]
        }
        Some(GameOutcome::Lost { solution }) => {
            let solution_str: String = solution.iter().map(|&c| uppercase_display(c)).collect();
            vec![
                Line::from(Span::styled(
                    format!("Game Over! The word was: {}", solution_str),
                    Style::default().fg(Color::Red).bold(),
                )),
                Line::from(""),
                Line::from("Press 'R' to restart or 'Q' to quit"),
            ]
        }
        None => {
            let mut status_lines = vec![Line::from(format!(
                "Attempt {}/{}",
                app.game.attempts() + 1,
                app.game.max_attempts()
            ))];

            if let Some(ref error) = app.error_message {
                status_lines.push(Line::from(Span::styled(
                    error.clone(),
                    Style::default().fg(Color::Red),
                )));
            } else {
                status_lines.push(Line::from("Type a 5-letter word and press Enter"));
            }

            status_lines.push(Line::from("Press Esc to quit"));
            status_lines
        }
    };

    let status = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Status"));

    frame.render_widget(status, area);
}
