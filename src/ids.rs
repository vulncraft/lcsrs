use anyhow::{Result, anyhow, bail};
use rusqlite::Connection;

/// Resolve a user-supplied id token (a full id or a prefix) to a unique card id.
///
/// Exact matches always win. If the token is not an exact id but is a unique
/// prefix of exactly one id, that id is returned. Ambiguous prefixes error.
pub fn resolve(conn: &Connection, token: &str) -> Result<i64> {
    if token.is_empty() {
        bail!("empty card id");
    }
    if !token.chars().all(|c| c.is_ascii_digit()) {
        bail!("card id must be numeric: {token}");
    }

    if let Some(id) = exact_match(conn, token)? {
        return Ok(id);
    }

    let mut stmt = conn.prepare("SELECT id FROM cards WHERE CAST(id AS TEXT) LIKE ?1 LIMIT 2")?;
    let pattern = format!("{token}%");
    let ids: Vec<i64> = stmt
        .query_map([&pattern], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    match ids.len() {
        0 => Err(anyhow!("no card matches id '{token}'")),
        1 => Ok(ids[0]),
        _ => Err(anyhow!(
            "id '{token}' is ambiguous (matches multiple cards); use the full id"
        )),
    }
}

fn exact_match(conn: &Connection, token: &str) -> Result<Option<i64>> {
    let id: i64 = match token.parse() {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };
    let found: Option<i64> = conn
        .query_row("SELECT id FROM cards WHERE id = ?1", [id], |r| r.get(0))
        .ok();
    Ok(found)
}
