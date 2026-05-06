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
