use std::collections::{HashMap, HashSet};
use std::fs;

pub fn day3() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let lines = contents.split("\n");

    let lines: Vec<&str> = lines.into_iter().collect();
    println!("Common priority sum: {}", get_common_priority_sum(&lines));
    println!("Badges priority sum: {}", get_badge_priority_sum(&lines));
}

fn get_common_priority_sum(lines: &Vec<&str>) -> u32 {
    let mut total = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let midpoint = line.chars().count() / 2;
        let compartment1: &str = &line[..midpoint];
        let compartment2: &str = &line[midpoint..];
        let compartment1: HashSet<char> = compartment1.chars().into_iter().collect();
        let compartment2: HashSet<char> = compartment2.chars().into_iter().collect();
        let common_character: Vec<&char> = compartment1.intersection(&compartment2).collect();
        let common_character = common_character.first();

        if common_character.is_some() {
            let common_character = common_character.unwrap();
            total += get_score(*common_character) as u32;
        }
    }

    total
}

fn get_badge_priority_sum(lines: &Vec<&str>) -> u32 {
    let mut total = 0;
    let groups = lines.chunks(3);
    for group in groups {
        if group.len() < 3 {
            continue;
        }
        let rucksack1: HashSet<char> = group.first().unwrap().chars().into_iter().collect();
        let rucksack2: HashSet<char> = group.get(1).unwrap().chars().into_iter().collect();
        let rucksack3: HashSet<char> = group.last().unwrap().chars().into_iter().collect();
        let common_character: Vec<&char> = rucksack1.intersection(&rucksack2)
            .into_iter()
            .filter(|character| rucksack3.contains(character))
            .collect();
        let common_character = common_character.first();

        if common_character.is_some() {
            let common_character = common_character.unwrap();
            total += get_score(*common_character) as u32;
        }
    }

    total
}

fn get_score(character: &char) -> u8 {
    let mut score = character.to_ascii_lowercase() as u32 - 'a' as u32 + 1;
    if character.is_uppercase() {
        score += 26;
    }
    score as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1_sum_is_157() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
            "",
        ];
        let actual = get_common_priority_sum(&lines);
        assert_eq!(157, actual)
    }

    #[test]
    fn test_example_part2_sum_is_70() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
            "",
        ];
        let actual = get_badge_priority_sum(&lines);
        assert_eq!(70, actual)
    }
}