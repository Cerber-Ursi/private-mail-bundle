use crate::config::Config;
use log::{debug, error};
use rouille::Response;
use std::{convert::Infallible, error::Error, fs::File, path::PathBuf};
use url::form_urlencoded::Parse;

mod statics;

pub fn route<E: Error>(result: Result<Response, E>) -> Response {
    match result {
        Ok(res) => res,
        Err(e) => {
            error!("{}", e.to_string());
            Response::text(e.to_string()).with_status_code(500)
        }
    }
}

pub fn root() -> Result<Response, Infallible> {
    Ok(Response::redirect_303("index"))
}

pub fn resource(cfg: &Config, path: PathBuf) -> Result<Response, std::io::Error> {
    let mut path = cfg.web_path().join(path);
    if path.extension().is_none() {
        path = path.with_extension("html");
    }
    debug!("Loading web resource from path: {:?}", path.to_str());
    Ok(Response::from_file(
        statics::mime_type_of(&path),
        File::open(&path)?,
    ))
}

pub fn mailbox(cfg: &Config, path: PathBuf) -> Result<Response, std::io::Error> {
    let path = cfg.mailbox_path().join(path);
    debug!("Loading mailbox data from path: {:?}", path.to_str());
    Ok(Response::from_file(
        statics::mime_type_of(&path),
        File::open(&path)?,
    ))
}
