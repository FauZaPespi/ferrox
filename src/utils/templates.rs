use html_escape::encode_safe;

const ERROR_TEMPLATE: &str = include_str!("../../templates/error.html");
const INDEXING_TEMPLATE: &str = include_str!("../../templates/indexing.html");

/// Renders an error page by replacing placeholders in the error template.
///
/// # Arguments
///
/// * `code` - The HTTP error status code.
/// * `message` - The error message to display.
pub fn render_error(code: &str, message: &str) -> Vec<u8> {
    ERROR_TEMPLATE.replace("{{CODE}}", code).replace("{{MESSAGE}}", message).into_bytes()
}

/// Renders a directory indexing page by replacing placeholders in the indexing template.
///
/// # Arguments
///
/// * `title` - The title of the indexing page, typically the directory name.
/// * `list` - The HTML list of files and directories within the directory.
pub fn render_indexing(title: &str, list: &str) -> Vec<u8> {
    let safe_title = encode_safe(title);
    INDEXING_TEMPLATE.replace("{{TITLE}}", &safe_title).replace("{{LISTING}}", list).into_bytes()
}