# ccstat

A terminal statusline binary for [Claude Code](https://claude.ai/code) that displays API usage and context window stats.

It reads the rate-limit usage, context window fill, and current working directory from the JSON object Claude Code provides on stdin (via its statusline hook), and prints a compact statusline intended for use with tmux or similar multiplexers.

## Output format

```
5h ████████████░░░░░░░░ 42% 18:30 | 7d ██░░░░░░░░ 12% Wed 3h | Ctx ██░░░░░░░░ 18% | ~/src/myproject  main
```

- **5h** — 5-hour rolling usage window (bar + percentage + reset time)
- **7d** — 7-day rolling usage window
- **Ctx** — context window fill for the current conversation
- **Location** — current working directory and git branch

All data is read from the stdin JSON. Missing fields render as `--`.

## Build

```sh
cargo build --release
```

## Install

```sh
just install
```

Copies the binary to `~/.local/bin/ccstat`.

## Usage

```sh
ccstat [--dir <path>] [--prefix <label>]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--dir` | `/tmp/claude-statusline` | Directory for `log.txt` and the captured `stdin.json` |
| `--prefix` | _(none)_ | Optional label shown at the start of the line |
