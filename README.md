# ccstat

A terminal statusline binary for [Claude Code](https://claude.ai/code) that displays API usage and context window stats.

It reads Claude's credentials from `~/.claude/.credentials.json`, fetches usage from the Anthropic API (cached for 15 minutes), and prints a compact statusline intended for use with tmux or similar multiplexers.

## Output format

```
5h ████████████░░░░░░░░ 42% 18:30 | 7d ██░░░░░░░░ 12% Wed 3h | Ctx ██░░░░░░░░ 18% | ~/src/myproject  main
```

- **5h** — 5-hour rolling usage window (bar + percentage + reset time)
- **7d** — 7-day rolling usage window
- **Ctx** — context window fill for the current conversation (read from stdin JSON)
- **Location** — current working directory and git branch (read from stdin JSON)

Input is a JSON object on stdin (as provided by Claude Code's statusline hook).

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
ccstat [--dir <path>] [--creds <path>] [--prefix <label>]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--dir` | `/tmp/claude-statusline` | Directory for `usage.cache` and `log.txt` |
| `--creds` | `~/.claude/.credentials.json` | Path to Claude credentials file |
| `--prefix` | _(none)_ | Optional label shown at the start of the line |
