/// Maths but operator precedence is just left to right
/// parenthesis still override order.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day18::*;
///
/// let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 13632);
/// ```
pub fn part1(i: &String) -> i64 {
  i.split("\n")
    .filter_map(|line| match line.trim().len() {
      0 => None,
      _ => Some(arithmetic::parens_priority(line)),
    })
    .sum::<Result<i64, _>>()
    .unwrap()
}

/// Now addition has precedence over multiplication
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day18::*;
///
/// let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 13632);
/// ```
pub fn part2(i: &String) -> i64 {
  i.split("\n")
    .filter_map(|line| match line.trim().len() {
      0 => None,
      _ => Some(arithmetic::parens_addition_priority(line)),
    })
    .sum::<Result<i64, _>>()
    .unwrap()
}

peg::parser! {
    grammar arithmetic() for str {
        rule num() -> i64 = _ n:$(['0'..='9']) _ { n.parse().unwrap() }

        pub rule parens_priority() -> i64 = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "*" y:@ { x * y }
            --
            n:num() { n }
            _ "(" e:parens_priority() ")" _ { e }
        }

        pub rule parens_addition_priority() -> i64 = precedence!{
            x:(@) "*" y:@ { x * y }
            --
            x:(@) "+" y:@ { x + y }
            --
            n:num() { n }
            _ "(" e:parens_addition_priority() ")" _ { e }
        }

        rule _() = quiet!{[c if c.is_whitespace()]*}
    }
}
