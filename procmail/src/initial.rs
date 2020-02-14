use mailparse::{parse_mail, ParsedMail};
use std::error::Error;
use std::io::Read;

#[derive(Debug)]
enum Body {
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Debug)]
struct MailPart {
    body: Option<Body>,
    parts: Vec<MailPart>,
}

#[derive(Debug)]
struct Header {
    key: String,
    value: String,
}

#[derive(Debug)]
struct Mail {
    headers: Vec<Header>,
    main: MailPart,
}

fn to_owned(mail: ParsedMail) -> Result<Mail, Box<dyn Error>> {
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
    let parts = mail.subparts;
    Ok(Mail { headers, parts })
}

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    let mail = parse_mail(&input);
    println!("{:?}", mail.map(to_owned));
}
