use mailparse::{ParsedMail};
use std::error::Error;

#[derive(Debug)]
pub enum Body {
    Text(String),
    Binary(Vec<u8>),
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
    if mail.subparts.len() > 0 {
        Ok(MailPart { body: None, parts: mail.subparts.into_iter().map(to_owned_part).collect::<Result<_, _>>()? })
    } else {
        // TODO differentiate between strings and binary data; now going conservative
        Ok(MailPart { body: mail.get_body_raw().ok().map(Body::Binary), parts: vec![] })
    }
}

