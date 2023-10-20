use crate::utilities::file_utilities::read_lines;
use itertools::Itertools;

use rustc_hash::{FxHashMap, FxHashSet};

fn parse_line(line: &str) -> (Vec<String>, Vec<String>) {
    let split_line: Vec<&str> = line
        .split(&['(', ')'])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let ingredients: Vec<String> = split_line[0]
        .split(' ')
        .map(str::trim)
        .map(|s| s.to_string())
        .collect();

    let allergens: Vec<String> = split_line[1]
        .split(&[' ', ','])
        .filter(|s| !s.is_empty())
        .skip(1)
        .map(str::trim)
        .map(|s| s.to_string())
        .collect();

    (ingredients, allergens)
}

fn parse_data(file_path: String) -> Vec<(Vec<String>, Vec<String>)> {
    read_lines(file_path)
        .into_iter()
        .map(|line| parse_line(line.as_str()))
        .collect()
}

fn get_allergens_mapping(
    foods: &[(Vec<String>, Vec<String>)],
    all_ingredients: &FxHashSet<String>,
    all_allergens: &FxHashSet<String>,
) -> FxHashMap<String, String> {
    let mut ingredients_to_allergens_result = FxHashMap::default();

    let mut unknown_ingredients_remaining = all_ingredients.clone();
    let mut unknown_allergens_remaining = all_allergens.clone();

    while !unknown_allergens_remaining.is_empty() {
        for allergen in all_allergens.iter() {
            if !unknown_allergens_remaining.contains(allergen) {
                continue;
            }

            let foods_that_have_it = foods
                .iter()
                .filter(|food| food.1.contains(allergen))
                .collect::<Vec<_>>();
            // println!("I know that {allergen} is in {foods_that_have_it:?}");

            let ingredients_in_common = unknown_ingredients_remaining
                .iter()
                .filter(|ingredient| {
                    foods_that_have_it
                        .iter()
                        .all(|food| food.0.contains(ingredient))
                })
                .collect::<Vec<_>>();

            // println!("And those foods have {ingredients_in_common:?} in common");

            match ingredients_in_common.len() {
                0 => panic!(
                    "You should have something here, otherwise you forgot to update your sets!"
                ),
                1 => {
                    let ingredient_in_common = *ingredients_in_common.first().unwrap();

                    ingredients_to_allergens_result
                        .insert(ingredient_in_common.clone(), allergen.clone());
                    unknown_allergens_remaining.remove(allergen);
                    unknown_ingredients_remaining.remove(&ingredient_in_common.clone());
                    break;
                }
                _ => continue,
            }
        }
    }

    ingredients_to_allergens_result
}

pub fn part_1(file_path: String) -> i64 {
    let foods = parse_data(file_path);

    let all_ingredients: FxHashSet<String> = foods.iter().flat_map(|food| food.0.clone()).collect();
    let all_allergens: FxHashSet<String> = foods.iter().flat_map(|food| food.1.clone()).collect();

    let ingredients_with_allergens: FxHashSet<String> =
        get_allergens_mapping(&foods, &all_ingredients, &all_allergens)
            .keys()
            .cloned()
            .collect();

    let non_allergenic_ingredients: FxHashSet<String> = all_ingredients
        .difference(&ingredients_with_allergens)
        .cloned()
        .collect();

    // println!("Non-allergenic: {non_allergenic_ingredients:?}");

    return foods
        .iter()
        .flat_map(|food| food.0.clone())
        .filter(|ingredient| non_allergenic_ingredients.contains(ingredient))
        .count() as i64;
}

pub fn part_2(file_path: String) -> String {
    let foods = parse_data(file_path);

    let all_ingredients: FxHashSet<String> = foods.iter().flat_map(|food| food.0.clone()).collect();
    let all_allergens: FxHashSet<String> = foods.iter().flat_map(|food| food.1.clone()).collect();

    let canonical_list = get_allergens_mapping(&foods, &all_ingredients, &all_allergens)
        .iter()
        .sorted_by(|kvp_1, kvp_2| kvp_1.1.partial_cmp(kvp_2.1).unwrap())
        .map(|(key, _)| key)
        .join(",");

    canonical_list
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::utilities::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 5)]
    #[case(false, 2573)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 21, None)));
    }

    #[rstest]
    #[case(true, "mxmxvkd,sqjhc,fvjkl")]
    #[case(false, "bjpkhx,nsnqf,snhph,zmfqpn,qrbnjtj,dbhfd,thn,sthnsg")]
    fn test_part_2(#[case] is_test: bool, #[case] expected: String) {
        assert_eq!(expected, part_2(get_file_path(is_test, 21, None)));
    }
}
