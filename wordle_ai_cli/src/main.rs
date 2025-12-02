mod assistant;
mod common;
mod simulate;

use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre::Result;
use common::AIType;
use wordle_core::Language as CoreLanguage;

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
#[command(name = "wordle_ai_cli")]
#[command(about = "Wordle AI - assistant and simulator", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the AI assistant to help you solve Wordle puzzles
    Assistant {
        /// Which AI agent to use
        #[arg(short, long, value_enum, default_value_t = AIType::Heuristic)]
        ai: AIType,

        /// Language to play in
        #[arg(short, long, value_enum, default_value_t = Language::English)]
        language: Language,
    },
    /// Simulate games and compare AI performance
    Simulate {
        /// Number of games to simulate
        #[arg(short, long, default_value_t = 1000)]
        num_games: usize,

        /// Which AI agents to test (can specify multiple)
        /// Default: Random, RandomUpdates, Heuristic (Entropy excluded due to slowness)
        #[arg(short, long, value_enum)]
        ai: Vec<AIType>,

        /// Language to play in
        #[arg(short, long, value_enum, default_value_t = Language::English)]
        language: Language,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.command {
        Commands::Assistant { ai, language } => {
            assistant::run_assistant(ai, language.into())?;
        }
        Commands::Simulate {
            num_games,
            ai,
            language,
        } => {
            // Default to fast AIs if none specified
            let ai_types = if ai.is_empty() {
                vec![AIType::Random, AIType::RandomUpdates, AIType::Heuristic]
            } else {
                ai
            };
            simulate::run_simulation(num_games, ai_types, language.into())?;
        }
    }

    Ok(())
}
