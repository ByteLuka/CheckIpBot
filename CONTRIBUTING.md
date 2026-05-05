# Contributing to CheckIpBot

Thank you for your interest in contributing! This document explains how to get involved.

## Scope

CheckIpBot is intentionally narrow in scope. If you want to add a new feature, **please open an issue for discussion before writing any code** — that way we can agree on whether it fits the project before you invest the time.

Bug fixes and small improvements can go straight to a pull request without an issue.

## Getting started

### Prerequisites

- Python 3.14+
- [uv](https://docs.astral.sh/uv/getting-started/installation/)
- A Discord bot token — see [docs/discord-token.md](docs/discord-token.md)

### Setup

```bash
git clone https://github.com/ByteLuka/CheckIpBot.git
cd CheckIpBot

uv sync --group dev
uv run pre-commit install
```

### Run locally

```bash
DISCORD_TOKEN=<your-token> uv run python bot.py
```

Set `LOG_LEVEL=DEBUG` for verbose output.

## Development workflow

1. Fork the repository
2. Create a branch from `master`: `git checkout -b my-fix`
3. Make your changes
4. Run the pre-commit checks: `uv run pre-commit run --all-files`
5. Open a pull request and fill in the template

Branches should be short-lived and focused on a single change.

## Code style

[ruff](https://docs.astral.sh/ruff/) handles both linting and formatting. Configuration lives in `pyproject.toml`.

```bash
uv run ruff check .           # lint
uv run ruff check --fix .     # lint with auto-fix
uv run ruff format .          # format
uv run ruff format --check .  # format check (no writes)
```

Pre-commit runs both checks automatically on every commit once installed.

## Adding a new command

1. Create a new file under `extensions/` (or add to an existing one) with an `Extension` subclass
2. Register it in `bot.py` with `bot.load_extension("extensions.<module_name>")`

See `extensions/ip_commands.py` for a minimal example.

For outbound HTTP, use `aiohttp.ClientSession` — it is already available as a transitive dependency. Do not add additional HTTP libraries.

## Pull request checklist

- [ ] Pre-commit hooks pass: `uv run pre-commit run --all-files`
- [ ] Tested locally with a real Discord token
- [ ] No unrelated changes included
- [ ] Issue linked if this resolves one

## Reporting bugs

Open a [GitHub issue](https://github.com/ByteLuka/CheckIpBot/issues) with:

- A clear description of the problem
- Steps to reproduce
- Expected vs. actual behaviour
- Bot version (image tag or git SHA)

## Releases

Releases are handled by maintainers. A new release is triggered by pushing a semver tag:

```bash
git tag v1.2.3 && git push --tags
```

This builds and publishes the Docker image to `ghcr.io/byteluka/checkipbot` and the Helm chart to `oci://ghcr.io/byteluka/charts/checkipbot`.
