mod save;

use rust_common::to_owned;
use save::write_json;
use std::io::Read;
use mailparse::parse_mail;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    let mail = parse_mail(&input);
    println!("{:?}", mail.map_err(|err| Box::new(err) as _).and_then(to_owned).and_then(write_json));
}
