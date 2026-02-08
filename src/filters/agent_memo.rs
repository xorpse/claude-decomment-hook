use once_cell::sync::Lazy;
use regex::Regex;

use crate::models::CommentInfo;

static AGENT_MEMO_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)^[\s#/*-]*changed?\s+(from|to)\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*modified?\s+(from|to)?\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*updated?\s+(from|to)?\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*refactor(ed|ing)?\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*moved?\s+(from|to)\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*renamed?\s+(from|to)?\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*replaced?\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*removed?\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*deleted?\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*added?\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*implemented?\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*this\s+(implements?|adds?|removes?|changes?|fixes?)\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*here\s+we\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*now\s+(we|this|it)\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*previously\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*before\s+this\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*after\s+this\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*was\s+changed\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*implementation\s+(of|note)\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*note:\s*\w").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*[a-z]+\s*->\s*[a-z]+").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*converted?\s+(from|to)\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*migrated?\s+(from|to)?\b").unwrap(),
        Regex::new(r"(?i)^[\s#/*-]*switched?\s+(from|to)\b").unwrap(),
        Regex::new(r"(?i)여기(서|에서)\s*").unwrap(),
        Regex::new(r"(?i)(으로|로)\s*(바뀜|변경|변환)").unwrap(),
        Regex::new(r"(?i)구현(임|함|했|된|됨)").unwrap(),
        Regex::new(r"(?i)추가(함|했|된|됨)").unwrap(),
        Regex::new(r"(?i)삭제(함|했|된|됨)").unwrap(),
        Regex::new(r"(?i)수정(함|했|된|됨)").unwrap(),
        Regex::new(r"(?i)변경(함|했|된|됨)").unwrap(),
        Regex::new(r"(?i)리팩(터|토)링").unwrap(),
        Regex::new(r"(?i)이전(에는|엔)").unwrap(),
        Regex::new(r"(?i)기존(에는|엔|의)").unwrap(),
        Regex::new(r"(?i)에서\s+\S+\s*(으로|로)\b").unwrap(),
    ]
});

pub struct AgentMemoFilter;

impl AgentMemoFilter {
    pub fn new() -> Self {
        Self
    }

    pub fn is_agent_memo(&self, comment: &CommentInfo) -> bool {
        let mut text = comment.text().trim().to_owned();
        for prefix in &["#", "//", "/*", "--", "*"] {
            if let Some(rest) = text.strip_prefix(prefix) {
                text = rest.trim().to_owned();
            }
        }
        for pattern in AGENT_MEMO_PATTERNS.iter() {
            if pattern.is_match(&text) {
                return true;
            }
        }
        false
    }
}

impl Default for AgentMemoFilter {
    fn default() -> Self {
        Self::new()
    }
}
