use crate::models::CommentInfo;

pub fn build_comments_xml(comments: &[CommentInfo], file_path: &str) -> String {
    if comments.is_empty() {
        return String::new();
    }
    let mut sb = String::new();
    sb.push_str(&format!("<comments file=\"{}\">\n", file_path));
    for comment in comments {
        sb.push_str(&format!(
            "\t<comment line-number=\"{}\">{}</comment>\n",
            comment.line_number(),
            comment.text()
        ));
    }
    sb.push_str("</comments>");
    sb
}
