use mailparse::ParsedMail;
use std::error::Error;

#[derive(Debug)]
pub struct Body {
    content: Vec<u8>,
    name: String,
}

#[derive(Debug)]
pub struct MailPart {
    body: Option<Body>,
    parts: Vec<MailPart>,
}

#[derive(Debug)]
pub struct Header {
    key: String,
    value: String,
}

#[derive(Debug)]
pub struct Mail {
    headers: Vec<Header>,
    main: MailPart,
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
    let main = to_owned_part(mail)?;
    Ok(Mail { headers, main })
}

fn to_owned_part(mail: ParsedMail) -> Result<MailPart, Box<dyn Error>> {
    Ok(MailPart {
        body: extract_body(&mail),
        parts: mail
            .subparts
            .into_iter()
            .map(to_owned_part)
            .collect::<Result<_, _>>()?,
    })
}

fn extract_body(mail: &ParsedMail) -> Option<Body> {
    let content = mail
        .get_body_raw()
        .ok()
        .and_then(|v| if v.len() > 0 { Some(v) } else { None });
    let name = if let Some(filename) = mail.ctype.params.get("filename") {
        filename
    } else {
        match mail.ctype.mimetype.as_str() {
            "text/plain" => "file.txt",
            "text/html" => "file.html",
            _ => "file",
        }
    }
    .to_owned();
    content.map(|content| Body { content, name })
}
