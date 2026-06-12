# qhelp

A lightweight CLI tool for quick command lookups powered by a local LLM via [Ollama](https://ollama.com).

Ask natural language questions, get commands back. No cloud, no API keys, fully offline.

## Usage

```bash
# quick answer — just the command
qhelp "check disk space in GB on arch linux"

# explain mode — command + explanation
qhelp -e "\S+@\S+\.\S+ explain this regex"
```

## Example

```
$ qhelp "find files larger than 1GB"
find / -type f -size +1G
# Searches the entire filesystem for files exceeding 1GB

$ qhelp -e "git undo last commit but keep changes"
git reset --soft HEAD~1
# Moves HEAD back one commit while keeping all changes staged.
# --soft means your working directory and index stay untouched.
# You can then re-commit or modify the changes as needed.
```

## Setup

### 1. Install Ollama and pull a model

```bash
curl -fsSL https://ollama.com/install.sh | sh
ollama pull qwen2.5-coder:7b
```

### 2. Create config file

```bash
mkdir -p ~/.config/qhelp
```

```toml
# ~/.config/qhelp/config.toml
base_url = "http://localhost:11434"
model = "qwen2.5-coder:7b"
```

### 3. Build and install

```bash
cargo build --release
cp target/release/qhelp ~/.local/bin/
```

## Project Structure

```
src/
├── main.rs      # entry point, clap arg parsing
├── config.rs    # config file loading from XDG path
├── llm.rs       # Ollama API communication
└── prompt.rs    # system prompts for default and explain mode
```

## Configuration

qhelp reads its config from `~/.config/qhelp/config.toml`.

| Key        | Description              | Default                    |
|------------|--------------------------|----------------------------|
| `base_url` | Ollama API URL           | `http://localhost:11434`   |
| `model`    | Model name               | `qwen2.5-coder:7b`        |

## Requirements

- [Rust](https://rustup.rs/) (build)
- [Ollama](https://ollama.com) running locally with a pulled model

## License

MIT
