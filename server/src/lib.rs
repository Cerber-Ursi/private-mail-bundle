use log::{debug, error};
use rouille::{Request, Response};
use std::{error::Error, fs::read_to_string};
use structopt::StructOpt;
use toml::from_str;
use url::Url;

mod config;
mod routes;

use config::{CliOptions, Config};
use routes::route;

fn router(cfg: Config) -> impl Send + Sync + 'static + Fn(&Request) -> Response {
    let base = Url::parse("local:/").unwrap();
    move |request| {
        let uri = match base.join(request.raw_url()) {
            Ok(uri) => uri,
            Err(e) => {
                error!("{}", e.to_string());
                return Response::text(e.to_string()).with_status_code(500);
            }
        };
        debug!("Local uri: {:?}", uri);
        let mut parts = match uri.path_segments() {
            Some(parts) => parts,
            None => return route(routes::root()),
        };
        match parts.next() {
            Some(part) => match part {
                "mail" => route(routes::mailbox(&cfg, parts.collect())),
                "" => route(routes::root()),
                first => route(routes::resource(
                    &cfg,
                    std::iter::once(first).chain(parts).collect(),
                )),
            },
            None => route(routes::root()),
        }
    }
}

pub fn start() -> Result<(), Box<dyn Error>> {
    let opts = CliOptions::from_args();
    let cfg: Config = from_str(&read_to_string(&opts.cfg)?)?;
    stderrlog::new().verbosity(cfg.verbosity()).init()?;

    rouille::start_server(("127.0.0.1", cfg.port()), router(cfg))
}
