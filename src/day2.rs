use std::fs;
use std::str::Split;

#[derive(PartialEq, Debug)]
enum MatchResult {
    WIN,
    DRAW,
    LOSE,
}

impl MatchResult {
    fn point_value(&self) -> u8 {
        match *self {
            MatchResult::WIN => 6,
            MatchResult::DRAW => 3,
            MatchResult::LOSE => 0,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Choice {
    fn get_match_result(&self, opponent_choice: &Choice) -> MatchResult {
        if opponent_choice == self {
            MatchResult::DRAW
        } else if self.wins_against(opponent_choice) {
            MatchResult::WIN
        } else {
            MatchResult::LOSE
        }
    }

    fn wins_against(&self, opponent_choice: &Choice) -> bool {
        *opponent_choice == self.get_weak_opponent()
    }

    fn get_weak_opponent(&self) -> Choice {
        match *self {
            Choice::ROCK => Choice::SCISSORS,
            Choice::PAPER => Choice::ROCK,
            Choice::SCISSORS => Choice::PAPER,
        }
    }

    fn get_strong_opponent(&self) -> Choice {
        match *self {
            Choice::ROCK => Choice::PAPER,
            Choice::PAPER => Choice::SCISSORS,
            Choice::SCISSORS => Choice::ROCK,
        }
    }

    fn point_value(&self) -> u8 {
        match *self {
            Choice::ROCK => 1,
            Choice::PAPER => 2,
            Choice::SCISSORS => 3,
        }
    }

    fn get_user_choice_for_user_result(&self, desired_result: &MatchResult) -> Choice {
        match *desired_result {
            MatchResult::WIN => self.get_strong_opponent(),
            MatchResult::DRAW => self.clone(),
            MatchResult::LOSE => self.get_weak_opponent(),
        }
    }
}

pub fn day2() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let lines = contents.split("\n");

    let lines: Vec<&str> = lines.into_iter().collect();
    let total = calculate_total_score_part1(lines.clone());

    println!("Total score (part 1): {}", total);

    let total = calculate_total_score_part2(lines);

    println!("Total score (part 2): {}", total);
}

fn calculate_total_score_part1(lines: Vec<&str>) -> u32 {
    let mut total: u32 = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let opponent_choice = letter_to_choice(&line[0..1]);
        let user_choice = letter_to_choice(&line[2..3]);
        total += user_choice.point_value() as u32;
        total += user_choice.get_match_result(&opponent_choice).point_value() as u32;
    }

    total
}

fn calculate_total_score_part2(lines: Vec<&str>) -> u32 {
    let mut total: u32 = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let opponent_choice = letter_to_choice(&line[0..1]);
        let desired_result = letter_to_result(&line[2..3]);
        let user_choice = opponent_choice.get_user_choice_for_user_result(&desired_result);
        total += user_choice.point_value() as u32;
        total += user_choice.get_match_result(&opponent_choice).point_value() as u32;
    }

    total
}

fn letter_to_choice(letter: &str) -> Choice {
    match letter {
        "A" => Choice::ROCK,
        "B" => Choice::PAPER,
        "C" => Choice::SCISSORS,
        "X" => Choice::ROCK,
        "Y" => Choice::PAPER,
        "Z" => Choice::SCISSORS,
        _ => unimplemented!()
    }
}

fn letter_to_result(letter: &str) -> MatchResult {
    match letter {
        "X" => MatchResult::LOSE,
        "Y" => MatchResult::DRAW,
        "Z" => MatchResult::WIN,
        _ => unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Choice::*;
    use super::MatchResult::*;

    #[test]
    fn test_example_strategy_part1_scores_15() {
        let lines = vec!["A Y", "B X", "C Z", ""];
        let total = calculate_total_score_part1(lines);
        assert_eq!(15, total)
    }

    #[test]
    fn test_example_strategy_part2_scores_12() {
        let lines = vec!["A Y", "B X", "C Z"];
        let total = calculate_total_score_part2(lines);
        assert_eq!(12, total)
    }

    #[test]
    fn test_rock_beats_scissors() {
        assert_eq!(WIN, ROCK.get_match_result(&SCISSORS))
    }

    #[test]
    fn test_paper_beats_rock() {
        assert_eq!(WIN, PAPER.get_match_result(&ROCK))
    }

    #[test]
    fn test_scissors_beats_paper() {
        assert_eq!(WIN, SCISSORS.get_match_result(&PAPER))
    }

    #[test]
    fn test_rock_loses_to_paper() {
        assert_eq!(LOSE, ROCK.get_match_result(&PAPER))
    }

    #[test]
    fn test_paper_loses_to_scissors() {
        assert_eq!(LOSE, PAPER.get_match_result(&SCISSORS))
    }

    #[test]
    fn test_scissors_loses_to_rock() {
        assert_eq!(LOSE, SCISSORS.get_match_result(&ROCK))
    }

    #[test]
    fn test_rock_draw_against_rock() {
        assert_eq!(DRAW, ROCK.get_match_result(&ROCK))
    }

    #[test]
    fn test_paper_draw_against_paper() {
        assert_eq!(DRAW, PAPER.get_match_result(&PAPER))
    }

    #[test]
    fn test_scissors_draw_against_scissors() {
        assert_eq!(DRAW, SCISSORS.get_match_result(&SCISSORS))
    }
}