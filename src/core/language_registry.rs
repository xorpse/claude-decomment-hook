use std::collections::HashMap;

use once_cell::sync::Lazy;
use tree_sitter::Language;

pub static EXTENSION_TO_LANGUAGE: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("py", "python");
    m.insert("js", "javascript");
    m.insert("jsx", "javascript");
    m.insert("ts", "typescript");
    m.insert("tsx", "tsx");
    m.insert("go", "golang");
    m.insert("java", "java");
    m.insert("scala", "scala");
    m.insert("c", "c");
    m.insert("h", "c");
    m.insert("cpp", "cpp");
    m.insert("cc", "cpp");
    m.insert("cxx", "cpp");
    m.insert("hpp", "cpp");
    m.insert("rs", "rust");
    m.insert("rb", "ruby");
    m.insert("sh", "bash");
    m.insert("bash", "bash");
    m.insert("cs", "csharp");
    m.insert("swift", "swift");
    m.insert("ex", "elixir");
    m.insert("exs", "elixir");
    m.insert("lua", "lua");
    m.insert("php", "php");
    m.insert("ml", "ocaml");
    m.insert("mli", "ocaml");
    m.insert("html", "html");
    m.insert("htm", "html");
    m.insert("css", "css");
    m.insert("yaml", "yaml");
    m.insert("yml", "yaml");
    m
});

pub struct LanguageRegistry;

impl LanguageRegistry {
    pub fn new() -> Self {
        Self
    }

    pub fn get_language_name(&self, extension: &str) -> Option<&'static str> {
        let ext = extension.trim_start_matches('.').to_lowercase();
        EXTENSION_TO_LANGUAGE.get(ext.as_str()).copied()
    }

    pub fn is_supported(&self, extension: &str) -> bool {
        self.get_language_name(extension).is_some()
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_language(name: &str) -> Option<Language> {
    match name {
        "python" => Some(tree_sitter_python::LANGUAGE.into()),
        "javascript" => Some(tree_sitter_javascript::LANGUAGE.into()),
        "typescript" => Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
        "tsx" => Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
        "golang" => Some(tree_sitter_go::LANGUAGE.into()),
        "java" => Some(tree_sitter_java::LANGUAGE.into()),
        "c" => Some(tree_sitter_c::LANGUAGE.into()),
        "cpp" => Some(tree_sitter_cpp::LANGUAGE.into()),
        "rust" => Some(tree_sitter_rust::LANGUAGE.into()),
        "ruby" => Some(tree_sitter_ruby::LANGUAGE.into()),
        "bash" => Some(tree_sitter_bash::LANGUAGE.into()),
        "csharp" => Some(tree_sitter_c_sharp::LANGUAGE.into()),
        "swift" => Some(tree_sitter_swift::LANGUAGE.into()),
        "elixir" => Some(tree_sitter_elixir::LANGUAGE.into()),
        "lua" => Some(tree_sitter_lua::LANGUAGE.into()),
        "php" => Some(tree_sitter_php::LANGUAGE_PHP.into()),
        "scala" => Some(tree_sitter_scala::LANGUAGE.into()),
        "ocaml" => Some(tree_sitter_ocaml::LANGUAGE_OCAML.into()),
        "html" => Some(tree_sitter_html::LANGUAGE.into()),
        "css" => Some(tree_sitter_css::LANGUAGE.into()),
        "yaml" => Some(tree_sitter_yaml::LANGUAGE.into()),
        _ => None,
    }
}
