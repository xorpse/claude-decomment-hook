use std::collections::HashMap;

use crate::filters::AgentMemoFilter;
use crate::models::CommentInfo;
use crate::output::xml_builder::build_comments_xml;

pub fn format_hook_message(comments: &[CommentInfo], custom_prompt: Option<&str>) -> String {
    if comments.is_empty() {
        return String::new();
    }
    let mut by_file = HashMap::<&str, Vec<&CommentInfo>>::new();
    let mut file_order = Vec::<&str>::new();
    for comment in comments {
        let path = comment.file_path();
        if !by_file.contains_key(path) {
            file_order.push(path);
        }
        by_file.entry(path).or_default().push(comment);
    }
    let mut comments_xml = String::new();
    for file_path in &file_order {
        if let Some(file_comments) = by_file.get(file_path) {
            let owned_comments = file_comments
                .iter()
                .map(|c| (*c).clone())
                .collect::<Vec<CommentInfo>>();
            comments_xml.push_str(&build_comments_xml(&owned_comments, file_path));
            comments_xml.push('\n');
        }
    }
    if let Some(prompt) = custom_prompt
        && !prompt.is_empty()
    {
        return prompt.replace("{{comments}}", &comments_xml);
    }
    let agent_memo_filter = AgentMemoFilter::new();
    let agent_memo_comments = comments
        .iter()
        .filter(|c| agent_memo_filter.is_agent_memo(c))
        .collect::<Vec<&CommentInfo>>();
    let has_agent_memo = !agent_memo_comments.is_empty();
    let mut sb = String::new();
    if has_agent_memo {
        sb.push_str("AGENT MEMO COMMENT DETECTED - CODE SMELL ALERT\n\n");
    } else {
        sb.push_str("COMMENT/DOCSTRING DETECTED - IMMEDIATE ACTION REQUIRED\n\n");
    }
    if has_agent_memo {
        sb.push_str("AGENT MEMO COMMENTS DETECTED - THIS IS A CODE SMELL\n\n");
        sb.push_str("You left \"memo-style\" comments that describe WHAT you changed or HOW you implemented something.\n");
        sb.push_str(
            "These are typically signs of an AI agent leaving notes for itself or the user.\n\n",
        );
        sb.push_str("Examples of agent memo patterns detected:\n");
        sb.push_str("  - \"Changed from X to Y\", \"Modified to...\", \"Updated from...\"\n");
        sb.push_str("  - \"Added new...\", \"Removed...\", \"Refactored...\"\n");
        sb.push_str("  - \"This implements...\", \"Here we...\", \"Now this...\"\n");
        sb.push_str("  - \"Note:\", \"Implementation of...\"\n");
        sb.push_str("WHY THIS IS BAD:\n");
        sb.push_str("  1. Code should be self-documenting - if you need to explain what changed, the code isn't clear enough\n");
        sb.push_str("  2. These comments become outdated instantly and mislead future readers\n");
        sb.push_str(
            "  3. Git history already tracks what changed - comments duplicate this poorly\n",
        );
        sb.push_str("  4. It's a sign of rushing without proper refactoring\n\n");
        sb.push_str("ACTION REQUIRED:\n");
        sb.push_str("  -> REMOVE these memo comments immediately\n");
        sb.push_str("  -> If the code needs explanation, refactor it to be clearer instead\n");
        sb.push_str("  -> Use meaningful variable/function names that explain intent\n");
        sb.push_str("  -> Let git commit messages document the \"what\" and \"why\"\n\n");
        sb.push_str("Detected agent memo comments:\n");
        for memo in &agent_memo_comments {
            sb.push_str(&format!(
                "  - Line {}: {}\n",
                memo.line_number(),
                memo.text().trim()
            ));
        }
        sb.push_str("\n---\n\n");
    }
    sb.push_str("Your recent changes contain comments or docstrings, which triggered this hook.\n");
    sb.push_str("You need to take immediate action. You must follow the conditions below.\n");
    sb.push_str(
        "(Listed in priority order - you must always act according to this priority order)\n\n",
    );
    sb.push_str("CRITICAL WARNING: This hook message MUST NEVER be ignored, even if you receive it multiple times.\n");
    sb.push_str("You MUST provide corresponding explanation or action for EACH occurrence of this message.\n");
    sb.push_str(
        "Ignoring this message or failing to respond appropriately is strictly prohibited.\n\n",
    );
    sb.push_str("PRIORITY-BASED ACTION GUIDELINES:\n\n");
    sb.push_str("1. This is a comment/docstring that already existed before\n");
    sb.push_str("\t-> Explain to the user that this is an existing comment/docstring and proceed (justify it)\n\n");
    sb.push_str("2. This is a newly written comment: but it's in given, when, then format\n");
    sb.push_str("\t-> Tell the user it's a BDD comment and proceed (justify it)\n");
    sb.push_str("\t-> Note: This applies to comments only, not docstrings\n\n");
    sb.push_str("\t-> Tell the user why this comment/docstring is absolutely necessary and proceed (justify it)\n");
    sb.push_str("\t-> Examples of necessary comments: complex algorithms, security-related, performance optimization, regex, mathematical formulas\n");
    sb.push_str("\t-> Examples of necessary docstrings: public API documentation, complex module/class interfaces\n");
    sb.push_str("\t-> IMPORTANT: Most docstrings are unnecessary if the code is self-explanatory. Only keep truly essential ones.\n\n");
    sb.push_str(
        "3. This is a newly written comment/docstring: but it's an unnecessary comment/docstring\n",
    );
    sb.push_str("\t-> Apologize to the user and remove the comment/docstring.\n");
    sb.push_str(
        "\t-> Make the code itself clearer so it can be understood without comments/docstrings.\n",
    );
    sb.push_str("\t-> For verbose docstrings: refactor code to be self-documenting instead of adding lengthy explanations.\n\n");
    sb.push_str("MANDATORY REQUIREMENT: You must acknowledge this hook message and take one of the above actions.\n");
    sb.push_str("Review in the above priority order and take the corresponding action EVERY TIME this appears.\n\n");
    sb.push_str("REMINDER: These rules apply to ALL your future code, not just this specific edit. Always be deliberate and cautious when writing comments - only add them when absolutely necessary.\n\n");
    sb.push_str("Detected comments/docstrings:\n");
    sb.push_str(&comments_xml);
    sb
}
