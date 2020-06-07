use std::path::Path;

pub fn mime_type_of(path: impl AsRef<Path>) -> &'static str {
    match path.as_ref().extension().and_then(|s| s.to_str()) {
        Some("css") => "text/css",
        Some("json") => "application/json",
        Some("js") => "text/javascript",
        // TODO add other types as they are in the code base
        _ => "text/html",
    }
}
