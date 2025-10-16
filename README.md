# redash-tool

Rust CLI for version controlling Redash queries and dashboards.

## Features

- Version control Redash queries (SQL + metadata)
- Automatic query discovery from local files
- Automatic deployment to Redash
- Built in Rust for blazing fast performance
- Pedantic code quality with clippy pre-commit hooks

## Prerequisites

- Rust 1.89+ with 2024 edition support
- Redash API key from https://sql.telemetry.mozilla.org

## Installation

```bash
cargo build --release
# The binary will be at ./target/release/redash-tool
```

## Setup

1. Get your Redash API key from your user profile

2. Set environment variables:
```bash
export REDASH_API_KEY="your-api-key-here"
export REDASH_URL="https://sql.telemetry.mozilla.org"  # optional, this is the default
```

3. Create directories:
```bash
redash-tool init
```

4. Discover available queries:
```bash
redash-tool discover
```

5. Fetch specific queries:
```bash
redash-tool fetch 123 456 789
```

## Usage

### Fetch Queries from Redash

```bash
redash-tool fetch --all        # Fetch all tracked queries
redash-tool fetch 123 456 789  # Fetch specific queries
redash-tool discover            # List available queries
```

This creates/updates:
- `queries/{id}-{slug}.sql` - Query SQL
- `queries/{id}-{slug}.yaml` - Query metadata (parameters, visualizations, etc.)

### Deploy to Redash

```bash
redash-tool deploy       # Deploy changed queries (detected via git status)
redash-tool deploy --all # Deploy all queries
```

**Warning**: This force overwrites the queries in Redash. Git is the source of truth.

## File Structure

```
queries/
├── 123-mobile-crashes.sql
└── 123-mobile-crashes.yaml
```

Query IDs are embedded in filenames (`{id}-{slug}.{ext}`), so no separate config file is needed.

## Development

### Pre-commit Hooks

The project uses clippy in pedantic mode:

```bash
cargo clippy --all-targets --all-features -- -W clippy::pedantic -D warnings
```

Install pre-commit hooks:
```bash
pip install pre-commit
pre-commit install
```

### Building for Release

```bash
cargo build --release
./target/release/redash-tool --help
```

## Architecture

See [CLAUDE.md](./CLAUDE.md) for detailed architecture documentation.
