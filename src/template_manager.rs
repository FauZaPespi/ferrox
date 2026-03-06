const TEMPLATE: &str = include_str!("../templates/error.html");

pub fn render(code: &str, message: &str) -> String {
    TEMPLATE.replace("{{CODE}}", code).replace("{{MESSAGE}}", message)
}