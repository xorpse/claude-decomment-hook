use std::collections::HashMap;

use once_cell::sync::Lazy;

pub static QUERY_TEMPLATES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("python", "(comment) @comment");
    m.insert("javascript", "(comment) @comment");
    m.insert("typescript", "(comment) @comment");
    m.insert("tsx", "(comment) @comment");
    m.insert("golang", "(comment) @comment");
    m.insert("rust", "(line_comment) @comment\n(block_comment) @comment");
    m.insert("swift", "(comment) @comment");
    m.insert("java", "(line_comment) @comment\n(block_comment) @comment");
    m.insert("elixir", "(comment) @comment");
    m.insert("c", "(comment) @comment");
    m.insert("cpp", "(comment) @comment");
    m.insert("csharp", "(comment) @comment");
    m.insert("ruby", "(comment) @comment");
    m.insert("php", "(comment) @comment");
    m.insert("bash", "(comment) @comment");
    m.insert("lua", "(comment) @comment");
    m.insert("ocaml", "(comment) @comment");
    m.insert("html", "(comment) @comment");
    m.insert("css", "(comment) @comment");
    m.insert("yaml", "(comment) @comment");
    m
});

pub static DOCSTRING_QUERIES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "python",
        r#"
        (module . (expression_statement (string) @docstring))
        (class_definition body: (block . (expression_statement (string) @docstring)))
        (function_definition body: (block . (expression_statement (string) @docstring)))
    "#,
    );
    m.insert(
        "javascript",
        r#"
        (comment) @jsdoc
        (#match? @jsdoc "^/\\*\\*")
    "#,
    );
    m.insert(
        "typescript",
        r#"
        (comment) @jsdoc
        (#match? @jsdoc "^/\\*\\*")
    "#,
    );
    m.insert(
        "tsx",
        r#"
        (comment) @jsdoc
        (#match? @jsdoc "^/\\*\\*")
    "#,
    );
    m.insert(
        "java",
        r#"
        (comment) @javadoc
        (#match? @javadoc "^/\\*\\*")
    "#,
    );
    m
});
