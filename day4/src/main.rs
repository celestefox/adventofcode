extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let beginning = include_str!("input.txt").to_string().trim().trim_matches('\n').to_string();
    let mut number = 0;
    loop {
        let tohash = (beginning.clone() + &number.to_string());
        let mut hash = Md5::new();
        hash.input_str(&tohash);
        let result = hash.result_str();
        if result.starts_with("000000") {
            println!("{}", result);
            break;
        }
        number += 1;
    }
    println!("{}", number);
}
