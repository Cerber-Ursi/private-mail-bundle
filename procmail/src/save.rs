use rust_common::{Mail, MailPart};
use std::error::Error;
use std::fs::{File, create_dir_all};
use std::path::{Path, PathBuf};
use std::io::Write;
use serde_json::to_string;

pub fn write_json(mail: Mail) -> Result<(), Box<dyn Error>> {
    let recipient = mail.find_header("To").ok_or("No To header found")?;
    let id = mail.find_header("Message-ID").ok_or("No ID found")?;
    let mut path: PathBuf = [
        "mailbox",
        recipient.split("@").next().ok_or("Recipient is empty")?,
        &id,
    ].iter().collect();
    create_dir_all(&path)?;
    path.push("info.json");
    write_part(&path.with_file_name(""), &mail.main)?;
    write!(File::create(path)?, "{}", to_string(&mail)?)?;
    Ok(())
}

fn write_part(base_path: &Path, mail: &MailPart) -> Result<(), Box<dyn Error>> {
    if let Some(body) = &mail.body {
        let path = base_path.join(&body.name);
        create_dir_all(&path.with_file_name(""))?;
        File::create(path)?.write(&body.content)?;
    }
    if let Some(parts) = &mail.parts {
        for part in parts.inner().iter() {
            write_part(base_path, part)?;
        }
    }
    Ok(())
}