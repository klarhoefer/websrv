

pub enum MimeType {
    Text, Html, Js, Css, Ico
}

impl MimeType {
    // https://wiki.selfhtml.org/wiki/MIME-Type/%C3%9Cbersicht
    pub fn as_str(&self) -> &str {
        match self {
            MimeType::Html => "text/html",
            MimeType::Text => "text/plain",
            MimeType::Css => "text/css",
            MimeType::Js => "text/javascript",
            MimeType::Ico => "image/x-icon",
        }
    }
}
