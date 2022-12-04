use std::{
    collections::{HashMap, HashSet},
    env, fs, str,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut contents = fs::read_to_string(file_path).unwrap();
    // Last line is not needed
    contents.pop();

    let rucksack_contents: Vec<&str> = contents.split("\n").collect();
    let rucksack_contents_by_compartment: Vec<(&str, &str)> = rucksack_contents
        .iter()
        .map(|&line| {
            let split_index = line.len() / 2;
            line.split_at(split_index)
        })
        .collect();

    let common_elements: Vec<_> = rucksack_contents_by_compartment
        .iter()
        .map(|&compartments| get_common_element(vec![compartments.0, compartments.1]))
        .collect();

    let result = get_result(&common_elements);

    println!("Result Part One: {result}");

    let rucksack_content_chunks = rucksack_contents.chunks(3);
    let common_elements: Vec<char> = rucksack_content_chunks
        .map(|chunk| get_common_element(chunk.to_vec()))
        .collect();

    let result = get_result(&common_elements);

    println!("Result Part Two: {result}");
}

fn get_alphabet_scores() -> HashMap<char, usize> {
    let lowercase_alphabet: Vec<char> = ('a'..='z').collect();
    let uppercase_alphabet: Vec<char> = ('A'..='Z').collect();
    let mut alphabet_scores = HashMap::new();
    lowercase_alphabet
        .iter()
        .enumerate()
        .for_each(|(index, &letter)| {
            alphabet_scores.insert(letter, index + 1);
        });
    uppercase_alphabet
        .iter()
        .enumerate()
        .for_each(|(index, &letter)| {
            alphabet_scores.insert(letter, index + 27);
        });

    alphabet_scores
}

fn get_result(common_elements: &Vec<char>) -> usize {
    let alphabet_scores = get_alphabet_scores();
    common_elements
        .into_iter()
        .fold(0, |acc, letter| acc + alphabet_scores.get(&letter).unwrap())
}

fn get_common_element(item_groups: Vec<&str>) -> char {
    let item_group_sets = item_groups.iter().map(|&item_group| {
        let mut item_group_set = HashSet::new();
        item_group.chars().for_each(|letter| {
            item_group_set.insert(letter);
        });

        item_group_set
    });

    let mut item_counts = HashMap::new();

    item_group_sets.for_each(|item_group_set| {
        item_group_set.iter().for_each(|&item| {
            item_counts
                .entry(item)
                .and_modify(|value| *value += 1)
                .or_insert(1);
        });
    });

    let common_element = item_counts
        .into_iter()
        .find(|(_, count)| *count == item_groups.len());

    match common_element {
        Some((value, _)) => value,
        _ => 'a',
    }
}
