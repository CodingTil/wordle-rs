use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use wordle_ai::WordleAI;
use wordle_core::LetterResult;

use crate::common::{AIType, WORD_LENGTH, WORDLIST_ARRAY, create_ai};

enum FeedbackInputState {
    EnteringFeedback {
        current_position: usize,
        feedback: [Option<LetterResult>; 5],
    },
    WaitingForNextWord,
}

struct App {
    ai: Box<dyn WordleAI>,
    ai_type: AIType,
    current_recommendation: Option<[char; 5]>,
    feedback_state: FeedbackInputState,
    history: Vec<([char; 5], [LetterResult; 5])>,
    error_message: Option<String>,
    info_message: Option<String>,
}

impl App {
    fn new(ai_type: AIType) -> Self {
        let wordlist = WORDLIST_ARRAY.to_vec();
        let mut ai = create_ai(ai_type, wordlist);
        let current_recommendation = ai.make_guess();

        Self {
            ai,
            ai_type,
            current_recommendation,
            feedback_state: FeedbackInputState::WaitingForNextWord,
            history: Vec::new(),
            error_message: None,
            info_message: None,
        }
    }

    /// Cycle through feedback options
    /// - forward: Absent -> Misplaced -> Correct -> Absent
    /// - backward: Absent -> Correct -> Misplaced -> Absent
    fn cycle_feedback(current: Option<LetterResult>, forward: bool) -> LetterResult {
        match (current, forward) {
            (None, true) => LetterResult::Misplaced,
            (Some(LetterResult::Absent), true) => LetterResult::Misplaced,
            (Some(LetterResult::Misplaced), true) => LetterResult::Correct,
            (Some(LetterResult::Correct), true) => LetterResult::Absent,

            (None, false) => LetterResult::Absent,
            (Some(LetterResult::Absent), false) => LetterResult::Correct,
            (Some(LetterResult::Misplaced), false) => LetterResult::Absent,
            (Some(LetterResult::Correct), false) => LetterResult::Misplaced,
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        // Handle feedback submission separately to avoid borrow checker issues
        if let FeedbackInputState::EnteringFeedback { feedback, .. } = &self.feedback_state
            && key.code == KeyCode::Enter
            && feedback.iter().all(|f| f.is_some())
        {
            let feedback_copy = *feedback;
            self.submit_feedback(feedback_copy);
            return;
        }

        match &mut self.feedback_state {
            FeedbackInputState::WaitingForNextWord => {
                match key.code {
                    KeyCode::Enter => {
                        // Start entering feedback for current recommendation
                        if self.current_recommendation.is_some() {
                            self.feedback_state = FeedbackInputState::EnteringFeedback {
                                current_position: 0,
                                feedback: [None; 5],
                            };
                            self.error_message = None;
                            self.info_message = Some("Use ←/→ to navigate, ↑/↓ or C/M/A to set feedback, Enter to submit, N to mark word as not in list".to_string());
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        // Mark word as not in list
                        if let Some(word) = self.current_recommendation {
                            let word_str: String = word.iter().collect();
                            self.info_message =
                                Some(format!("Word '{}' marked as not in list", word_str));
                            // Mark as invalid so AI won't suggest it again
                            self.ai.mark_invalid(word);
                            // Get next recommendation
                            self.current_recommendation = self.ai.make_guess();
                            if self.current_recommendation.is_none() {
                                self.error_message =
                                    Some("AI has no more valid words to suggest!".to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
            FeedbackInputState::EnteringFeedback {
                current_position,
                feedback,
            } => {
                match key.code {
                    KeyCode::Left => {
                        if *current_position > 0 {
                            *current_position -= 1;
                        }
                        self.error_message = None;
                    }
                    KeyCode::Right => {
                        if *current_position < WORD_LENGTH - 1 {
                            *current_position += 1;
                        }
                        self.error_message = None;
                    }
                    KeyCode::Up => {
                        feedback[*current_position] =
                            Some(Self::cycle_feedback(feedback[*current_position], true));
                        self.error_message = None;
                    }
                    KeyCode::Down => {
                        feedback[*current_position] =
                            Some(Self::cycle_feedback(feedback[*current_position], false));
                        self.error_message = None;
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        feedback[*current_position] = Some(LetterResult::Correct);
                        self.error_message = None;
                    }
                    KeyCode::Char('m') | KeyCode::Char('M') => {
                        feedback[*current_position] = Some(LetterResult::Misplaced);
                        self.error_message = None;
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        feedback[*current_position] = Some(LetterResult::Absent);
                        self.error_message = None;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        // Mark word as not in list and cancel feedback
                        if let Some(word) = self.current_recommendation {
                            let word_str: String = word.iter().collect();
                            self.info_message =
                                Some(format!("Word '{}' marked as not in list", word_str));
                            // Mark as invalid so AI won't suggest it again
                            self.ai.mark_invalid(word);
                            // Get next recommendation
                            self.current_recommendation = self.ai.make_guess();
                            self.feedback_state = FeedbackInputState::WaitingForNextWord;
                            if self.current_recommendation.is_none() {
                                self.error_message =
                                    Some("AI has no more valid words to suggest!".to_string());
                            }
                        }
                    }
                    KeyCode::Enter => {
                        // Check if all feedback is set
                        if !feedback.iter().all(|f| f.is_some()) {
                            self.error_message =
                                Some("Please set feedback for all letters".to_string());
                        }
                        // Actual submission is handled above to avoid borrow checker issues
                    }
                    KeyCode::Esc => {
                        // Cancel feedback input
                        self.feedback_state = FeedbackInputState::WaitingForNextWord;
                        self.error_message = None;
                        self.info_message = None;
                    }
                    _ => {}
                }
            }
        }
    }

    fn submit_feedback(&mut self, feedback: [Option<LetterResult>; 5]) {
        if let Some(word) = self.current_recommendation {
            let feedback_unwrapped: [LetterResult; 5] = [
                feedback[0].unwrap(),
                feedback[1].unwrap(),
                feedback[2].unwrap(),
                feedback[3].unwrap(),
                feedback[4].unwrap(),
            ];

            // Check if won (all correct)
            if feedback_unwrapped
                .iter()
                .all(|&f| f == LetterResult::Correct)
            {
                self.history.push((word, feedback_unwrapped));
                self.info_message =
                    Some("Congratulations! You won! Press Q to quit or R to restart.".to_string());
                self.current_recommendation = None;
                self.feedback_state = FeedbackInputState::WaitingForNextWord;
                return;
            }

            // Update AI with feedback
            self.ai.update(word, feedback_unwrapped);
            self.history.push((word, feedback_unwrapped));

            // Get next recommendation
            self.current_recommendation = self.ai.make_guess();
            self.feedback_state = FeedbackInputState::WaitingForNextWord;
            self.error_message = None;
            self.info_message = None;

            if self.current_recommendation.is_none() {
                self.error_message = Some("AI has no more words to suggest!".to_string());
            }
        }
    }

    fn reset(&mut self) {
        let wordlist = WORDLIST_ARRAY.to_vec();
        let mut ai = create_ai(self.ai_type, wordlist);
        let current_recommendation = ai.make_guess();

        self.ai = ai;
        self.current_recommendation = current_recommendation;
        self.feedback_state = FeedbackInputState::WaitingForNextWord;
        self.history.clear();
        self.error_message = None;
        self.info_message = None;
    }
}

pub fn run_assistant(ai_type: AIType) -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal, ai_type);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, ai_type: AIType) -> Result<()> {
    let mut app = App::new(ai_type);

    loop {
        terminal.draw(|frame| render(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                break Ok(());
            }
            if key.code == KeyCode::Char('r') || key.code == KeyCode::Char('R') {
                app.reset();
                continue;
            }
            if key.code == KeyCode::Esc
                && matches!(app.feedback_state, FeedbackInputState::WaitingForNextWord)
            {
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
        Constraint::Length(5), // Current recommendation
        Constraint::Min(8),    // History
        Constraint::Length(7), // Status/help
    ])
    .split(area);

    // Title
    let title = Paragraph::new("WORDLE AI ASSISTANT")
        .style(Style::default().fg(Color::White).bold())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, layout[0]);

    // Current recommendation
    render_recommendation(frame, app, layout[1]);

    // History
    render_history(frame, app, layout[2]);

    // Status and help
    render_status(frame, app, layout[3]);
}

fn render_recommendation(frame: &mut Frame, app: &App, area: Rect) {
    let lines = if let Some(word) = app.current_recommendation {
        match &app.feedback_state {
            FeedbackInputState::WaitingForNextWord => {
                // Show word in simple format
                let spans: Vec<Span> = word
                    .iter()
                    .map(|&ch| {
                        Span::styled(
                            format!(" {} ", ch.to_uppercase()),
                            Style::default().fg(Color::White).bg(Color::Blue).bold(),
                        )
                    })
                    .collect();
                vec![Line::from(""), Line::from(spans)]
            }
            FeedbackInputState::EnteringFeedback {
                current_position,
                feedback,
            } => {
                // Show word with feedback colors and highlight current position
                let spans: Vec<Span> = word
                    .iter()
                    .enumerate()
                    .map(|(i, &ch)| {
                        let (bg_color, border_char_before, border_char_after) =
                            if i == *current_position {
                                // Current position - highlight
                                match feedback[i] {
                                    None => (Color::DarkGray, '<', '>'),
                                    Some(LetterResult::Correct) => (Color::Green, '<', '>'),
                                    Some(LetterResult::Misplaced) => (Color::Yellow, '<', '>'),
                                    Some(LetterResult::Absent) => (Color::Black, '<', '>'),
                                }
                            } else {
                                // Not current position
                                match feedback[i] {
                                    None => (Color::DarkGray, ' ', ' '),
                                    Some(LetterResult::Correct) => (Color::Green, ' ', ' '),
                                    Some(LetterResult::Misplaced) => (Color::Yellow, ' ', ' '),
                                    Some(LetterResult::Absent) => (Color::Black, ' ', ' '),
                                }
                            };
                        Span::styled(
                            format!(
                                "{}{}{}",
                                border_char_before,
                                ch.to_uppercase(),
                                border_char_after
                            ),
                            Style::default()
                                .fg(if bg_color == Color::Black {
                                    Color::White
                                } else {
                                    Color::Black
                                })
                                .bg(bg_color)
                                .bold(),
                        )
                    })
                    .collect();
                vec![Line::from(""), Line::from(spans)]
            }
        }
    } else {
        vec![
            Line::from(""),
            Line::from("No more recommendations available"),
        ]
    };

    let recommendation = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .title("AI Recommendation"),
    );

    frame.render_widget(recommendation, area);
}

fn render_history(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();

    // Show last few guesses
    let start_idx = app.history.len().saturating_sub(6);
    for (guess, results) in app.history.iter().skip(start_idx) {
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
                    format!(" {} ", ch.to_uppercase()),
                    Style::default().fg(Color::Black).bg(color).bold(),
                )
            })
            .collect();
        lines.push(Line::from(spans));
    }

    if lines.is_empty() {
        lines.push(Line::from("No guesses yet"));
    }

    let history = Paragraph::new(lines).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .title("History (Last 6)"),
    );

    frame.render_widget(history, area);
}

fn render_status(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();

    // Show info or error message
    if let Some(ref error) = app.error_message {
        lines.push(Line::from(Span::styled(
            error.clone(),
            Style::default().fg(Color::Red).bold(),
        )));
    } else if let Some(ref info) = app.info_message {
        lines.push(Line::from(Span::styled(
            info.clone(),
            Style::default().fg(Color::Yellow),
        )));
    }

    // Show instructions based on state
    match &app.feedback_state {
        FeedbackInputState::WaitingForNextWord => {
            if app.current_recommendation.is_some() {
                lines.push(Line::from(""));
                lines.push(Line::from("Press Enter to enter feedback for this word"));
                lines.push(Line::from("Press 'N' to mark word as not in list"));
            }
            lines.push(Line::from("Press 'R' to restart, 'Q' or Esc to quit"));
        }
        FeedbackInputState::EnteringFeedback { .. } => {
            lines.push(Line::from(""));
            lines.push(Line::from(
                "←/→: Navigate | ↑/↓: Cycle feedback | C/M/A: Correct/Misplaced/Absent",
            ));
            lines.push(Line::from(
                "Enter: Submit feedback | N: Not in list | Esc: Cancel",
            ));
        }
    }

    let status = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Status"));

    frame.render_widget(status, area);
}
