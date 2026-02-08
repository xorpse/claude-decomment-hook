use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::models::CommentInfo;

static BDD_KEYWORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert("given");
    s.insert("when");
    s.insert("then");
    s.insert("arrange");
    s.insert("act");
    s.insert("assert");
    s.insert("when & then");
    s.insert("when&then");
    s
});

pub struct BddFilter;

impl BddFilter {
    pub fn new() -> Self {
        Self
    }

    pub fn should_skip(&self, comment: &CommentInfo) -> bool {
        let mut normalised = comment.text().trim().to_lowercase();
        for prefix in &["#", "//", "--"] {
            if let Some(rest) = normalised.strip_prefix(prefix) {
                normalised = rest.trim().to_owned();
                break;
            }
        }
        BDD_KEYWORDS.contains(normalised.as_str())
    }
}

impl Default for BddFilter {
    fn default() -> Self {
        Self::new()
    }
}
