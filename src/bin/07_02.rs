use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

fn parse_rule(rule: &str) -> (&str, Vec<(i32, &str)>) {
    lazy_static! {
        static ref PAT1: Regex = Regex::new(r"(\w+ \w+) bags contain (no other bags|.*).").unwrap();
        static ref PAT2: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    }

    let captures = PAT1.captures(rule).unwrap();
    let outer_bag = captures.get(1).unwrap().as_str();
    let inner_bags = match captures.get(2).unwrap().as_str() {
        "no other bags" => vec![],
        s => PAT2
            .captures_iter(s)
            .map(|cap| {
                (
                    i32::from_str(cap.get(1).unwrap().as_str()).unwrap(),
                    cap.get(2).unwrap().as_str(),
                )
            })
            .collect(),
    };
    (outer_bag, inner_bags)
}

fn fits<'a>(bags: &HashMap<&'a str, Vec<(i32, &'a str)>>, bag: &'a str) -> i32 {
    bags[bag]
        .iter()
        .map(|(cnt, bag)| cnt + cnt * fits(bags, bag))
        .sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read from stdin");

    let bags = input.lines().map(parse_rule).collect::<HashMap<_, _>>();

    println!("{}", fits(&bags, "shiny gold"));
}
