# wordle-rs

A Wordle game implementation in Rust with a terminal UI, supporting multiple languages.

## Playing the Game

Play in English (default):
```bash
cargo run -p wordle_cli
```

Play in German:
```bash
cargo run -p wordle_cli -- --language de
```

### Language Options
- `--language en` or `-l en` - English (default)
- `--language de` or `-l de` - German

## AI Solver

Run the AI assistant to help solve Wordle puzzles.

### Terminal AI Assistant

English (default):
```bash
cargo run -p wordle_ai_cli --release -- assistant
```

German:
```bash
cargo run -p wordle_ai_cli --release -- assistant --language de
```

You can also choose different AI strategies:
```bash
cargo run -p wordle_ai_cli --release -- assistant --ai heuristic --language en
```

AI options (`--ai` or `-a`):
- `heuristic` - Uses letter frequency analysis (default, recommended)
- `random-updates` - Random guessing with feedback filtering
- `random` - Pure random guessing
- `entropy` - Maximum information gain (slower but optimal)

### Web AI Assistant

Or use the browser version at [https://ai.wordle.tilmohr.com](https://ai.wordle.tilmohr.com).

Alternatively start a local web instance with:
```bash
cd wordle_ai_web
trunk serve --open
```

The web version includes a language selector dropdown to switch between English and German.

### AI Simulation

Evaluate and compare different AI strategies on simulated games.

English wordlist (default):
```bash
cargo run -p wordle_ai_cli --release -- simulate --num-games 1000
```

German wordlist:
```bash
cargo run -p wordle_ai_cli --release -- simulate --num-games 1000 --language de
```

Compare specific AI agents:
```bash
cargo run -p wordle_ai_cli --release -- simulate --num-games 1000 --ai heuristic --ai random-updates --language en
```

Simulation options:
- `--num-games` or `-n` - Number of games to simulate (default: 1000)
- `--ai` or `-a` - AI agents to test (can specify multiple, defaults to fast agents)
- `--language` or `-l` - Language wordlist to use (default: en)

## Supported Languages

- **English** (`en`) - 14,855 words
- **German** (`de`) - 6,832 words (including words with umlauts ä, ö, ü, ß)

All game modes (CLI game, AI assistant, web assistant, and simulation) support both languages!
