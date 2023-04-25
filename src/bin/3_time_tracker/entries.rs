use std::time::Duration;

use anyhow::Context;
use chrono::{DateTime, Utc};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TimeEntry {
    pub id: u32,
    pub start: DateTime<Utc>,
    pub length: Duration,
    pub description: String,
}

const ENTRY_PATH: &str = "/tmp/time_tracking.json";

pub fn load_entries() -> Result<Vec<TimeEntry>, anyhow::Error> {
    let file = std::fs::File::open(ENTRY_PATH)
        .with_context(|| format!("Failed to find path {ENTRY_PATH}"))?;
    let reader = std::io::BufReader::new(file);
    let entries = serde_json::from_reader(reader)
        .with_context(|| format!("Failed to load entries from {ENTRY_PATH}"))?;
    Ok(entries)
}

pub fn save_entries(entries: &[TimeEntry]) -> Result<(), anyhow::Error> {
    let file = std::fs::File::create(ENTRY_PATH)
        .with_context(|| format!("Failed to create file at {ENTRY_PATH}"))?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, entries)
        .with_context(|| format!("Failed to save entries to {ENTRY_PATH}"))?;
    Ok(())
}
