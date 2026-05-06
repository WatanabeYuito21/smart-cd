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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Entry;
    use chrono::Utc;

    fn entry(path: &str) -> Entry {
        Entry { path: path.to_string(), visit_count: 1, last_visited: Utc::now() }
    }

    #[test]
    fn subsequence_contiguous_match() {
        assert!(is_subsequence("usr", "/usr/local"));
    }

    #[test]
    fn subsequence_non_contiguous_match() {
        // is_subsequence は事前にローワーケース化済みの文字列を受け取る
        assert!(is_subsequence("doc", "/home/user/documents"));
    }

    #[test]
    fn subsequence_no_match() {
        assert!(!is_subsequence("xyz", "/home/user/Documents"));
    }

    #[test]
    fn subsequence_empty_needle() {
        assert!(is_subsequence("", "/any/path"));
    }

    #[test]
    fn filter_empty_keywords_returns_all() {
        let e1 = entry("/foo");
        let e2 = entry("/bar");
        let entries = vec![&e1, &e2];
        assert_eq!(filter(&entries, &[]).len(), 2);
    }

    #[test]
    fn filter_case_insensitive() {
        let e = entry("/home/user/Documents");
        let entries = vec![&e];
        assert_eq!(filter(&entries, &["DOC"]).len(), 1);
    }

    #[test]
    fn filter_and_logic_all_must_match() {
        let e1 = entry("/home/user/projects/rust");
        let e2 = entry("/home/user/documents");
        let entries = vec![&e1, &e2];
        let result = filter(&entries, &["proj", "rust"]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, "/home/user/projects/rust");
    }

    #[test]
    fn filter_no_match_returns_empty() {
        let e = entry("/home/user");
        let entries = vec![&e];
        assert!(filter(&entries, &["zzz"]).is_empty());
    }

    #[test]
    fn filter_preserves_order() {
        let e1 = entry("/alpha/doc");
        let e2 = entry("/beta/doc");
        let entries = vec![&e1, &e2];
        let result = filter(&entries, &["doc"]);
        assert_eq!(result[0].path, "/alpha/doc");
        assert_eq!(result[1].path, "/beta/doc");
    }
}
