# PR Checker

<div align="left">

[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

</div>

A simple, configurable GitHub Action to validate pull request rules. Built with Rust for reliability and performance.

## Features

- ✅ **PR Title Validation** - Enforce title patterns (e.g., Conventional Commits) and length constraints
- ✅ **Required Labels** - Ensure PRs have specific labels before merging
- ✅ **GitHub Annotations** - Clear error messages directly in PR checks
- ✅ **Zero Configuration** - Works out of the box with sensible defaults
- ✅ **Fast & Reliable** - Built with Rust, runs in seconds

## Quick Start

### 1. Add the Action to your workflow (Required)

Create `.github/workflows/pr-check-ci.yml`:

```yml
name: PR Check

on:
  pull_request:
    types: [opened, synchronize, reopened, labeled, unlabeled, edited]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: PR Checker
        uses: chenjjiaa/pr-checker@v0.1.0
        with:
          config: .github/pr-checker.yml
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### 2. Create configuration file (Optional)

Create `.github/pr-checker.yml`.

> If you don't create this file, the default configuration will be loaded from [https://github.com/chenjjiaa/pr-checker/blob/main/pr-checker.yml](https://github.com/chenjjiaa/pr-checker/blob/main/pr-checker.yml).

```yaml
title:
  pattern: "^(feat|fix|docs|chore|refactor|test|style|perf|ci|build|revert|Bump)(\\([^)]+\\))?:"
  min_length: 10

labels:
  required: []
```

That's it! The Action will now validate all PRs against your rules.

## Configuration

### Title Rules

```yaml
title:
  # Regex pattern to match (optional)
  pattern: "^(feat|fix|docs|chore):"

  # Minimum length (optional)
  min_length: 10

  # Maximum length (optional)
  max_length: 100
```

### Label Rules

```yaml
labels:
  # List of required labels (optional)
  required:
    - "kind/bug"
    - "priority/high"
```

### Complete Example

```yaml
title:
  pattern: "^(feat|fix|docs|chore|refactor|test|style|perf|ci|build|revert|Bump)(\\([^)]+\\))?:"
  min_length: 10
  max_length: 100

labels:
  required:
    - "kind/bug"
```

## How It Works

1. The Action reads the PR event from `GITHUB_EVENT_PATH`
2. Fetches PR details via GitHub REST API
3. Validates against configured rules
4. Outputs GitHub annotations for any violations
5. Exits with code `1` if validation fails, `0` if all checks pass

## Exit Codes

| Code | Meaning             |
| ---- | ------------------- |
| 0    | All checks passed   |
| 1    | Validation failed   |
| 2    | Configuration error |
| 3    | GitHub API error    |
| 10   | Internal error      |

## Examples

### Example 1: Conventional Commits

```yaml
title:
  pattern: "^(feat|fix|docs|chore|refactor|test|style|perf|ci|build|revert|Bump)(\\([^)]+\\))?:"
  min_length: 10
```

**Valid titles:**

- `feat: add user authentication`
- `fix: resolve memory leak`
- `docs: update API documentation`

**Invalid titles:**

- `add user authentication` (missing prefix)
- `feat: fix` (too short)

### Example 2: Required Labels

```yaml
labels:
  required:
    - "kind/bug"
    - "priority/high"
```

PRs must have both `kind/bug` and `priority/high` labels.

## Inputs

| Input    | Description         | Required | Default                  |
| -------- | ------------------- | -------- | ------------------------ |
| `config` | Path to config file | No       | `.github/pr-checker.yml` |

## Environment Variables

| Variable            | Description                 | Required                 |
| ------------------- | --------------------------- | ------------------------ |
| `GITHUB_TOKEN`      | GitHub token for API access | Yes                      |
| `GITHUB_EVENT_PATH` | Path to GitHub event JSON   | Yes (auto-set by GitHub) |

## Development

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test
```

### Docker Build

```bash
docker build -t pr-checker .
```

### Run Locally

```bash
export GITHUB_TOKEN=your_token
export GITHUB_EVENT_PATH=/path/to/event.json
cargo run -- --config .github/pr-checker.yml
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Author

chenjjiaa
