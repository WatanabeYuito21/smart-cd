use crate::db::Entry;

fn is_subsequence(needle: &str, haystack: &str) -> bool {
    let mut haystack_chars = haystack.chars();
    needle.chars().all(|nc| haystack_chars.any(|hc| hc == nc))
}

pub fn filter<'a>(entries: &[&'a Entry], keywords: &[&str]) -> Vec<&'a Entry> {
    if keywords.is_empty() {
        return entries.to_vec();
    }
    let lower_keywords: Vec<String> = keywords.iter().map(|k| k.to_lowercase()).collect();
    entries
        .iter()
        .filter(|e| {
            let lower_path = e.path.to_lowercase();
            lower_keywords.iter().all(|k| is_subsequence(k, &lower_path))
        })
        .copied()
        .collect()
}
