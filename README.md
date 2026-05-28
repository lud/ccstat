# ccstat

A lightweight, super fast statusline for [Claude Code](https://claude.ai/code)
that displays API usage and context window stats. It's intentionally minimal:
single static binary with no daemon, no config file, and a fixed layout. If you
need a customizable statusline, this isn't it.

## Output format


```
5h ████████████░░░░░░░░ 42% 18:30 | 7d ██░░░░░░░░ 12% Wed 3h | Ctx ██░░░░░░░░ 18% | ~/src/myproject  main
```

_Actual output uses different bar colors for elapsed session time and usage._

- **5h** — 5-hour rolling usage window (bar + percentage + reset time)
- **7d** — 7-day rolling usage window
- **Ctx** — context window fill for the current conversation
- **Location** — current working directory and git branch

All data is read from the stdin JSON that Claude Code provides on each
statusline refresh. Missing fields render as `--`.

## Installation

Download the archive for your platform from the
[latest release](https://github.com/lud/ccstat/releases/latest):

- `ccstat-x86_64-unknown-linux-gnu.tar.gz` — Linux (x86_64)
- `ccstat-aarch64-unknown-linux-gnu.tar.gz` — Linux (arm64)
- `ccstat-x86_64-apple-darwin.tar.gz` — macOS (Intel)
- `ccstat-aarch64-apple-darwin.tar.gz` — macOS (Apple Silicon)

Extract the `ccstat` binary and place it anywhere on your `PATH`. For example, using `~/.local/bin`:

```sh
mkdir -p ~/.local/bin
curl -L https://github.com/lud/ccstat/releases/latest/download/ccstat-x86_64-unknown-linux-gnu.tar.gz \
  | tar -xz -C ~/.local/bin ccstat
chmod +x ~/.local/bin/ccstat
```

Then enable it as the Claude Code statusline by adding this to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "ccstat"
  }
}
```

If `ccstat` is not on your `PATH`, use the absolute path instead (e.g. `/home/you/.local/bin/ccstat`).

## Usage

```sh
ccstat [--dir <path>] [--prefix <label>]
```

| Flag       | Default                  | Description                                           |
| ---------- | ------------------------ | ----------------------------------------------------- |
| `--dir`    | `/tmp/claude-statusline` | Directory for `log.txt` and the captured `stdin.json` |
| `--prefix` | _(none)_                 | Optional label shown at the start of the line         |

## Contributing

Build from source:

```sh
cargo build --release
```

Build and copy the binary to `~/.local/bin/ccstat`:

```sh
just install
```

Cut a release (requires [`cargo-release`](https://github.com/crate-ci/cargo-release)):

```sh
just release <major|minor|patch|x.y.z>
```
