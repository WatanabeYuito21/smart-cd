use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub path: String,
    pub visit_count: u32,
    pub last_visited: DateTime<Utc>,
}

impl Entry {
    pub fn score(&self) -> f64 {
        let elapsed_hours =
            (Utc::now() - self.last_visited).num_seconds() as f64 / 3600.0;
        let time_decay = 1.0 / (1.0 + elapsed_hours * 0.01);
        self.visit_count as f64 * time_decay
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub version: u32,
    pub entries: Vec<Entry>,
}

impl Database {
    pub fn db_path() -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
            PathBuf::from(appdata).join("smart-cd").join("db.json")
        }
        #[cfg(not(target_os = "windows"))]
        {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("smart-cd")
                .join("db.json")
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::db_path();
        if !path.exists() {
            return Ok(Self {
                version: 1,
                entries: Vec::new(),
            });
        }
        let contents = std::fs::read_to_string(&path)?;
        let db = serde_json::from_str(&contents)?;
        Ok(db)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::db_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, contents)?;
        Ok(())
    }

    pub fn add(&mut self, path: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.path == path) {
            entry.visit_count += 1;
            entry.last_visited = Utc::now();
        } else {
            self.entries.push(Entry {
                path: path.to_string(),
                visit_count: 1,
                last_visited: Utc::now(),
            });
        }
    }

    pub fn remove(&mut self, path: &str) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.path != path);
        self.entries.len() < before
    }

    pub fn clean(&mut self) -> usize {
        let before = self.entries.len();
        self.entries.retain(|e| std::path::Path::new(&e.path).exists());
        before - self.entries.len()
    }

    pub fn sorted_entries(&self) -> Vec<&Entry> {
        let mut entries: Vec<&Entry> = self.entries.iter().collect();
        entries.sort_by(|a, b| b.score().partial_cmp(&a.score()).unwrap_or(std::cmp::Ordering::Equal));
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn entry(path: &str, visit_count: u32, hours_ago: i64) -> Entry {
        Entry {
            path: path.to_string(),
            visit_count,
            last_visited: Utc::now() - Duration::hours(hours_ago),
        }
    }

    fn empty_db() -> Database {
        Database { version: 1, entries: Vec::new() }
    }

    #[test]
    fn score_recent_beats_old() {
        let recent = entry("/recent", 1, 0);
        let old = entry("/old", 1, 1000);
        assert!(recent.score() > old.score());
    }

    #[test]
    fn score_high_count_beats_low() {
        let frequent = entry("/frequent", 10, 1);
        let rare = entry("/rare", 1, 1);
        assert!(frequent.score() > rare.score());
    }

    #[test]
    fn score_is_positive() {
        let e = entry("/some/path", 5, 100);
        assert!(e.score() > 0.0);
    }

    #[test]
    fn add_new_entry() {
        let mut db = empty_db();
        db.add("/foo");
        assert_eq!(db.entries.len(), 1);
        assert_eq!(db.entries[0].path, "/foo");
        assert_eq!(db.entries[0].visit_count, 1);
    }

    #[test]
    fn add_existing_increments_count() {
        let mut db = empty_db();
        db.add("/foo");
        db.add("/foo");
        assert_eq!(db.entries.len(), 1);
        assert_eq!(db.entries[0].visit_count, 2);
    }

    #[test]
    fn remove_existing_returns_true() {
        let mut db = empty_db();
        db.add("/foo");
        assert!(db.remove("/foo"));
        assert!(db.entries.is_empty());
    }

    #[test]
    fn remove_missing_returns_false() {
        let mut db = empty_db();
        assert!(!db.remove("/nonexistent"));
    }

    #[test]
    fn clean_removes_nonexistent_paths() {
        let mut db = empty_db();
        db.entries.push(entry("/nonexistent/ghost", 1, 0));
        db.entries.push(entry("/tmp", 1, 0));
        let removed = db.clean();
        assert_eq!(removed, 1);
        assert_eq!(db.entries.len(), 1);
        assert_eq!(db.entries[0].path, "/tmp");
    }

    #[test]
    fn clean_all_valid_removes_none() {
        let mut db = empty_db();
        db.entries.push(entry("/tmp", 1, 0));
        assert_eq!(db.clean(), 0);
        assert_eq!(db.entries.len(), 1);
    }

    #[test]
    fn sorted_entries_descending_by_score() {
        let mut db = empty_db();
        db.entries.push(entry("/old", 1, 500));
        db.entries.push(entry("/recent", 5, 0));
        let sorted = db.sorted_entries();
        assert_eq!(sorted[0].path, "/recent");
        assert_eq!(sorted[1].path, "/old");
    }
}
