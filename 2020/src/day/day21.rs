use std::collections::{HashMap, HashSet};

/// Food ingredients and alergens.
/// Alergens may not always be listed.
/// Find the ingredients that do not contain listed alergens,
/// then count the number of times those ingredients appear
///
/// # Example
/// ```
/// use advent_of_code_2020::day::day21::*;
///
/// let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
///trh fvjkl sbzzf mxmxvkd (contains dairy)
///sqjhc fvjkl (contains soy)
///sqjhc mxmxvkd sbzzf (contains fish)".to_string();
/// let results = part1(&input);
/// assert_eq!(results, 5);
/// ```
pub fn part1(i: &String) -> usize {
  let foods = parse(i);
  let all_ingredients_vec: Vec<String> = foods.iter().flat_map(|f| f.ingredients.clone()).collect();
  let all_ingredients_set: HashSet<String> = all_ingredients_vec.iter().cloned().collect();
  let by_alergen = ingredient_by_alergen(&foods);
  let alergen_candidates: HashSet<String> = by_alergen.values().cloned().flatten().collect();
  all_ingredients_set
    .difference(&alergen_candidates)
    .fold(0, |count, ingredient| {
      count
        + all_ingredients_vec
          .iter()
          .filter(|s| *s == ingredient)
          .count()
    })
}

#[derive(Clone, Debug)]
struct Food {
  ingredients: Vec<String>,
  alergens: Vec<String>,
}

fn parse(i: &String) -> Vec<Food> {
  i.split("\n")
    .map(|line| {
      let parts: Vec<&str> = line.split("(").collect();
      let ingredients = parts[0].trim().split(" ").map(|s| s.to_string()).collect();
      let alergens = if parts.len() > 1 {
        parts[1]
          .replace(")", "")
          .replace("contains", "")
          .trim()
          .split(", ")
          .map(|s| s.to_string())
          .collect()
      } else {
        vec![]
      };
      Food {
        ingredients,
        alergens,
      }
    })
    .collect()
}

fn ingredient_by_alergen(foods: &Vec<Food>) -> HashMap<String, HashSet<String>> {
  let mut results: HashMap<String, HashSet<String>> = HashMap::new();
  foods.iter().cloned().for_each(|food| {
    let current_ingredients: HashSet<String> = food.ingredients.iter().cloned().collect();
    food
      .alergens
      .iter()
      .cloned()
      .for_each(|alergen| match results.clone().get(&alergen) {
        Some(ingredients) => {
          results.insert(
            alergen,
            ingredients
              .intersection(&current_ingredients)
              .map(|s| s.to_string())
              .collect(),
          );
        }
        None => {
          results.insert(alergen, current_ingredients.clone());
        }
      });
  });
  results
}

/// Now that you've isolated the inert ingredients, you should have enough information to figure out which ingredient contains which allergen.
/// Arrange the ingredients alphabetically by their allergen and separate them by commas to produce your canonical dangerous ingredient list. (There should not be any spaces in your canonical dangerous ingredient list.) In the above example, this would be mxmxvkd,sqjhc,fvjkl.
///
/// # Example
/// ```
/// use advent_of_code_2020::day::day21::*;
///
/// let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
///trh fvjkl sbzzf mxmxvkd (contains dairy)
///sqjhc fvjkl (contains soy)
///sqjhc mxmxvkd sbzzf (contains fish)".to_string();
/// let results = part2(&input);
/// assert_eq!(results, "mxmxvkd,sqjhc,fvjkl");
/// ```
pub fn part2(i: &String) -> String {
  let foods = parse(i);
  let by_alergen = ingredient_by_alergen(&foods);
  let mut alergens: Vec<&String> = by_alergen.keys().collect();
  alergens.sort();
  let alergen_map = narrow_candidates(&by_alergen);
  alergens
    .iter()
    .cloned()
    .filter_map(|alergen| alergen_map.get(alergen))
    .cloned()
    .collect::<Vec<String>>()
    .join(",")
}

fn narrow_candidates(candidates: &HashMap<String, HashSet<String>>) -> HashMap<String, String> {
  let mut pending = candidates.clone();
  let mut results: HashMap<String, String> = HashMap::new();
  loop {
    if pending.iter().all(|(_, c)| c.is_empty()) {
      break;
    }
    pending.clone().iter().for_each(|(alergen, ingredients)| {
      if ingredients.len() == 1 {
        let locked_ingredient = ingredients.iter().next().unwrap();
        results.insert(alergen.to_string(), locked_ingredient.to_string());
        for (_, val) in pending.iter_mut() {
          val.remove(locked_ingredient);
        }
      }
    });
  }
  results
}
