
use std::{io::{BufReader, BufRead}, fs::File};
use ndarray::{Array2, Array, Array1, s};


fn part1(lines: &Vec<String>) -> u32 {

    lines.iter().fold(0, |sum, line| {
        let winning_numbers: Vec<u32> = line[9..39].split_whitespace().map(|s| s.parse().unwrap()).collect();
        let given_numbers: Vec<u32> = line[41..].split_whitespace().map(|s| s.parse().unwrap()).collect();
        let hits = given_numbers.iter().filter(|given_number| winning_numbers.contains(&given_number)).count();

        if hits > 0 {
            return sum + (1 << (hits-1));
        }

        return sum;
    })
}

struct Card {
    winning_numbers: Vec<u32>,
    given_numbers: Vec<u32>
}

fn part2(lines: &Vec<String>) -> usize {

    let cards = extract_cards(lines);

    let scores: Array1<usize> = Array::from_vec( cards.iter().map(|card| {
        card.given_numbers.iter().filter(|given_number| card.winning_numbers.contains(&given_number)).count()
    }).collect::<Vec<usize>>());

    let permutated_scores: Array2<usize> = compute_permutated_scores(&scores);
    (permutated_scores + Array2::<usize>::eye(scores.len())).sum()
}

fn compute_permutated_scores(scores: &Array1<usize>) -> Array2<usize> {

    let mut res: Array2<usize> = Array2::zeros((scores.len(), scores.len()));

    for index in (0..scores.len()).rev() {
        let obtained_copies: Array1<usize> = obtain_copies(&index, scores);
        let mut new_res: Array1<usize> = obtained_copies.t().dot(&res);
        new_res.zip_mut_with(&obtained_copies.t(), |a, &b| *a += b);
        res.row_mut(index).assign(&new_res);
    }
    res
}

fn obtain_copies(index: &usize, scores: &Array1<usize>) -> Array1<usize> {
    let number_cards_won = scores[*index];
    let mut res: Array1<usize> = Array1::zeros(scores.dim());
    res.slice_mut(s![index + 1..upper_limit(index + 1 + number_cards_won, scores.len())]).fill(1);
    res
}

fn upper_limit(input: usize, limit: usize) -> usize {
    if input > limit {
        return limit;
    }
    return input;
}

fn extract_cards(lines: &Vec<String>) -> Vec<Card> {
    
    lines.iter().map(|line| {
        let winning_numbers: Vec<u32> = line[9..39].split_whitespace().map(|s| s.parse().unwrap()).collect();
        let given_numbers: Vec<u32> = line[41..].split_whitespace().map(|s| s.parse().unwrap()).collect();
        Card { winning_numbers, given_numbers }
    }).collect()
}

fn main() {
   
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

}