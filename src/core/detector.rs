use std::path::Path;

use regex::Regex;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::core::language_registry::{LanguageRegistry, get_language};
use crate::core::query_templates::{DOCSTRING_QUERIES, QUERY_TEMPLATES};
use crate::models::{CommentInfo, CommentType};

fn get_docstring_pattern(lang_name: &str) -> Option<&'static str> {
    match lang_name {
        "javascript" | "typescript" | "tsx" | "java" => Some(r"^/\*\*"),
        _ => None,
    }
}

pub struct CommentDetector {
    registry: LanguageRegistry,
}

impl CommentDetector {
    pub fn new() -> Self {
        Self {
            registry: LanguageRegistry::new(),
        }
    }

    pub fn detect(
        &self,
        content: &str,
        file_path: &str,
        include_docstrings: bool,
    ) -> Vec<CommentInfo> {
        let ext = Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_else(|| {
                Path::new(file_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default()
            })
            .to_lowercase();
        let lang_name = match self.registry.get_language_name(&ext) {
            Some(name) => name,
            None => return Vec::new(),
        };
        let lang = match get_language(lang_name) {
            Some(l) => l,
            None => return Vec::new(),
        };
        let mut parser = Parser::new();
        if parser.set_language(&lang).is_err() {
            return Vec::new();
        }
        let tree = match parser.parse(content, None) {
            Some(t) => t,
            None => return Vec::new(),
        };
        let query_pattern = QUERY_TEMPLATES
            .get(lang_name)
            .copied()
            .unwrap_or("(comment) @comment");
        let query = match Query::new(&lang, query_pattern) {
            Ok(q) => q,
            Err(_) => return Vec::new(),
        };
        let mut cursor = QueryCursor::new();
        let source_bytes = content.as_bytes();
        let mut comments = Vec::new();
        let docstring_pattern = get_docstring_pattern(lang_name);
        let docstring_regex = docstring_pattern.and_then(|p| Regex::new(p).ok());
        let mut matches = cursor.matches(&query, tree.root_node(), source_bytes);
        while let Some(m) = matches.next() {
            for capture in m.captures {
                let node = capture.node;
                let text = node.utf8_text(source_bytes).unwrap_or_default();
                if include_docstrings
                    && let Some(ref re) = docstring_regex
                    && re.is_match(text)
                {
                    continue;
                }
                let text = text.to_owned();
                let line_number = node.start_position().row + 1;
                let comment_type = self.determine_comment_type(&text, node.kind());
                let is_docstring = comment_type == CommentType::Docstring;
                if is_docstring && !include_docstrings {
                    continue;
                }
                comments.push(CommentInfo::new(
                    text,
                    line_number,
                    file_path.to_owned(),
                    comment_type,
                    is_docstring,
                ));
            }
        }
        if include_docstrings {
            let docstrings = self.detect_docstrings(source_bytes, file_path, &lang, lang_name);
            comments.extend(docstrings);
        }
        comments
    }

    fn detect_docstrings(
        &self,
        source_bytes: &[u8],
        file_path: &str,
        lang: &Language,
        lang_name: &str,
    ) -> Vec<CommentInfo> {
        let doc_query = match DOCSTRING_QUERIES.get(lang_name) {
            Some(q) => *q,
            None => return Vec::new(),
        };
        let mut parser = Parser::new();
        if parser.set_language(lang).is_err() {
            return Vec::new();
        }
        let tree = match parser.parse(source_bytes, None) {
            Some(t) => t,
            None => return Vec::new(),
        };
        let query = match Query::new(lang, doc_query) {
            Ok(q) => q,
            Err(_) => return Vec::new(),
        };
        let pattern = get_docstring_pattern(lang_name);
        let regex = pattern.and_then(|p| Regex::new(p).ok());
        let mut cursor = QueryCursor::new();
        let mut docstrings = Vec::new();
        let mut matches = cursor.matches(&query, tree.root_node(), source_bytes);
        while let Some(m) = matches.next() {
            for capture in m.captures {
                let node = capture.node;
                let text = node.utf8_text(source_bytes).unwrap_or_default();
                if let Some(ref re) = regex
                    && !re.is_match(text)
                {
                    continue;
                }
                let line_number = node.start_position().row + 1;
                docstrings.push(CommentInfo::new(
                    text.to_owned(),
                    line_number,
                    file_path.to_owned(),
                    CommentType::Docstring,
                    true,
                ));
            }
        }
        docstrings
    }

    fn determine_comment_type(&self, text: &str, node_type: &str) -> CommentType {
        let stripped = text.trim();
        if node_type == "line_comment" {
            return CommentType::Line;
        }
        if node_type == "block_comment" {
            return CommentType::Block;
        }
        if stripped.starts_with("\"\"\"") || stripped.starts_with("'''") {
            return CommentType::Docstring;
        }
        if stripped.starts_with("//") || stripped.starts_with('#') {
            return CommentType::Line;
        }
        if stripped.starts_with("/*") || stripped.starts_with("<!--") || stripped.starts_with("--")
        {
            return CommentType::Block;
        }
        CommentType::Line
    }
}

impl Default for CommentDetector {
    fn default() -> Self {
        Self::new()
    }
}
