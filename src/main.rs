mod core;
mod filters;
mod models;
mod output;

use std::collections::HashSet;
use std::io::{self, Read};
use std::path::Path;
use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;

use crate::core::{CommentDetector, LanguageRegistry};
use crate::filters::{BddFilter, DirectiveFilter, ShebangFilter};
use crate::models::CommentInfo;
use crate::output::format_hook_message;

const EXIT_PASS: u8 = 0;
const EXIT_BLOCK: u8 = 2;

#[derive(Parser)]
#[command(name = "comment-checker")]
#[command(about = "Check for problematic comments in source code")]
#[command(
    long_about = "A hook for Claude Code that detects and warns about comments and docstrings in source code."
)]
struct Cli {
    #[arg(
        long,
        help = "Custom prompt to replace the default warning message. Use {{comments}} placeholder for detected comments XML."
    )]
    prompt: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Edit {
    old_string: Option<String>,
    new_string: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ToolInput {
    file_path: Option<String>,
    content: Option<String>,
    new_string: Option<String>,
    old_string: Option<String>,
    edits: Option<Vec<Edit>>,
}

#[derive(Debug, Deserialize)]
struct HookInput {
    tool_name: Option<String>,
    tool_input: ToolInput,
}

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(_) => {
            tracing::warn!("skipping: internal error");
            ExitCode::from(EXIT_PASS)
        }
    }
}

fn run() -> Result<u8> {
    let cli = Cli::parse();
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        tracing::warn!("skipping: failed to read stdin");
        return Ok(EXIT_PASS);
    }
    if input.is_empty() {
        tracing::warn!("skipping: no input provided");
        return Ok(EXIT_PASS);
    }
    let hook_input = match serde_json::from_str::<HookInput>(&input) {
        Ok(h) => h,
        Err(_) => {
            tracing::warn!("skipping: invalid input format");
            return Ok(EXIT_PASS);
        }
    };
    let file_path = match &hook_input.tool_input.file_path {
        Some(p) if !p.is_empty() => p.clone(),
        _ => {
            tracing::warn!("skipping: no file path provided");
            return Ok(EXIT_PASS);
        }
    };
    let ext = Path::new(&file_path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| {
            Path::new(&file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_lowercase()
        });
    let registry = LanguageRegistry::new();
    if !registry.is_supported(&ext) {
        tracing::warn!("skipping: non-code file");
        return Ok(EXIT_PASS);
    }
    let detector = CommentDetector::new();
    let tool_name = hook_input.tool_name.as_deref().unwrap_or("");
    let comments = match tool_name {
        "Edit" => {
            let new_string = match &hook_input.tool_input.new_string {
                Some(s) if !s.is_empty() => s,
                _ => {
                    tracing::warn!("skipping: no content to check");
                    return Ok(EXIT_PASS);
                }
            };
            let old_string = hook_input.tool_input.old_string.as_deref().unwrap_or("");
            detect_new_comments_for_edit(&detector, old_string, new_string, &file_path)
        }
        "MultiEdit" => {
            let edits = match &hook_input.tool_input.edits {
                Some(e) if !e.is_empty() => e,
                _ => {
                    tracing::warn!("skipping: no content to check");
                    return Ok(EXIT_PASS);
                }
            };
            let mut all_comments = Vec::new();
            for edit in edits {
                let new_string = match &edit.new_string {
                    Some(s) if !s.is_empty() => s,
                    _ => continue,
                };
                let old_string = edit.old_string.as_deref().unwrap_or("");
                let edit_comments =
                    detect_new_comments_for_edit(&detector, old_string, new_string, &file_path);
                all_comments.extend(edit_comments);
            }
            all_comments
        }
        _ => {
            let content = get_content_to_check(&hook_input);
            if content.is_empty() {
                tracing::warn!("Skipping: No content to check");
                return Ok(EXIT_PASS);
            }
            detector.detect(&content, &file_path, true)
        }
    };
    if comments.is_empty() {
        tracing::info!("success: no problematic comments/docstrings found");
        return Ok(EXIT_PASS);
    }
    let filtered = apply_filters(comments);
    if filtered.is_empty() {
        tracing::info!("success: no problematic comments/docstrings found");
        return Ok(EXIT_PASS);
    }
    let message = format_hook_message(&filtered, cli.prompt.as_deref());
    eprint!("{}", message);
    Ok(EXIT_BLOCK)
}

fn get_content_to_check(input: &HookInput) -> String {
    let tool_name = input.tool_name.as_deref().unwrap_or("");
    match tool_name {
        "Write" => input.tool_input.content.clone().unwrap_or_default(),
        "Edit" => input.tool_input.new_string.clone().unwrap_or_default(),
        "MultiEdit" => {
            let mut parts = Vec::new();
            if let Some(edits) = &input.tool_input.edits {
                for edit in edits {
                    if let Some(new_string) = &edit.new_string
                        && !new_string.is_empty()
                    {
                        parts.push(new_string.clone());
                    }
                }
            }
            parts.join("\n")
        }
        _ => {
            if let Some(content) = &input.tool_input.content
                && !content.is_empty()
            {
                return content.clone();
            }
            input.tool_input.new_string.clone().unwrap_or_default()
        }
    }
}

fn apply_filters(comments: Vec<CommentInfo>) -> Vec<CommentInfo> {
    let bdd_filter = BddFilter::new();
    let directive_filter = DirectiveFilter::new();
    let shebang_filter = ShebangFilter::new();
    comments
        .into_iter()
        .filter(|c| !bdd_filter.should_skip(c))
        .filter(|c| !directive_filter.should_skip(c))
        .filter(|c| !shebang_filter.should_skip(c))
        .collect()
}

fn build_comment_text_set(comments: &[CommentInfo]) -> HashSet<String> {
    comments.iter().map(|c| c.normalised_text()).collect()
}

fn filter_new_comments(
    old_comments: &[CommentInfo],
    new_comments: Vec<CommentInfo>,
) -> Vec<CommentInfo> {
    if old_comments.is_empty() {
        return new_comments;
    }
    let old_set = build_comment_text_set(old_comments);
    new_comments
        .into_iter()
        .filter(|c| !old_set.contains(&c.normalised_text()))
        .collect()
}

fn detect_new_comments_for_edit(
    detector: &CommentDetector,
    old_string: &str,
    new_string: &str,
    file_path: &str,
) -> Vec<CommentInfo> {
    let old_comments = detector.detect(old_string, file_path, true);
    let new_comments = detector.detect(new_string, file_path, true);
    filter_new_comments(&old_comments, new_comments)
}
