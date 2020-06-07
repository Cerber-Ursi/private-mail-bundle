use mailparse::ParsedMail;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    #[serde(skip)]
    pub content: Vec<u8>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Multipart {
    Mixed(Vec<MailPart>),
    Alternative(Vec<MailPart>),
    Related(Vec<MailPart>),
    Other(Vec<MailPart>),
}

impl Multipart {
    pub fn inner(&self) -> &[MailPart] {
        use Multipart::*;
        match self {
            Mixed(v) => v,
            Alternative(v) => v,
            Related(v) => v,
            Other(v) => v,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailPart {
    pub body: Option<Body>,
    pub parts: Option<Multipart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    key: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mail {
    headers: Vec<Header>,
    pub main: MailPart,
}

impl Mail {
    pub fn find_header(&self, key: &str) -> Option<String> {
        self.headers
            .iter()
            .find(|h| h.key == key)
            .map(|h| h.value.clone())
    }
}

pub fn to_owned(mail: ParsedMail) -> Result<Mail, Box<dyn Error>> {
    let headers = mail
        .headers
        .iter()
        .map(|h| {
            Ok(Header {
                key: h.get_key()?,
                value: h.get_value()?,
            })
        })
        .collect::<Result<_, Box<dyn Error>>>()?;
    let main = to_owned_part(mail, "ROOT");
    Ok(Mail { headers, main })
}

fn to_owned_part(mail: ParsedMail, path: &str) -> MailPart {
    let body = extract_body(&mail, path);
    let parts = if mail.ctype.mimetype.starts_with("multipart/") {
        Some(match mail.ctype.mimetype.as_str() {
            "multipart/related" => Multipart::Related,
            "multipart/mixed" => Multipart::Mixed,
            "multipart/alternative" => Multipart::Alternative,
            _ => Multipart::Other,
        }(
            mail.subparts
                .into_iter()
                .enumerate()
                .map(|(index, mail)| to_owned_part(mail, &format!("{}/{}", path, index)))
                .collect::<Vec<_>>(),
        ))
    } else {
        None
    };
    MailPart { body, parts }
}

fn extract_body(mail: &ParsedMail, path: &str) -> Option<Body> {
    let content = mail
        .get_body_raw()
        .ok()
        .and_then(|v| if v.is_empty() { None } else { Some(v) });
    let name = if let Some(filename) = mail.ctype.params.get("filename") {
        filename.to_owned()
    } else {
        match mail.ctype.mimetype.as_str() {
            "text/plain" => path.to_owned() + ".txt",
            "text/html" => path.to_owned() + ".html",
            _ => path.to_owned(),
        }
    };
    content.map(|content| Body { content, name })
}
