use crate::db::Entry;

pub fn filter<'a>(entries: &[&'a Entry], keywords: &[&str]) -> Vec<&'a Entry> {
    if keywords.is_empty() {
        return entries.to_vec();
    }
    let lower_keywords: Vec<String> = keywords.iter().map(|k| k.to_lowercase()).collect();
    entries
        .iter()
        .filter(|e| {
            let lower_path = e.path.to_lowercase();
            lower_keywords.iter().all(|k| lower_path.contains(k.as_str()))
        })
        .copied()
        .collect()
}
