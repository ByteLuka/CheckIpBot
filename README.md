# CheckIpBot

A Discord bot that exposes your server's public IP address as a slash command.

![Lint](https://github.com/ByteLuka/CheckIpBot/actions/workflows/lint.yml/badge.svg)
![Release](https://github.com/ByteLuka/CheckIpBot/actions/workflows/release.yml/badge.svg)
![Latest Release](https://img.shields.io/github/v/release/ByteLuka/CheckIpBot)

## Table of Contents

- [Commands](#commands)
- [Installation (Docker Compose)](#installation-docker-compose)
- [Installation (Kubernetes)](#installation-kubernetes)
- [Development (local)](#development-local)
- [Contributing](#contributing)

## Commands

| Command   | Description                                               |
|-----------|-----------------------------------------------------------|
| `/get-ip` | Returns the public IP address of the host running the bot |

## Installation (Docker Compose)

### Prerequisites

- Docker with Compose (v2)
- A Discord bot token — see [docs/discord-token.md](docs/discord-token.md) if you need to create one

### Run

1. Download the provided `docker-compose.yml`
2. Replace `your_discord_token_here` with your bot token
3. Start the bot:

```bash
docker compose up -d
```

Optionally set `LOG_LEVEL` to `DEBUG`, `WARNING`, `ERROR`, or `CRITICAL` (default is `INFO`).

---

## Installation (Kubernetes)

### Prerequisites

- Kubernetes cluster with Helm 3.8+
- A Discord bot token — see [docs/discord-token.md](docs/discord-token.md) if you need to create one

### Install

```bash
helm upgrade --install checkipbot oci://ghcr.io/byteluka/charts/checkipbot \
  --version <chart-version> \
  --set discord.token=<your-token>
```

If you prefer to manage the token as a pre-existing Kubernetes secret (recommended for production), create the secret first:

```bash
kubectl create secret generic checkipbot-token --from-literal=DISCORD_TOKEN=<your-token>
```

Then install referencing it:

```bash
helm upgrade --install checkipbot oci://ghcr.io/byteluka/charts/checkipbot \
  --version <chart-version> \
  --set discord.existingSecret=checkipbot-token
```

### Upgrade

```bash
helm upgrade checkipbot oci://ghcr.io/byteluka/charts/checkipbot \
  --version <new-chart-version> \
  --reuse-values
```

### Configuration

| Value                    | Default                       | Description                                                 |
|--------------------------|-------------------------------|-------------------------------------------------------------|
| `image.repository`       | `ghcr.io/byteluka/checkipbot` | Container image repository                                  |
| `image.tag`              | *(chart appVersion)*          | Image tag to deploy                                         |
| `image.pullPolicy`       | `IfNotPresent`                | Kubernetes image pull policy                                |
| `discord.token`          | `""`                          | Bot token (creates a new secret)                            |
| `discord.existingSecret` | `""`                          | Name of a pre-existing secret with a `DISCORD_TOKEN` key    |
| `logLevel`               | `INFO`                        | Log level (`DEBUG`, `INFO`, `WARNING`, `ERROR`, `CRITICAL`) |
| `resources`              | `{}`                          | Container resource requests and limits                      |
| `nodeSelector`           | `{}`                          | Node selector                                               |
| `tolerations`            | `[]`                          | Tolerations                                                 |
| `affinity`               | `{}`                          | Affinity rules                                              |

---

## Development (local)

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain, including `rustfmt` and `clippy`)
- [pre-commit](https://pre-commit.com/#install)
- A Discord bot token — see [docs/discord-token.md](docs/discord-token.md)

### Setup

```bash
git clone https://github.com/ByteLuka/CheckIpBot.git
cd CheckIpBot

# Install pre-commit hooks
pre-commit install
```

### Run

```bash
DISCORD_TOKEN=<your-token> cargo run
```

Set `LOG_LEVEL=DEBUG` for verbose output:

```bash
LOG_LEVEL=DEBUG DISCORD_TOKEN=<your-token> cargo run
```

### Lint and format

```bash
cargo clippy --all-targets -- -D warnings  # lint
cargo fmt                                  # format
cargo fmt --check                          # format check (no writes)
```

Pre-commit runs both automatically on every commit once installed.

### Adding a new command

1. Create a new module under `src/commands/` with a `register` function returning the `CreateCommand` and a `run` function handling the interaction
2. Add it to `all()` in `src/commands/mod.rs` and dispatch it in `interaction_create` in `src/main.rs`

See `src/commands/ip.rs` for a minimal example.

---

## Contributing

Contributions are welcome. This bot is intentionally narrow in scope, so if you have an idea for a new feature, **please open an issue for discussion before writing any code** — that way we can agree on whether it fits before you invest the time.

Bug fixes and small improvements can go straight to a pull request.

### Workflow

1. Fork the repository and create a branch from `master`
2. Make your changes
3. Ensure pre-commit hooks pass: `pre-commit run --all-files`
4. Open a pull request — fill in the template

### Releases

Releases are handled by maintainers. A new release is triggered by pushing a semver tag:

```bash
git tag v1.2.3 && git push --tags
```

This builds and publishes the Docker image to `ghcr.io/byteluka/checkipbot` and the Helm chart to `oci://ghcr.io/byteluka/charts/checkipbot`.
