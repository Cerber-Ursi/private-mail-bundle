mod owned;

use owned::to_owned;
use std::io::Read;
use mailparse::parse_mail;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    let mail = parse_mail(&input);
    println!("{:?}", mail.map(to_owned));
}
