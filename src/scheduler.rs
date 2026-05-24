use anyhow::{Result, anyhow};
use chrono::{DateTime, Duration, Utc};
use fsrs::{DEFAULT_PARAMETERS, FSRS, MemoryState};

const DESIRED_RETENTION: f32 = 0.9;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Rating {
    Again,
    Hard,
    Good,
    Easy,
}

impl Rating {
    pub fn as_str(self) -> &'static str {
        match self {
            Rating::Again => "again",
            Rating::Hard => "hard",
            Rating::Good => "good",
            Rating::Easy => "easy",
        }
    }

}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CardState {
    New,
    Learning,
    Review,
    Relearning,
}

impl CardState {
    pub fn as_str(self) -> &'static str {
        match self {
            CardState::New => "new",
            CardState::Learning => "learning",
            CardState::Review => "review",
            CardState::Relearning => "relearning",
        }
    }

    pub fn parse(s: &str) -> Result<Self> {
        match s {
            "new" => Ok(CardState::New),
            "learning" => Ok(CardState::Learning),
            "review" => Ok(CardState::Review),
            "relearning" => Ok(CardState::Relearning),
            other => Err(anyhow!("unknown card state '{other}'")),
        }
    }
}

/// FSRS-relevant fields of a card, before applying a rating.
pub struct CurrentState {
    pub stability: f32,
    pub difficulty_f: f32,
    pub last_review: Option<DateTime<Utc>>,
    pub reps: i64,
    pub lapses: i64,
    pub state: CardState,
}

/// FSRS-relevant fields after applying a rating.
pub struct Outcome {
    pub stability: f32,
    pub difficulty_f: f32,
    pub due: DateTime<Utc>,
    pub state: CardState,
    pub reps: i64,
    pub lapses: i64,
    pub interval_days: i64,
}

/// Default state for a brand-new card that has never been reviewed.
pub fn new_card(now: DateTime<Utc>) -> CurrentState {
    let _ = now;
    CurrentState {
        stability: 0.0,
        difficulty_f: 0.0,
        last_review: None,
        reps: 0,
        lapses: 0,
        state: CardState::New,
    }
}

/// Apply a rating to a card's current state at `now`, returning the new state.
pub fn apply(prev: &CurrentState, rating: Rating, now: DateTime<Utc>) -> Result<Outcome> {
    let fsrs = FSRS::new(Some(&DEFAULT_PARAMETERS))?;
    let memory_state = if matches!(prev.state, CardState::New) {
        None
    } else {
        Some(MemoryState {
            stability: prev.stability,
            difficulty: prev.difficulty_f,
        })
    };
    let days_elapsed = match prev.last_review {
        Some(t) => (now - t).num_days().max(0) as u32,
        None => 0,
    };
    let next = fsrs.next_states(memory_state, DESIRED_RETENTION, days_elapsed)?;
    let item_state = match rating {
        Rating::Again => next.again,
        Rating::Hard => next.hard,
        Rating::Good => next.good,
        Rating::Easy => next.easy,
    };

    let new_state = match rating {
        Rating::Again => CardState::Relearning,
        _ => CardState::Review,
    };
    let lapses = prev.lapses + if matches!(rating, Rating::Again) { 1 } else { 0 };
    let interval_days = item_state.interval.max(1.0).round() as i64;
    let due = now + Duration::days(interval_days);

    Ok(Outcome {
        stability: item_state.memory.stability,
        difficulty_f: item_state.memory.difficulty,
        due,
        state: new_state,
        reps: prev.reps + 1,
        lapses,
        interval_days,
    })
}
