use crate::models::CommentInfo;

pub struct ShebangFilter;

impl ShebangFilter {
    pub fn new() -> Self {
        Self
    }

    pub fn should_skip(&self, comment: &CommentInfo) -> bool {
        comment.text().trim().starts_with("#!")
    }
}

impl Default for ShebangFilter {
    fn default() -> Self {
        Self::new()
    }
}
