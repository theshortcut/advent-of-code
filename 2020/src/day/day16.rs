use std::collections::HashMap;
use std::ops::Range;

/// Rules for ticket fields are inclusive ranges.
/// The order of fields in your ticket and nearby tickets are the same,
/// but you don't know which field is which.
///
/// Find the sum of all completely invalid fields
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day16::*;
///
/// let input = "class: 1-3 or 5-7
///row: 6-11 or 33-44
///seat: 13-40 or 45-50
///
///your ticket:
///7,1,14
///
///nearby tickets:
///7,3,47
///40,4,50
///55,2,20
///38,6,12".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 71);
/// ```
pub fn part1(i: &String) -> i64 {
  let (rules, _, nearby_tickets) = parse(i);
  let invalid_fields = find_invalid_fields(&rules, &nearby_tickets);
  invalid_fields.iter().sum()
}

struct Rule {
  field_name: String,
  ranges: Vec<Range<i64>>,
}

fn parse(i: &String) -> (Vec<Rule>, Vec<i64>, Vec<Vec<i64>>) {
  let parts: Vec<&str> = i.split("\n\n").collect();
  let rules = parts[0]
    .split("\n")
    .map(|line| {
      let parts: Vec<&str> = line.split(":").collect();
      let field_name = parts[0].into();
      let ranges = parts[1]
        .split("or")
        .map(|range_str| {
          let start_end: Vec<i64> = range_str
            .split("-")
            .map(|s| s.trim().parse().unwrap())
            .collect();
          start_end[0]..(start_end[1] + 1)
        })
        .collect();
      Rule { field_name, ranges }
    })
    .collect();
  let my_ticket = parts[1]
    .split("\n")
    .last()
    .unwrap()
    .split(",")
    .filter_map(|s| s.parse().ok())
    .collect();
  let tickets = parts[2]
    .split("\n")
    .skip(1)
    .map(|line| line.split(",").filter_map(|s| s.parse().ok()).collect())
    .collect();
  (rules, my_ticket, tickets)
}

fn find_invalid_fields(rules: &Vec<Rule>, tickets: &Vec<Vec<i64>>) -> Vec<i64> {
  tickets
    .iter()
    .flat_map(|ticket| {
      ticket.iter().cloned().filter(|field| {
        rules
          .iter()
          .all(|rule| rule.ranges.iter().all(|range| !range.contains(field)))
      })
    })
    .collect()
}

/// Discard the tickets with invalid fields.
/// Use the remaining tickets to determine the field order.
///
/// Find the product of the fields beginning with `departure` (the first 5 rules)
pub fn part2(i: &String) -> i64 {
  let (rules, my_ticket, nearby_tickets) = parse(i);
  let valid_tickets = filter_invalid_tickets(&rules, &nearby_tickets);
  let field_order = find_field_order(&rules, &valid_tickets);
  println!("{:?}", field_order);
  (0..6)
    .map(|i| {
      my_ticket[*field_order
        .iter()
        .find(|(_, &rule_idx)| rule_idx == i)
        .unwrap()
        .0]
    })
    .product()
}

fn filter_invalid_tickets(rules: &Vec<Rule>, tickets: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
  tickets
    .iter()
    .cloned()
    .filter(|ticket| {
      !ticket.iter().any(|field| {
        rules
          .iter()
          .all(|rule| rule.ranges.iter().all(|range| !range.contains(field)))
      })
    })
    .collect()
}

fn find_field_order(rules: &Vec<Rule>, tickets: &Vec<Vec<i64>>) -> HashMap<usize, usize> {
  let potentials = rules
    .iter()
    .map(|rule| {
      (0..rules.len())
        .filter(|idx| {
          tickets
            .iter()
            .map(|t| t[*idx])
            .all(|field| rule.ranges.iter().any(|r| r.contains(&field)))
        })
        .collect()
    })
    .collect();
  solve(&potentials)
}

fn solve(potentials: &Vec<Vec<usize>>) -> HashMap<usize, usize> {
  let mut solved_fields = HashMap::new();
  loop {
    if solved_fields.len() == potentials.len() {
      break;
    }
    let remaining_potentials = potentials
      .iter()
      .map(|p| {
        p.iter()
          .cloned()
          .filter(|i| !solved_fields.contains_key(i))
          .collect()
      })
      .collect();
    reduce_potentials(&mut solved_fields, &remaining_potentials);
  }
  solved_fields
}

fn reduce_potentials(solved_fields: &mut HashMap<usize, usize>, potentials: &Vec<Vec<usize>>) {
  potentials.iter().enumerate().for_each(|(idx, p)| {
    if p.len() == 1 {
      solved_fields.insert(p[0], idx);
    }
  })
}
