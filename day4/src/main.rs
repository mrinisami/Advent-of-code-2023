#![allow(dead_code)]
use std::collections::{HashSet, HashMap};
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

fn main() {
    let lines = read_lines("./input.txt");
    let mut res = 0;
    let mut line_nb = 0;
    let mut copies = HashMap::new();
    for line in lines{
        let card = card_parser(&line.unwrap());
        let score = card.card_score();
        add_copies(&mut copies, line_nb, i32::try_from(card.find_matches().len()).unwrap());
        res += &score;
        
        line_nb += 1;
    }

    let nb_cards = copies.values().sum::<i32>() + line_nb;
    println!("Card {res}; Copies {nb_cards}")
}

fn add_copies(copies: &mut HashMap<i32, i32>, line_nb: i32, nb_wins: i32) -> &mut HashMap<i32, i32>{
    let &copy_nb = match copies.get(&line_nb){
        Some(value) => value,
        None => &0
    };
    for i in 0..nb_wins{
        let index = line_nb + i + 1;
        copies.entry(index).and_modify(|value| *value += copy_nb + 1).or_insert(1 + copy_nb);
    }
    copies
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn card_parser(data: &String) -> Card{
    let line_data: Vec<&str> = data.split(":").collect::<Vec<&str>>()[1].split("|").collect();
    let winning_nbs = get_nbs(line_data[0]);
    let playing_nbs = get_nbs(line_data[1]);

    Card {winning: winning_nbs, playing: playing_nbs}
}

fn get_nbs(data: &str) -> HashSet<i32>{
    let nbs_vec = data.split(" ").filter(|char| char.parse::<f64>()
    .is_ok()).map(|char| char.parse::<i32>().unwrap()).collect::<HashSet<i32>>();

    nbs_vec
}

fn get_nbs_map(data: &str) -> HashMap<i32, i32>{
    let mut res = HashMap::new();
    data.split(" ").filter(|char| char.parse::<f64>()
    .is_ok()).map(|nb| nb.parse::<i32>().unwrap()).for_each(|nb| {res.insert(nb, 0);});

    res
}

struct Card{
    winning: HashSet<i32>,
    playing: HashSet<i32>
}

struct Game{
    copies: HashMap<i32, i32>
}

impl Card{
    fn find_matches(&self) -> Vec<i32>{
        self.playing.iter().filter(|nb| self.winning.contains(nb)).cloned().collect::<Vec<i32>>()
    }
    fn card_score(&self) -> i32{
        let matches = self.find_matches();
        if matches.is_empty() {return 0}
        else {return i32::from(2).pow(u32::try_from(matches.len() - 1).unwrap())}
    }
}