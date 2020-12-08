use std::collections::{HashMap, HashSet};
use std::io::Read;

use lazy_static::lazy_static;
use regex::Regex;

fn parse_rule(rule: &str) -> Option<(&str, HashSet<&str>)> {
    lazy_static! {
        static ref PAT1: Regex = Regex::new(r"(\w+ \w+) bags contain (no other bags|.*).").unwrap();
        static ref PAT2: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    }

    let captures = PAT1.captures(rule)?;
    let outer_bag = captures.get(1)?.as_str();
    let inner_bags = match captures.get(2)?.as_str() {
        "no other bags" => HashSet::new(),
        s => PAT2
            .captures_iter(s)
            .filter_map(|cap| Some(cap.get(2)?.as_str()))
            .collect(),
    };
    Some((outer_bag, inner_bags))
}

fn can_contain<'a>(
    bags: &HashMap<&'a str, HashSet<&'a str>>,
    seen: &mut HashSet<&'a str>,
    bag: &'a str,
    needle: &'a str,
) -> bool {
    if seen.contains(bag) {
        return false;
    }
    seen.insert(bag);

    bags[bag].contains(needle)
        || bags[bag]
            .iter()
            .any(|bag| can_contain(bags, seen, bag, needle))
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read from stdin");

    let bags = input
        .lines()
        .map(parse_rule)
        .collect::<Option<HashMap<_, _>>>()
        .unwrap();

    let needle = "shiny gold";

    let c = bags
        .keys()
        .filter(|bag| can_contain(&bags, &mut HashSet::new(), bag, needle))
        .count();
    println!("{}", c);
}
