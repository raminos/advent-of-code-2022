use std::{env, fs};

enum Element {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

#[derive(Debug)]
struct PointsPerOutcome {
    lose: i32,
    draw: i32,
    win: i32,
}

#[derive(Debug)]
struct PointsPerElement {
    rock: i32,
    paper: i32,
    scissors: i32,
}

const POINTS_PER_ELEMENT: PointsPerElement = PointsPerElement {
    rock: 1,
    paper: 2,
    scissors: 3,
};
const POINTS_PER_OUTCOME: PointsPerOutcome = PointsPerOutcome {
    lose: 0,
    draw: 3,
    win: 6,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut contents = fs::read_to_string(file_path).unwrap();
    // Last line is not needed
    contents.pop();

    let moves: Vec<&str> = contents.split("\n").collect();
    let moves_per_user: Vec<Vec<&str>> = moves
        .iter()
        .map(|match_string| match_string.split(" ").collect::<Vec<&str>>())
        .collect();

    let scores: Vec<i32> = moves_per_user
        .iter()
        .map(|values| get_part_one_score(get_element(values[1]), get_element(values[0])))
        .collect();
    let result: i32 = scores.iter().sum();

    println!("Result Part One: {result}");

    let scores: Vec<i32> = moves_per_user
        .iter()
        .map(|values| get_part_two_score(get_outcome(values[1]), get_element(values[0])))
        .collect();

    let result: i32 = scores.iter().sum();

    println!("Result Part Two: {result}");
}

fn get_part_one_score(user_element: Element, opponent_element: Element) -> i32 {
    match user_element {
        Element::Rock => match opponent_element {
            Element::Rock => POINTS_PER_ELEMENT.rock + POINTS_PER_OUTCOME.draw,
            Element::Paper => POINTS_PER_ELEMENT.rock + POINTS_PER_OUTCOME.lose,
            Element::Scissors => POINTS_PER_ELEMENT.rock + POINTS_PER_OUTCOME.win,
        },
        Element::Paper => match opponent_element {
            Element::Rock => POINTS_PER_ELEMENT.paper + POINTS_PER_OUTCOME.win,
            Element::Paper => POINTS_PER_ELEMENT.paper + POINTS_PER_OUTCOME.draw,
            Element::Scissors => POINTS_PER_ELEMENT.paper + POINTS_PER_OUTCOME.lose,
        },
        Element::Scissors => match opponent_element {
            Element::Rock => POINTS_PER_ELEMENT.scissors + POINTS_PER_OUTCOME.lose,
            Element::Paper => POINTS_PER_ELEMENT.scissors + POINTS_PER_OUTCOME.win,
            Element::Scissors => POINTS_PER_ELEMENT.scissors + POINTS_PER_OUTCOME.draw,
        },
    }
}

fn get_part_two_score(outcome: Outcome, opponent_element: Element) -> i32 {
    match outcome {
        Outcome::Win => match opponent_element {
            Element::Rock => POINTS_PER_OUTCOME.win + POINTS_PER_ELEMENT.paper,
            Element::Paper => POINTS_PER_OUTCOME.win + POINTS_PER_ELEMENT.scissors,
            Element::Scissors => POINTS_PER_OUTCOME.win + POINTS_PER_ELEMENT.rock,
        },
        Outcome::Draw => match opponent_element {
            Element::Rock => POINTS_PER_OUTCOME.draw + POINTS_PER_ELEMENT.rock,
            Element::Paper => POINTS_PER_OUTCOME.draw + POINTS_PER_ELEMENT.paper,
            Element::Scissors => POINTS_PER_OUTCOME.draw + POINTS_PER_ELEMENT.scissors,
        },
        Outcome::Lose => match opponent_element {
            Element::Rock => POINTS_PER_OUTCOME.lose + POINTS_PER_ELEMENT.scissors,
            Element::Paper => POINTS_PER_OUTCOME.lose + POINTS_PER_ELEMENT.rock,
            Element::Scissors => POINTS_PER_OUTCOME.lose + POINTS_PER_ELEMENT.paper,
        },
    }
}

fn get_element(value: &str) -> Element {
    match value {
        "A" => Element::Rock,
        "B" => Element::Paper,
        "C" => Element::Scissors,
        "X" => Element::Rock,
        "Y" => Element::Paper,
        "Z" => Element::Scissors,
        _ => Element::Rock,
    }
}

fn get_outcome(value: &str) -> Outcome {
    match value {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => Outcome::Lose,
    }
}
