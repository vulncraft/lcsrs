# lcsrs — Spec (v1)

A CLI tool for spaced-repetition scheduling of LeetCode-style programming problems.

## Purpose

Pick the next problem to re-solve based on FSRS, and record how well that attempt went. Single-user, local-first, terminal-only.

## Algorithm

- **FSRS** via the [`fsrs`](https://crates.io/crates/fsrs) crate.
- **Pointer model**: each card is a reference to an external problem (title + URL). The tool does not store problem statements, solutions, or notes. The "review" is the user re-solving the problem; the rating reflects how that attempt went.
- **One card per problem.** No multi-card-per-note split.
- **Desired retention**: 0.9 (FSRS default). Hardcoded in v1; no config file yet.
- **FSRS parameters**: crate defaults. No parameter optimization in v1, but full review history is persisted so an `optimize` command can be added later.

## Storage

- **Backend**: SQLite via `rusqlite` with the `bundled` feature (no system libsqlite dependency, single static binary).
- **Location**:
  - Default: `$XDG_DATA_HOME/lcsrs/lcsrs.db`, falling back to `~/.local/share/lcsrs/lcsrs.db`. Resolved via the `directories` crate.
  - Override: `LCSRS_DB` environment variable (full path to the `.db` file).
  - No `--db` flag.
- **Schema migrations**: managed via `rusqlite_migration` from day one.

### Schema

```sql
CREATE TABLE cards (
    id           INTEGER PRIMARY KEY,
    title        TEXT NOT NULL,
    url          TEXT NOT NULL UNIQUE,
    difficulty   TEXT CHECK (difficulty IN ('easy', 'medium', 'hard')),
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- FSRS state
    due          TIMESTAMP NOT NULL,
    stability    REAL NOT NULL,
    difficulty_f REAL NOT NULL,        -- FSRS "difficulty" (separate from LC difficulty)
    last_review  TIMESTAMP,
    reps         INTEGER NOT NULL DEFAULT 0,
    lapses       INTEGER NOT NULL DEFAULT 0,
    state        TEXT NOT NULL          -- FSRS state enum: New / Learning / Review / Relearning
);

CREATE TABLE tags (
    id    INTEGER PRIMARY KEY,
    name  TEXT NOT NULL UNIQUE
);

CREATE TABLE card_tags (
    card_id  INTEGER NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    tag_id   INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (card_id, tag_id)
);

CREATE TABLE reviews (
    id            INTEGER PRIMARY KEY,
    card_id       INTEGER NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    rating        TEXT NOT NULL CHECK (rating IN ('again', 'hard', 'good', 'easy')),
    reviewed_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_cards_due ON cards(due);
```

All timestamps stored in **UTC**. Convert at the CLI boundary using `chrono::Local`.

### URL normalization

Before insert and uniqueness check:
- Lowercase the host.
- Strip a trailing `/`.
- Strip any URL fragment (`#...`).
- Leave the rest untouched.

## CLI

Argument parsing via `clap` with derive macros. Five commands in v1.

### `lcsrs add <title> <url> [flags]`

Add a new problem to the database.

Flags:
- `--difficulty <easy|medium|hard>` — optional
- `--tag <name>` — repeatable for multiple tags; tags are created on demand
- `--solved <YYYY-MM-DD>` — backfill: register the card as if it had been successfully reviewed (`good`) on that date. Synthesizes a single `reviews` row at that timestamp and runs FSRS once to seed `stability`, `difficulty_f`, `due`, etc.

Without `--solved`: card enters as `New`, due immediately (`due = now()`).

Duplicate `url` (after normalization) → hard error:
```
Error: card already exists with id 42 ("Two Sum").
Use 'lcsrs delete 42' first if you want to re-add.
```

### `lcsrs next [flags]`

Print the single next card to review. Exit 0.

Flags:
- `--tag <name>` — restrict to cards with this tag (repeatable; AND semantics)
- `--difficulty <easy|medium|hard>` — restrict to this difficulty

Ordering:
1. Cards with `due <= now()` (in local TZ), most overdue first (`ORDER BY due ASC`).
2. Then `New`-state cards (never reviewed), oldest `created_at` first.

Output:
```
[42] Two Sum  (medium)  due 2 days ago
https://leetcode.com/problems/two-sum
tags: array, hash-table
```

If no card matches the filter and there are no due/new cards:
```
Nothing due. Next card ("Valid Parentheses") due in 3 days.
```
Exit 0.

### `lcsrs grade <id> <again|hard|good|easy>`

Record a review for a card.

- `<id>`: the integer id, or any unambiguous prefix (e.g. `4` matches `4`, but if both `4` and `42` exist you must type `42` in full and `4` matches only `4` exactly).
- Runs the FSRS state transition with the given rating, updates the card row, and inserts a `reviews` row with `reviewed_at = now()`.

Output:
```
Next review: 2026-06-08 (15 days)
```

### `lcsrs delete <id>`

Delete a card and its tag links and review history (cascade). Prints `Deleted card [42] "Two Sum".` Exits 0. No confirmation prompt.

### `lcsrs list [flags]`

List cards in a tabular format.

Flags:
- `--due` — only cards with `due <= now()`
- `--tag <name>` — repeatable, AND semantics
- `--difficulty <easy|medium|hard>`

Output: id, title, difficulty, due (relative), tags. Sorted by `due ASC`.

## Semantics & defaults

- **Timezone**: timestamps stored UTC; "due today" means `due <= now()` in local TZ. No day-rollover fudge.
- **Daily caps**: none. User controls volume via `add`.
- **Tag input**: repeated `--tag X --tag Y` flags. Tag names are lowercased; whitespace trimmed.
- **Output**: plain text, no `--json` mode in v1. No color.
- **Errors**: print to stderr, exit non-zero. No stack traces in user-facing output.

## Out of scope for v1

- `edit` command — workaround is `delete` + `add` (preserves no state, accepted tradeoff).
- `show` command — single-card detail view (fold into `list` if needed).
- `import` / `export` — SQLite file is the backup.
- `stats` — retention, streaks, etc.
- `optimize` — re-fit FSRS parameters from review history.
- Config file — retention and any other tunables are hardcoded.
- Multi-source (CodeForces, AoC, ...) — tool is leetcode-specific by name.
- Per-card notes / solution hints — incompatible with pointer-model recall.
- `--json` output / scripting hooks.
- TUI / interactive review session — explicitly rejected in favor of stateless commands.

## Deferred for future versions

Data already supports these without schema change:
- `optimize` — uses `reviews` table.
- `stats` — uses `reviews` table.
- `--last` sugar on `grade` — would require a "last shown" pointer; adds state, deliberately deferred.

Would require schema change:
- Per-card notes (add column).
- Multi-source (add `source` column or per-source URL prefix).
- New cards/day cap (add config + new-cards-introduced-today counter).
