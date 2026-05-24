use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "lcsrs", version, about = "Spaced-repetition scheduler for LeetCode-style problems")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new problem to the database.
    Add(AddArgs),
    /// Print the next card to review.
    Next(FilterArgs),
    /// Record a review for a card.
    Grade(GradeArgs),
    /// Delete a card.
    Delete(DeleteArgs),
    /// List cards.
    List(ListArgs),
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Card title.
    pub title: String,
    /// Problem URL.
    pub url: String,
    /// LeetCode difficulty.
    #[arg(long, value_enum)]
    pub difficulty: Option<Difficulty>,
    /// Tag (repeatable).
    #[arg(long = "tag", value_name = "TAG")]
    pub tags: Vec<String>,
    /// Backfill: register as if successfully reviewed on this date (YYYY-MM-DD).
    #[arg(long, value_parser = parse_date)]
    pub solved: Option<NaiveDate>,
}

#[derive(Args, Debug)]
pub struct FilterArgs {
    /// Restrict to cards with this tag (repeatable, AND semantics).
    #[arg(long = "tag", value_name = "TAG")]
    pub tags: Vec<String>,
    /// Restrict to this difficulty.
    #[arg(long, value_enum)]
    pub difficulty: Option<Difficulty>,
}

#[derive(Args, Debug)]
pub struct GradeArgs {
    /// Card id (or unambiguous prefix).
    pub id: String,
    /// Rating.
    #[arg(value_enum)]
    pub rating: RatingArg,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    pub id: String,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Only cards due now or earlier.
    #[arg(long)]
    pub due: bool,
    #[arg(long = "tag", value_name = "TAG")]
    pub tags: Vec<String>,
    #[arg(long, value_enum)]
    pub difficulty: Option<Difficulty>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn as_str(self) -> &'static str {
        match self {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum RatingArg {
    Again,
    Hard,
    Good,
    Easy,
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| format!("invalid date '{s}': {e}"))
}
