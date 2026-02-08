use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentType {
    Line,
    Block,
    Docstring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentInfo {
    text: String,
    line_number: usize,
    file_path: String,
    comment_type: CommentType,
    is_docstring: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
}

impl CommentInfo {
    pub fn new(
        text: String,
        line_number: usize,
        file_path: String,
        comment_type: CommentType,
        is_docstring: bool,
    ) -> Self {
        Self {
            text,
            line_number,
            file_path,
            comment_type,
            is_docstring,
            metadata: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn line_number(&self) -> usize {
        self.line_number
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn normalised_text(&self) -> String {
        self.text.trim().to_lowercase()
    }
}
