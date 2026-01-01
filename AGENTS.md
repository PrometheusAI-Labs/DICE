# AGENTS.md

This file contains guidelines for agentic coding assistants working on the Telegram Dice Bot.

## Essential Commands

### Build & Run
```bash
cargo build                    # Build the project
cargo run                      # Run the bot (requires BOT_TOKEN env var)
RUST_LOG=debug cargo run       # Run with debug logging
```

### Code Quality
```bash
cargo fmt --all                # Format all code
cargo fmt -- --check           # Check formatting without changes
cargo clippy --all-targets --all-features -- -D warnings
                              # Lint with warnings as errors
cargo clippy --fix             # Auto-fix clippy suggestions
```

### Testing
```bash
cargo test --all-features --workspace    # Run all tests
cargo test <test_name>                   # Run single test
cargo test test_compare_dices_bot_wins   # Example: run specific test
cargo test properties                    # Run property-based tests
cargo test -- --nocapture                # Show test output
```

## Testing Guidelines

### Unit Tests
- Place tests in `#[cfg(test)] mod tests` modules at the bottom of files
- Use descriptive test names following `test_<function>_<scenario>` pattern
- For game logic, test all branches (win, lose, tie, edge cases)
- Use `pretty_assertions::assert_eq` for better failure messages

### Property-Based Tests
- Use `proptest` for testing invariants across all possible inputs
- Place property tests in a nested `mod properties` module
- Test ranges: dice values are `1u8..=6u8`
- Example: even/odd property should hold for all dice values

### Running Single Tests
```bash
cargo test test_check_even_odd_basic
cargo test test_compare_dices_bot_wins
cargo test properties::even_odd_property
```

## Code Formatting

- **Indentation**: 4 spaces (configured in .editorconfig)
- **Line endings**: LF only
- **Encoding**: UTF-8
- **Line length**: Prefer reasonable lengths, no strict limit
- **Trailing whitespace**: Always trim (except in .md files)
- **Final newline**: Required for all files

Run `cargo fmt --all` before committing. The pre-commit hook enforces this.

## Import Organization

Organize imports in this order:
1. Standard library (`std::*`)
2. External crates (teloxide, tokio, rand, log, etc.)
3. Local modules (`crate::*`)
4. Empty line before use statements in functions

Example:
```rust
use axum::{http::StatusCode, response::Html, routing::get, Router};
use log::{error, info};
use std::net::SocketAddr;
use teloxide::prelude::*;

mod bot;
mod game;
mod state;

use bot::BotHandler;
```

## Naming Conventions

### Functions & Variables
- **snake_case**: `handle_dice_message`, `user_dice`, `is_win`
- Descriptive names that clearly indicate purpose
- Avoid abbreviations unless widely understood

### Types & Structs
- **PascalCase**: `DiceGame`, `BotHandler`, `ChatId`
- Enums use PascalCase for variants: `Even`, `Odd`, `High`, `Low`
- Structs with no fields: use `pub struct BotHandler;`

### Constants
- **SCREAMING_SNAKE_CASE**: (not currently used, but preferred if needed)

### Module Names
- **snake_case**: `bot.rs`, `game.rs`, `state.rs`

## Type System Guidelines

### Enums for State
- Use enums to represent game states and user choices
- Derive `Clone`, `Debug`, `PartialEq` for all state enums
- For enums that may be serialized, add `Serialize`, `Deserialize`
- Implement `Default` for initial states

Example:
```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EvenOddChoice {
    Even,
    Odd,
}
```

### String Types
- Use `&'static str` for static strings (error messages, constants)
- Use `String` for dynamic strings (user input, formatted messages)
- Use `format!` for building messages with variables

### Type Annotations
- Add explicit type annotations for clarity in complex cases
- Type inference is preferred for simple cases
- Use `as u8` for casting dice values: `dice.value as u8`

## Error Handling

### Async Handlers
- Return `ResponseResult<()>` from all async Telegram handlers
- Use `?` operator to propagate errors
- Log errors with `error!` macro before returning

Example:
```rust
async fn start_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    info!("User {} started bot", msg.chat.id);
    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}
```

### Errors in Main
- Log errors with appropriate level (`error!`, `info!`)
- Handle environment variable parsing with `expect()` for required vars
- Use `unwrap_or_else()` with fallback for optional vars

Example:
```rust
let bot_token = std::env::var("BOT_TOKEN")
    .expect("BOT_TOKEN must be set");
let port = std::env::var("PORT")
    .unwrap_or_else(|_| "5000".to_string())
    .parse::<u16>()
    .expect("PORT must be a valid port number");
```

## Async/Await Guidelines

### Async Functions
- All Telegram handlers are `async fn`
- Use `tokio::time::sleep()` for delays (3 seconds for dice animation)
- Never block: use `.await` instead

### Spawned Tasks
- Use `tokio::spawn()` for concurrent tasks (HTTP server, bot)
- Use `tokio::select!` to wait for multiple tasks
- Handle task errors in the join results

Example:
```rust
let server_handle = tokio::spawn(async move {
    axum::serve(listener, app).await
});
tokio::select! {
    result = server_handle => { /* handle result */ }
}
```

## Teloxide-Specific Patterns

### Message Sending
- Chain builder methods: `.parse_mode()`, `.reply_markup()`, `.await?`
- Use HTML parsing for formatted messages: `.parse_mode(ParseMode::Html)`
- Use HTML tags: `<b>`, `<i>`, `<u>` for emphasis

### Inline Keyboards
- Use `InlineKeyboardMarkup::new()` with nested vecs
- Buttons in same row go in inner vec
- Callback data should be descriptive: `"game_even_odd"`, `"number_4"`

Example:
```rust
let keyboard = InlineKeyboardMarkup::new(vec![
    vec![InlineKeyboardButton::callback("1️⃣", "number_1")],
    vec![InlineKeyboardButton::callback("2️⃣", "number_2")],
]);
```

### Command Handling
- Use `#[derive(BotCommands)]` for command enums
- Match commands with `case![Command::Start].endpoint(Self::start_command)`
- Use lowercase for command names: `#[command(rename_rule = "lowercase")]`

### Dice Messages
- Check for dice with `if msg.dice().is_some()`
- Get value with `.unwrap().value as u8`
- Always wait 3 seconds after sending dice for animation

## Logging Guidelines

### Log Levels
- **info!**: Significant events (user actions, bot start/stop)
- **error!**: Errors and failures (request errors, parse failures)
- **warn!**: Warning conditions (deprecated usage)
- **debug!**: Detailed diagnostics (dice results, message IDs)

### Log Format
- Include relevant context: user ID, dice values, choices
- Use Russian for user-facing logs, English for technical logs
- Log both user actions and bot actions

Example:
```rust
info!("User {} started new game", msg.chat.id);
info!("User dice result: {}, Bot dice result: {}", user_dice, bot_dice);
error!("Unknown callback: {}", data);
```

## Game Logic Patterns

### Pure Functions
- Keep game logic in `game.rs` as pure functions
- Functions should be deterministic (no randomness in tests)
- Return simple types: `bool`, `&'static str`

### Randomness
- Use `rand::thread_rng()` for random number generation
- Use `rng.gen_range(1..=6)` for dice values
- Use random arrays for varied messages (win/lose)

### Emoji Guidelines
- **Use**: 🎱 (billiard ball), 🎲 (game die), 🎉, 🤖, 🤝
- **Avoid**: ⚀ (die face 1) - has display issues in some clients
- Use emojis consistently with game modes (🔵 for even/odd, etc.)

## File Structure

```
src/
├── main.rs      # Entry point, server setup
├── bot.rs       # Telegram handlers, commands, keyboards
├── game.rs      # Game logic, validation, tests
└── state.rs     # State enums, types
```

- `main.rs`: Bootstrapping, tokio runtime, HTTP server
- `bot.rs`: All Telegram interaction (500+ lines, split if larger)
- `game.rs`: Pure game logic with comprehensive tests
- `state.rs`: Type definitions, enums, structs

## Internationalization

### User-Facing Strings
- **Language**: Russian for all user-facing text
- **HTML formatting**: Use `<b>` for bold in messages
- **Emoji**: Include relevant emojis for visual context

### Comments & Documentation
- **Language**: Russian for code comments and doc comments
- **Function docs**: Use `///` for public functions
- **Clear descriptions**: Explain what, not just how

Example:
```rust
/// Проверка результата для игры "Четное/Нечетное"
pub fn check_even_odd(dice_result: u8, user_choice: EvenOddChoice) -> bool {
    let is_even = dice_result % 2 == 0;
    match user_choice {
        EvenOddChoice::Even => is_even,
        EvenOddChoice::Odd => !is_even,
    }
}
```

## Pre-Commit Workflow

The pre-commit hooks ensure code quality:
1. `cargo fmt --all` - Code must be formatted
2. `cargo clippy --all-targets --all-features -- -D warnings` - No warnings allowed
3. `cargo test --all-features --workspace` - All tests must pass

Before pushing, run:
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features --workspace
```

## Git Commit Guidelines

- Commit messages should be concise and descriptive
- Use conventional commits format: `type: description`
- Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`
- Examples: `feat: add dice battle mode`, `fix: resolve dice message handling`

## Bot Token & Environment

### Required Environment Variables
- `BOT_TOKEN`: Telegram bot token from @BotFather
- `PORT`: Health check port (default: 5000)

### .env File
- Development: Use `.env` file (gitignored, see `.env.example`)
- Production: Set environment variables directly
- Never commit `.env` or real tokens

## Testing Before Changes

Always run these commands before making changes:
```bash
cargo test --all-features --workspace    # Ensure baseline tests pass
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check
```

After making changes:
1. Run tests for affected modules
2. Run full test suite
3. Run clippy and fix warnings
4. Format code with `cargo fmt --all`

## Common Patterns

### Starting a Game
1. Show selection keyboard with `show_game_selection()`
2. User clicks callback button
3. Match callback in `handle_callback()`
4. Call specific game start function
5. Send dice with `bot.send_dice()`
6. Wait 3 seconds for animation
7. Get result and send result message

### Adding a New Game Mode
1. Add enum variant to state.rs
2. Add command or callback handler in bot.rs
3. Add keyboard in `show_game_selection()`
4. Add game logic function in game.rs
5. Add tests for the game logic
6. Update `/help` command description
7. Update README.md

### Adding a New Command
1. Add variant to `Command` enum in bot.rs
2. Add match branch in schema dispatcher
3. Implement handler function
4. Update `/help` command
5. Test the command

## Debugging Tips

### Enable Debug Logging
```bash
RUST_LOG=debug cargo run
```

### Check Telegram Updates
Monitor what messages the bot receives:
```bash
# Look for log lines showing dice results, user actions
# Use debug logs to trace message flow
```

### Test Locally
- Use your Telegram bot directly: @dice_game_tgbot
- Test all game modes and edge cases
- Verify dice animation timing
- Check keyboard buttons work correctly

## Performance Considerations

- Avoid blocking operations in async handlers
- Use `tokio::time::sleep()` for delays (not `std::thread::sleep`)
- Keep game logic pure and fast
- Cache message strings where appropriate
- Use `&str` instead of `String` for static text

## Security Notes

- Never log or expose the bot token
- Validate user input in callbacks
- Don't trust callback data blindly (parse and validate)
- Keep sensitive data in environment variables only
- Review third-party dependencies regularly with `cargo audit`

## Repository Info

- **GitHub**: https://github.com/PrometheusAI-Labs/DICE
- **Branch**: `main`
- **Language**: Rust 2021 edition
- **Framework**: teloxide 0.12
- **Maintainer**: Alxy Dev
