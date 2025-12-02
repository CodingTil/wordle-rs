# wordle-rs

A Wordle game implementation in Rust with a terminal UI.

## Playing the Game
```bash
cargo run -p wordle_cli
```

## AI Solver
Run the AI assistant using:
```bash
cargo run -p wordle_ai_cli --release -- assistant
```

Or in the browser at [https://ai.wordle.tilmohr.com](https://ai.wordle.tilmohr.com).

Altnernatively start a local web instance with:
```bash
cd wordle_ai_web
trunk serve --open
```

### AI Simulation
Different AI strategies can be evaluated on simulated games:
```bash
cargo run -p wordle_ai_cli -- simulate
```
