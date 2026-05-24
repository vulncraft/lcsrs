use anyhow::{Context, Result, anyhow, bail};
use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeZone, Utc};
use rusqlite::{Connection, OptionalExtension, params, params_from_iter};

use crate::cli::{AddArgs, DeleteArgs, Difficulty, FilterArgs, GradeArgs, ListArgs, RatingArg};
use crate::ids;
use crate::scheduler::{self, CardState, CurrentState, Rating};
use crate::url_norm;

pub fn add(conn: &mut Connection, args: AddArgs) -> Result<()> {
    let url = url_norm::normalize(&args.url)?;
    let difficulty_str = args.difficulty.map(|d| d.as_str());

    if let Some((id, title)) = find_by_url(conn, &url)? {
        bail!(
            "card already exists with id {id} (\"{title}\").\n\
             Use 'lcsrs delete {id}' first if you want to re-add."
        );
    }

    let now = Utc::now();
    let (state, outcome_now) = match args.solved {
        None => {
            let s = scheduler::new_card(now);
            (
                s,
                ComputedNow {
                    stability: 0.0,
                    difficulty_f: 0.0,
                    due: now,
                    last_review: None,
                    state: CardState::New,
                    reps: 0,
                    lapses: 0,
                },
            )
        }
        Some(date) => {
            let solved_at = local_date_to_utc(date)?;
            let seed = scheduler::new_card(solved_at);
            let outcome = scheduler::apply(&seed, Rating::Good, solved_at)?;
            (
                seed,
                ComputedNow {
                    stability: outcome.stability,
                    difficulty_f: outcome.difficulty_f,
                    due: outcome.due,
                    last_review: Some(solved_at),
                    state: outcome.state,
                    reps: outcome.reps,
                    lapses: outcome.lapses,
                },
            )
        }
    };
    let _ = state;

    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO cards (title, url, difficulty, due, stability, difficulty_f, last_review, reps, lapses, state)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            args.title,
            url,
            difficulty_str,
            outcome_now.due,
            outcome_now.stability,
            outcome_now.difficulty_f,
            outcome_now.last_review,
            outcome_now.reps,
            outcome_now.lapses,
            outcome_now.state.as_str(),
        ],
    )?;
    let card_id = tx.last_insert_rowid();

    if let Some(date) = args.solved {
        let solved_at = local_date_to_utc(date)?;
        tx.execute(
            "INSERT INTO reviews (card_id, rating, reviewed_at) VALUES (?1, 'good', ?2)",
            params![card_id, solved_at],
        )?;
    }

    for tag in normalize_tags(&args.tags) {
        attach_tag(&tx, card_id, &tag)?;
    }

    tx.commit()?;
    println!("Added card [{card_id}] \"{}\".", args.title);
    Ok(())
}

pub fn next(conn: &Connection, args: FilterArgs) -> Result<()> {
    let now = Utc::now();
    let (mut sql, mut params_v) = base_select();
    sql.push_str(" WHERE due <= ?");
    params_v.push(Box::new(now));
    apply_filters(&mut sql, &mut params_v, &args.tags, args.difficulty);
    sql.push_str(" ORDER BY (state = 'new') ASC, due ASC LIMIT 1");

    if let Some(card) = query_one(conn, &sql, &params_v)? {
        print_next(&card, now, conn)?;
        return Ok(());
    }

    let (mut sql, mut params_v) = base_select();
    sql.push_str(" WHERE 1=1");
    apply_filters(&mut sql, &mut params_v, &args.tags, args.difficulty);
    sql.push_str(" ORDER BY due ASC LIMIT 1");
    match query_one(conn, &sql, &params_v)? {
        Some(card) => {
            println!(
                "Nothing due. Next card (\"{}\") {}.",
                card.title,
                relative_time(now, card.due)
            );
        }
        None => {
            println!("No cards match.");
        }
    }
    Ok(())
}

pub fn grade(conn: &mut Connection, args: GradeArgs) -> Result<()> {
    let id = ids::resolve(conn, &args.id)?;
    let row = load_card_state(conn, id)?;
    let rating = match args.rating {
        RatingArg::Again => Rating::Again,
        RatingArg::Hard => Rating::Hard,
        RatingArg::Good => Rating::Good,
        RatingArg::Easy => Rating::Easy,
    };
    let now = Utc::now();
    let outcome = scheduler::apply(&row, rating, now)?;

    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE cards
            SET stability = ?1,
                difficulty_f = ?2,
                due = ?3,
                last_review = ?4,
                reps = ?5,
                lapses = ?6,
                state = ?7
          WHERE id = ?8",
        params![
            outcome.stability,
            outcome.difficulty_f,
            outcome.due,
            now,
            outcome.reps,
            outcome.lapses,
            outcome.state.as_str(),
            id,
        ],
    )?;
    tx.execute(
        "INSERT INTO reviews (card_id, rating, reviewed_at) VALUES (?1, ?2, ?3)",
        params![id, rating.as_str(), now],
    )?;
    tx.commit()?;

    let due_local = outcome.due.with_timezone(&Local).date_naive();
    println!(
        "Next review: {due_local} ({} day{})",
        outcome.interval_days,
        if outcome.interval_days == 1 { "" } else { "s" }
    );
    Ok(())
}

pub fn delete(conn: &Connection, args: DeleteArgs) -> Result<()> {
    let id = ids::resolve(conn, &args.id)?;
    let title: String = conn.query_row("SELECT title FROM cards WHERE id = ?1", [id], |r| r.get(0))?;
    conn.execute("DELETE FROM cards WHERE id = ?1", [id])?;
    println!("Deleted card [{id}] \"{title}\".");
    Ok(())
}

pub fn list(conn: &Connection, args: ListArgs) -> Result<()> {
    let now = Utc::now();
    let (mut sql, mut params_v) = base_select();
    sql.push_str(" WHERE 1=1");
    if args.due {
        sql.push_str(" AND due <= ?");
        params_v.push(Box::new(now));
    }
    apply_filters(&mut sql, &mut params_v, &args.tags, args.difficulty);
    sql.push_str(" ORDER BY due ASC");

    let cards = query_many(conn, &sql, &params_v)?;
    if cards.is_empty() {
        println!("No cards.");
        return Ok(());
    }

    let max_title = cards.iter().map(|c| c.title.chars().count()).max().unwrap_or(0).max(5);
    println!(
        "{:>4}  {:<width$}  {:<6}  {}",
        "id",
        "title",
        "diff",
        "due",
        width = max_title
    );
    for c in &cards {
        let diff = c.difficulty.as_deref().unwrap_or("-");
        let due = relative_time(now, c.due);
        let tags = card_tags(conn, c.id)?;
        let tag_str = if tags.is_empty() { String::new() } else { format!("  [{}]", tags.join(", ")) };
        println!(
            "{:>4}  {:<width$}  {:<6}  {due}{tag_str}",
            c.id,
            c.title,
            diff,
            width = max_title
        );
    }
    Ok(())
}

// ----- helpers --------------------------------------------------------------

struct CardRow {
    id: i64,
    title: String,
    url: String,
    difficulty: Option<String>,
    due: DateTime<Utc>,
    state: String,
}

struct ComputedNow {
    stability: f32,
    difficulty_f: f32,
    due: DateTime<Utc>,
    last_review: Option<DateTime<Utc>>,
    state: CardState,
    reps: i64,
    lapses: i64,
}

fn base_select() -> (String, Vec<Box<dyn rusqlite::ToSql>>) {
    (
        "SELECT id, title, url, difficulty, due, state FROM cards".to_string(),
        Vec::new(),
    )
}

fn apply_filters(
    sql: &mut String,
    params_v: &mut Vec<Box<dyn rusqlite::ToSql>>,
    tags: &[String],
    difficulty: Option<Difficulty>,
) {
    for tag in normalize_tags(tags) {
        sql.push_str(
            " AND EXISTS (SELECT 1 FROM card_tags ct JOIN tags t ON ct.tag_id = t.id \
             WHERE ct.card_id = cards.id AND t.name = ?)",
        );
        params_v.push(Box::new(tag));
    }
    if let Some(d) = difficulty {
        sql.push_str(" AND difficulty = ?");
        params_v.push(Box::new(d.as_str().to_string()));
    }
}

fn query_one(
    conn: &Connection,
    sql: &str,
    params_v: &[Box<dyn rusqlite::ToSql>],
) -> Result<Option<CardRow>> {
    let mut stmt = conn.prepare(sql)?;
    let refs: Vec<&dyn rusqlite::ToSql> = params_v.iter().map(|b| b.as_ref()).collect();
    let row = stmt
        .query_row(params_from_iter(refs), row_to_card)
        .optional()?;
    Ok(row)
}

fn query_many(
    conn: &Connection,
    sql: &str,
    params_v: &[Box<dyn rusqlite::ToSql>],
) -> Result<Vec<CardRow>> {
    let mut stmt = conn.prepare(sql)?;
    let refs: Vec<&dyn rusqlite::ToSql> = params_v.iter().map(|b| b.as_ref()).collect();
    let rows = stmt
        .query_map(params_from_iter(refs), row_to_card)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

fn row_to_card(row: &rusqlite::Row) -> rusqlite::Result<CardRow> {
    Ok(CardRow {
        id: row.get(0)?,
        title: row.get(1)?,
        url: row.get(2)?,
        difficulty: row.get(3)?,
        due: row.get(4)?,
        state: row.get(5)?,
    })
}

fn print_next(card: &CardRow, now: DateTime<Utc>, conn: &Connection) -> Result<()> {
    let diff = card.difficulty.as_deref().unwrap_or("-");
    let due_str = relative_time(now, card.due);
    let _ = &card.state;
    println!("[{}] {}  ({})  {}", card.id, card.title, diff, due_str);
    println!("{}", card.url);
    let tags = card_tags(conn, card.id)?;
    if !tags.is_empty() {
        println!("tags: {}", tags.join(", "));
    }
    Ok(())
}

fn relative_time(now: DateTime<Utc>, then: DateTime<Utc>) -> String {
    let now_local = now.with_timezone(&Local).date_naive();
    let then_local = then.with_timezone(&Local).date_naive();
    let days = (then_local - now_local).num_days();
    if days > 0 {
        format!("due in {days} day{}", if days == 1 { "" } else { "s" })
    } else if days < 0 {
        let ago = -days;
        format!("due {ago} day{} ago", if ago == 1 { "" } else { "s" })
    } else if then <= now {
        "due now".to_string()
    } else {
        "due today".to_string()
    }
}

fn card_tags(conn: &Connection, card_id: i64) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT t.name FROM tags t JOIN card_tags ct ON ct.tag_id = t.id \
         WHERE ct.card_id = ?1 ORDER BY t.name",
    )?;
    let tags = stmt
        .query_map([card_id], |r| r.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(tags)
}

fn find_by_url(conn: &Connection, url: &str) -> Result<Option<(i64, String)>> {
    Ok(conn
        .query_row(
            "SELECT id, title FROM cards WHERE url = ?1",
            [url],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?)
}

fn attach_tag(conn: &Connection, card_id: i64, tag: &str) -> Result<()> {
    conn.execute("INSERT OR IGNORE INTO tags (name) VALUES (?1)", [tag])?;
    let tag_id: i64 = conn.query_row("SELECT id FROM tags WHERE name = ?1", [tag], |r| r.get(0))?;
    conn.execute(
        "INSERT OR IGNORE INTO card_tags (card_id, tag_id) VALUES (?1, ?2)",
        params![card_id, tag_id],
    )?;
    Ok(())
}

fn normalize_tags(tags: &[String]) -> Vec<String> {
    let mut out: Vec<String> = tags
        .iter()
        .map(|t| t.trim().to_ascii_lowercase())
        .filter(|t| !t.is_empty())
        .collect();
    out.sort();
    out.dedup();
    out
}

fn local_date_to_utc(date: NaiveDate) -> Result<DateTime<Utc>> {
    let local_dt = date.and_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap());
    let local = Local
        .from_local_datetime(&local_dt)
        .single()
        .ok_or_else(|| anyhow!("ambiguous local date {date}"))?;
    Ok(local.with_timezone(&Utc))
}

fn load_card_state(conn: &Connection, id: i64) -> Result<CurrentState> {
    conn.query_row(
        "SELECT stability, difficulty_f, last_review, reps, lapses, state FROM cards WHERE id = ?1",
        [id],
        |r| {
            Ok(CurrentState {
                stability: r.get(0)?,
                difficulty_f: r.get(1)?,
                last_review: r.get(2)?,
                reps: r.get(3)?,
                lapses: r.get(4)?,
                state: CardState::parse(&r.get::<_, String>(5)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(5, rusqlite::types::Type::Text, e.into()))?,
            })
        },
    )
    .context("card not found")
}
