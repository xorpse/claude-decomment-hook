use once_cell::sync::Lazy;

use crate::models::CommentInfo;

static TYPE_CHECKER_PREFIXES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "type:",
        "noqa",
        "pyright:",
        "ruff:",
        "mypy:",
        "pylint:",
        "flake8:",
        "pyre:",
        "pytype:",
        "eslint-disable",
        "eslint-ignore",
        "prettier-ignore",
        "ts-ignore",
        "ts-expect-error",
        "clippy:",
        "allow",
        "deny",
        "warn",
        "forbid",
    ]
});

pub struct DirectiveFilter;

impl DirectiveFilter {
    pub fn new() -> Self {
        Self
    }

    pub fn should_skip(&self, comment: &CommentInfo) -> bool {
        let mut normalised = comment.text().trim().to_lowercase();
        for prefix in &["#", "//", "/*", "--"] {
            if let Some(rest) = normalised.strip_prefix(prefix) {
                normalised = rest.trim().to_owned();
                break;
            }
        }
        if let Some(rest) = normalised.strip_prefix('@') {
            normalised = rest.trim().to_owned();
        }
        for directive in TYPE_CHECKER_PREFIXES.iter() {
            if normalised.starts_with(directive) {
                return true;
            }
        }
        false
    }
}

impl Default for DirectiveFilter {
    fn default() -> Self {
        Self::new()
    }
}
