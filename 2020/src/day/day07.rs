use regex::Regex;
use std::collections::HashSet;

/// Each line is a rule about what bags a certain color bag may contain.
/// You have a shiny gold bag. Find how many bags may contain a shiny gold bag.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day07::*;
///
/// let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
///dark orange bags contain 3 bright white bags, 4 muted yellow bags.
///bright white bags contain 1 shiny gold bag.
///muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
///shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
///dark olive bags contain 3 faded blue bags, 4 dotted black bags.
///vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
///faded blue bags contain no other bags.
///dotted black bags contain no other bags.".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 4);
/// ```
pub fn part1(i: &String) -> usize {
  let rules = parse_rules(i);
  count_can_contain("shiny gold".into(), &rules)
}

#[derive(Debug)]
struct BagRule {
  color: String,
  count: usize,
}

#[derive(Debug)]
struct Rule {
  color: String,
  contents: Vec<BagRule>,
}

fn parse_rules(i: &String) -> Vec<Rule> {
  i.split("\n")
    .map(|line| {
      let re1 = Regex::new(r"^(.*) bags contain (.*).$").unwrap();
      let re2 = Regex::new(r"(\d+) (.*) bag").unwrap();
      re1.captures(line).map(|groups| {
        let color: String = groups.get(1).unwrap().as_str().into();
        let contents: Vec<BagRule> = groups
          .get(2)
          .unwrap()
          .as_str()
          .split(",")
          .map(|rule_str| {
            re2.captures(rule_str).map(|groups| {
              let color: String = groups.get(2).unwrap().as_str().into();
              let count: usize = groups.get(1).unwrap().as_str().parse().ok().unwrap();
              BagRule { color, count }
            })
          })
          .flatten()
          .collect();
        Rule { color, contents }
      })
    })
    .flatten()
    .collect()
}

fn get_possible_contents(color: &String, rules: &Vec<Rule>, results: &mut HashSet<String>) {
  let rule = rules.iter().find(|r| r.color == *color).unwrap();
  rule.contents.iter().for_each(|bag_rule| {
    results.insert(bag_rule.color.to_string());
    get_possible_contents(&bag_rule.color, rules, results);
  });
}

fn count_can_contain(color: String, rules: &Vec<Rule>) -> usize {
  rules
    .iter()
    .map(|rule| {
      let mut contents = HashSet::new();
      get_possible_contents(&rule.color, rules, &mut contents);
      if contents.contains(&color) {
        Some(&color)
      } else {
        None
      }
    })
    .filter(|c| c.is_some())
    .count()
}

/// How many bags must your shiny gold bag contain
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day07::*;
///
/// let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
///dark orange bags contain 3 bright white bags, 4 muted yellow bags.
///bright white bags contain 1 shiny gold bag.
///muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
///shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
///dark olive bags contain 3 faded blue bags, 4 dotted black bags.
///vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
///faded blue bags contain no other bags.
///dotted black bags contain no other bags.".to_string();
/// let result = part2(&input);
/// assert_eq!(result, 32);
/// ```
pub fn part2(i: &String) -> usize {
  let rules = parse_rules(i);
  count_contents("shiny gold".into(), &rules)
}

fn count_contents(color: String, rules: &Vec<Rule>) -> usize {
  let mut count: usize = 0;
  get_contents_count(&color, rules, &mut count, 1);
  count
}

fn get_contents_count(color: &String, rules: &Vec<Rule>, count: &mut usize, multiplier: usize) {
  let rule = rules.iter().find(|r| r.color == *color).unwrap();
  rule.contents.iter().for_each(|bag_rule| {
    *count = *count + (bag_rule.count * multiplier);
    get_contents_count(&bag_rule.color, rules, count, multiplier * bag_rule.count);
  });
}
