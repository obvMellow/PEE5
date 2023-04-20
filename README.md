# PEE5

[![Rust](https://github.com/obvMellow/PEE5/actions/workflows/rust.yml/badge.svg)](https://github.com/obvMellow/PEE5/actions/workflows/rust.yml)

**Disclaimer:** This bot is still in heavy development

Completely free and open-source, *blazingly fast* Discord bot that aims to be an alternative to MEE6

## Usage

### Adding the bot the your server

You can add the bot using this [link](https://top.gg/bot/1087464844288069722?s=051495d9e370e)

### Self hosting

Also you can self host the bot.

#### Dependencies
-   [Cargo (Automatically installed with setup script)](https://www.rust-lang.org/tools/install)
-   [Python3 (Required for setup.py)](https://www.python.org/downloads/)

#### Setting up
-   Clone this repository
```sh
git clone https://github.com/obvMellow/PEE5.git && cd PEE5
```
-   Run setup script
```sh
python3 setup.py
```
-   Follow the instructions
-   Run the bot
```sh
cargo run --release
```
-   Done!

## Developing

Any contribution to the project is appreciated.
If you want to help the development of this bot, please follow these guidelines.

### Code Style

Just follow the [Rust code style guide](https://doc.rust-lang.org/beta/style-guide/index.html).

### General Structure

When developing, keep the general structure of the code as following.

For slash commands:
- All command files must be in `src/commands`.
- The main command name should be `run`.
- Command can have side effects.
- The command call should be like the following:
```rust
let result: Result<()> = match command.data.name.as_str() {
    "example" => commands::example::run(&ctx, &command).await, // Arguments for the function can change.
    _ => Ok(()),
}
```
[Click to see the actual code used in production.](https://github.com/obvMellow/PEE5/blob/master/src/main.rs#L33)

For plugins:
- All plugin files must be in `src/plugins`.
- Main command name should be `run`.
- Plugin can have side effects.
- Plugin should be called in a place that it makes sense.
For example, if the plugin is a command, it should be called on `message` event in the proper spot.

Finally just create a pull request and I will review it!
