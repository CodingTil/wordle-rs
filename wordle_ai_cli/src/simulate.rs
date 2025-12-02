use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{BarChart, Block, Borders, Paragraph},
};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wordle_ai::WordleAI;
use wordle_core::{Game, GuessResult};

use crate::common::{AIType, create_ai, get_wordlist};
use wordle_core::Language;

const MAX_ATTEMPTS: usize = 6;

#[derive(Debug, Clone)]
struct AgentStats {
    ai_type: AIType,
    wins: usize,
    losses: usize,
    guess_distribution: HashMap<usize, usize>, // guesses -> count
    total_guesses: usize,                      // for calculating average
}

impl AgentStats {
    fn new(ai_type: AIType) -> Self {
        Self {
            ai_type,
            wins: 0,
            losses: 0,
            guess_distribution: HashMap::new(),
            total_guesses: 0,
        }
    }

    fn record_win(&mut self, num_guesses: usize) {
        self.wins += 1;
        self.total_guesses += num_guesses;
        *self.guess_distribution.entry(num_guesses).or_insert(0) += 1;
    }

    fn record_loss(&mut self) {
        self.losses += 1;
    }

    fn win_rate(&self) -> f64 {
        if self.wins + self.losses == 0 {
            0.0
        } else {
            (self.wins as f64) / ((self.wins + self.losses) as f64) * 100.0
        }
    }

    fn avg_guesses(&self) -> f64 {
        if self.wins == 0 {
            0.0
        } else {
            (self.total_guesses as f64) / (self.wins as f64)
        }
    }

    fn min_guesses(&self) -> Option<usize> {
        self.guess_distribution.keys().min().copied()
    }

    fn max_guesses(&self) -> Option<usize> {
        self.guess_distribution.keys().max().copied()
    }
}

/// Simulate a single game with a given AI
fn simulate_game(ai: &mut Box<dyn WordleAI>, game: &Game) -> Option<usize> {
    let mut num_guesses = 0;
    let mut game = game.clone();

    loop {
        // Get AI's guess
        let guess = ai.make_guess()?;

        num_guesses += 1;

        // Submit guess to game
        match game.take_guess(&guess) {
            Ok(GuessResult::Won(_)) => {
                return Some(num_guesses);
            }
            Ok(GuessResult::Lost { .. }) => {
                return None;
            }
            Ok(GuessResult::Continue(result)) => {
                // Update AI with feedback
                ai.update(guess, result);
            }
            Err(_) => {
                // Word not in list - mark as invalid and try again
                ai.mark_invalid(guess);
            }
        }
    }
}

/// Run simulation for specified AI agents (parallelized)
pub fn run_simulation(num_games: usize, ai_types: Vec<AIType>, language: Language) -> Result<()> {
    println!("Starting simulation of {} games...", num_games);
    println!(
        "Testing AI agents: {}",
        ai_types
            .iter()
            .map(|ai| ai.name())
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Initialize stats for each AI wrapped in Arc<Mutex>
    let all_stats: Arc<Mutex<HashMap<AIType, AgentStats>>> = Arc::new(Mutex::new(
        ai_types
            .iter()
            .map(|&ai_type| (ai_type, AgentStats::new(ai_type)))
            .collect(),
    ));

    // Progress counter
    let progress = Arc::new(Mutex::new(0usize));

    // Run simulations in parallel
    (0..num_games).into_par_iter().for_each(|_| {
        // Update progress
        {
            let mut p = progress.lock().unwrap();
            *p += 1;
            if (*p).is_multiple_of(100) {
                println!("Progress: {}/{}", *p, num_games);
            }
        }

        let game = Game::new(MAX_ATTEMPTS, language).unwrap();

        // Each AI plays this game
        for &ai_type in &ai_types {
            let wordlist = get_wordlist(language).to_vec();
            let mut ai = create_ai(ai_type, wordlist);

            let result = simulate_game(&mut ai, &game);

            // Update stats
            let mut stats = all_stats.lock().unwrap();
            match result {
                Some(num_guesses) => {
                    stats.get_mut(&ai_type).unwrap().record_win(num_guesses);
                }
                None => {
                    stats.get_mut(&ai_type).unwrap().record_loss();
                }
            }
        }
    });

    println!("Simulation complete!");

    // Extract stats from Arc<Mutex>
    let final_stats = Arc::try_unwrap(all_stats).unwrap().into_inner().unwrap();

    // Display results in TUI
    let terminal = ratatui::init();
    let result = display_results(terminal, final_stats, num_games, &ai_types);
    ratatui::restore();
    result
}

fn display_results(
    mut terminal: DefaultTerminal,
    stats: HashMap<AIType, AgentStats>,
    num_games: usize,
    ai_types: &[AIType],
) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, &stats, num_games, ai_types))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    break Ok(());
                }
                _ => {}
            }
        }
    }
}

fn render(
    frame: &mut Frame,
    stats: &HashMap<AIType, AgentStats>,
    num_games: usize,
    ai_types: &[AIType],
) {
    let area = frame.area();

    let layout = Layout::vertical([
        Constraint::Length(3), // Title
        Constraint::Min(10),   // Stats
        Constraint::Length(3), // Help
    ])
    .split(area);

    // Title
    let title = Paragraph::new(format!("SIMULATION RESULTS ({} games)", num_games))
        .style(Style::default().fg(Color::White).bold())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, layout[0]);

    // Stats - split horizontally for each AI (dynamically)
    let num_agents = ai_types.len();

    // Dynamically create constraints for any number of agents
    let constraints: Vec<Constraint> = (0..num_agents)
        .map(|_| Constraint::Ratio(1, num_agents as u32))
        .collect();

    let stats_layout = Layout::horizontal(constraints).split(layout[1]);

    // Render each agent's stats (supports any number of agents)
    for (i, &ai_type) in ai_types.iter().enumerate() {
        if let Some(agent_stats) = stats.get(&ai_type) {
            render_agent_stats(frame, stats_layout[i], agent_stats);
        }
    }

    // Help
    let help = Paragraph::new("Press Q or Esc to quit")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, layout[2]);
}

fn render_agent_stats(frame: &mut Frame, area: Rect, stats: &AgentStats) {
    let layout = Layout::vertical([
        Constraint::Length(10), // Stats text
        Constraint::Min(5),     // Chart
    ])
    .split(area);

    // Stats text
    let win_rate = stats.win_rate();
    let avg_guesses = stats.avg_guesses();
    let min_guesses = stats
        .min_guesses()
        .map(|n| n.to_string())
        .unwrap_or_else(|| "N/A".to_string());
    let max_guesses = stats
        .max_guesses()
        .map(|n| n.to_string())
        .unwrap_or_else(|| "N/A".to_string());

    let text = vec![
        Line::from(""),
        Line::from(format!("Wins: {} ({:.1}%)", stats.wins, win_rate)),
        Line::from(format!("Losses: {}", stats.losses)),
        Line::from(""),
        Line::from(format!("Avg guesses: {:.2}", avg_guesses)),
        Line::from(format!("Min: {} | Max: {}", min_guesses, max_guesses)),
    ];

    let stats_widget = Paragraph::new(text).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .title(stats.ai_type.name()),
    );

    frame.render_widget(stats_widget, layout[0]);

    // Chart - guess distribution
    render_chart(frame, layout[1], stats);
}

fn render_chart(frame: &mut Frame, area: Rect, stats: &AgentStats) {
    // Prepare data for bar chart
    let mut data: Vec<(&str, u64)> = Vec::new();

    // Create labels for 1-6 guesses
    let labels = ["1", "2", "3", "4", "5", "6"];
    let mut counts = [0u64; 6];

    for guess_num in 1..=6 {
        if let Some(&count) = stats.guess_distribution.get(&guess_num) {
            counts[guess_num - 1] = count as u64;
        }
    }

    for (i, &label) in labels.iter().enumerate() {
        data.push((label, counts[i]));
    }

    let chart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Guess Distribution"),
        )
        .data(&data)
        .bar_width(3)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(Style::default().fg(Color::White).bold());

    frame.render_widget(chart, area);
}
