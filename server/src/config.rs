use serde::Deserialize;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[non_exhaustive]
#[derive(StructOpt)]
pub struct CliOptions {
    #[structopt(long)]
    pub cfg: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    mailbox_path: PathBuf,
    web_path: PathBuf,
    port: u16,
    verbosity: usize,
}

impl Config {
    pub fn mailbox_path(&self) -> &Path {
        &self.mailbox_path
    }

    pub fn web_path(&self) -> &Path {
        &self.web_path
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn verbosity(&self) -> usize {
        self.verbosity
    }
}
