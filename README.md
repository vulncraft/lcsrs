# lcsrs


A CLI for spaced-repetition scheduling of LeetCode-style problems. Uses [FSRS](https://github.com/open-spaced-repetition) to decide which problem to re-solve next based on how well your past attempts went.

The tool doesn't store problem statements or solutions — it stores a pointer (title + URL) and your review history. When a card comes due, you go solve the problem again on LeetCode (or wherever), then grade your attempt.

## Install

```sh
cargo build --release
# binary at target/release/lcsrs
```

The database lives at `$XDG_DATA_HOME/lcsrs/lcsrs.db` (typically `~/.local/share/lcsrs/lcsrs.db`). Override with `LCSRS_DB=/path/to/file.db`.

## Daily flow

```sh
$ lcsrs next
[7] Valid Parentheses  (easy)  due 2 days ago
https://leetcode.com/problems/valid-parentheses
tags: stack

# ... go solve it ...

$ lcsrs grade 7 good
Next review: 2026-06-08 (15 days)
```

## Commands

### `add` — record a new problem

```sh
lcsrs add "Two Sum" https://leetcode.com/problems/two-sum \
    --difficulty medium \
    --tag array --tag hash-table
```

Flags:
- `--difficulty <easy|medium|hard>` — optional.
- `--tag <name>` — repeat for multiple tags. Tags are created on demand and stored lowercased.
- `--solved <YYYY-MM-DD>` — backfill. Treats the card as if you'd successfully reviewed it (`good`) on that date. Useful for seeding problems you already solved before adopting the tool.

The card's URL must be unique (normalized: lowercased host, trailing slash stripped, fragment stripped). Adding a duplicate is a hard error.

### `next` — what should I review now?

```sh
lcsrs next
lcsrs next --tag graph
lcsrs next --difficulty hard --tag dp
```

Prints one card: the most overdue first, then new cards (oldest first). If the filter excludes everything currently due, prints when the next matching card will be due.

### `grade` — record a review outcome

```sh
lcsrs grade 7 good
lcsrs grade 7 again   # blanked / had to look it up
lcsrs grade 7 hard    # solved but slow or with hints
lcsrs grade 7 easy    # trivial, faster than last time
```

The id is the integer shown by `next`/`list`. Any unambiguous prefix also works (e.g., `lcsrs grade 4 good` matches card 4 if 4 is the only card whose id starts with `4`).

### `list` — show cards in a table

```sh
lcsrs list
lcsrs list --due                    # only cards due now or earlier
lcsrs list --tag graph --difficulty hard
```

Output is sorted by due date (oldest first).

### `delete` — remove a card

```sh
lcsrs delete 7
```

Cascades to that card's tag links and review history. No confirmation prompt.

## Tips

- **Tag conventions**: lowercase, hyphenated (`two-pointer`, `binary-search`). Tags are normalized to lowercase on insert.
- **Backup**: the SQLite file is everything. `cp $LCSRS_DB ~/backups/lcsrs-$(date +%F).db` if you want safety.
- **Inspect the DB**: `sqlite3 $LCSRS_DB` — the schema is in [`SPEC.md`](./SPEC.md).
- **Fix a typo**: there's no `edit` command yet. `lcsrs delete <id>` then `lcsrs add ...` (you'll lose review history for that card).

## How the scheduling works

[FSRS](https://github.com/open-spaced-repetition) (Free Spaced Repetition Scheduler) tracks each card's *stability* (how long the memory lasts) and *difficulty* (how hard the material is for you), updating both after every review. The next-due date is computed to land when your predicted retention drops to 90%.

- **Default retention target**: 0.9 (90% recall when due). Hardcoded.
- **Parameters**: the FSRS default weights — not yet tuned to your personal review history.

Review history is persisted (`reviews` table) so a future `lcsrs optimize` command can re-fit the FSRS parameters once you've accumulated enough data.

## See also

- [`SPEC.md`](./SPEC.md) — the full design spec, including the schema and what's deliberately out of scope for v1.

## Disclaimer

This application was fully vibecoded.
