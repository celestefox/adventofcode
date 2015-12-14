extern crate regex;
extern crate pcre;

use regex::Regex;
use pcre::Pcre;

fn is_match(line: &str) -> bool {
    let mut aeiou : pcre::Pcre = Pcre::compile(r".*?[aeiou].*[aeiou].*[aeiou].*?").unwrap();
    let mut dc : pcre::Pcre = Pcre::compile(r".*?([a-z])\1.*?").unwrap();
    let mut bl = Pcre::compile(r".*?(ab|cd|pq|xy).*?").unwrap();
    (aeiou.exec(line).is_some()) &&
            (dc.exec(line).is_some()) &&
            (bl.exec(line).is_none())
}

fn is_part2(line: &str) -> bool {
    let mut pair = Pcre::compile(r".*?(..).*?\1.*?").unwrap();
    let mut between = Pcre::compile(r"(.).\1").unwrap();
    pair.exec(line).is_some() && between.exec(line).is_some()
}

fn main() {
    println!("{}", is_match("ugknbfddgicrmopn"));
    println!("{}", is_match("aaa"));
    println!("{}", is_match("jchzalrnumimnmhp"));
    println!("{}", is_match("haegwjzuvuyypxyu"));
    println!("{}", is_match("dvszwmarrgswjxmb"));
    let input = include_str!("input.txt");
    let mut count = 0;
    let mut count2 = 0;
    for line in input.lines() {
        if is_match(line) {
            count += 1;
        }
        if is_part2(line) {
            count2 += 1;
        }
    }
    println!("Nice lines: {}", count);
    println!("New rules nice lines: {}", count2);
}
